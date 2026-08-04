#!/usr/bin/env bash
# Builds the browser version of the Financial Planner into egui/web/dist.
#
# Same job as build.ps1, for building on the Linux box that runs Apache.
# Everything it produces is static; copy dist/ into the document root.
#
#   ./egui/web/build.sh            # release build
#   ./egui/web/build.sh --debug    # fast to compile, big and slow to load
set -euo pipefail

# Without this, any aborted command leaves nothing behind but the process exit
# status -- and chasing an unexplained "exit code 127" through a CI log is exactly
# the time sink this avoids.
trap 'status=$?; echo "build.sh: aborted at line $LINENO with exit status $status" >&2' ERR

web_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$web_dir/../.." && pwd)"
out_dir="$web_dir/dist"

profile="release"
cargo_profile_args=("--release")
if [[ "${1:-}" == "--debug" ]]; then
    profile="debug"
    cargo_profile_args=()
fi

echo "Repository:  $repo_root"
echo "Output:      $out_dir"
echo "Profile:     $profile"
echo

# --- toolchain ---------------------------------------------------------------

if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
    echo 'Adding the wasm32-unknown-unknown target...'
    rustup target add wasm32-unknown-unknown
fi

# The generated glue and the compiled-in ABI must come from the same
# wasm-bindgen release; a mismatch fails the build with a schema error.
want_version="$(
    awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[version = "]/, ""); print; exit }' \
        "$repo_root/Cargo.lock"
)"
if [[ -z "$want_version" ]]; then
    echo 'Could not find the wasm-bindgen version in Cargo.lock' >&2
    exit 1
fi

have_version=""
if command -v wasm-bindgen >/dev/null 2>&1; then
    # A binary can be on PATH and still refuse to run: a copy restored from another
    # machine's build cache, a half-finished install, a missing shared library.
    #
    # This has to be guarded rather than written as a plain assignment. Under
    # `set -e` with `pipefail`, the failing pipeline inside a plain assignment takes
    # the whole script down with the tool's own exit status and prints nothing of its
    # own -- which surfaces as an unexplained "exit code 127" from a build that has
    # barely started. Not being able to read the version is not fatal; it just means
    # the matching CLI has to be installed below.
    if ! have_version="$(wasm-bindgen --version 2>/dev/null | awk '{print $NF}')"; then
        echo 'The wasm-bindgen on PATH will not run; installing it again.' >&2
        have_version=""
    fi
fi

if [[ "$have_version" != "$want_version" ]]; then
    if [[ -n "$have_version" ]]; then
        echo "wasm-bindgen CLI is $have_version but the crate is $want_version; installing the matching CLI..."
    else
        echo "Installing the wasm-bindgen CLI ($want_version)..."
    fi
    # cargo-binstall grabs a prebuilt binary in seconds; cargo install builds it
    # from source, which takes a few minutes but needs nothing extra.
    if command -v cargo-binstall >/dev/null 2>&1; then
        cargo binstall --no-confirm --force "wasm-bindgen-cli@$want_version"
    else
        cargo install --locked --force wasm-bindgen-cli --version "$want_version"
    fi
fi

# Everything below depends on wasm-bindgen working, so say so here rather than
# failing further down with only an exit status to go on.
if ! wasm-bindgen --version >/dev/null 2>&1; then
    echo 'wasm-bindgen still will not run after being installed.' >&2
    echo "Try removing it and rerunning: rm -f \"\$(command -v wasm-bindgen)\"" >&2
    exit 1
fi

# --- compile -----------------------------------------------------------------

echo
echo 'Compiling to WebAssembly...'
(
    cd "$repo_root"
    cargo build --target wasm32-unknown-unknown --package fpapp-egui --bin fpapp-egui \
        "${cargo_profile_args[@]}"
)

wasm_in="$repo_root/target/wasm32-unknown-unknown/$profile/fpapp-egui.wasm"
[[ -f "$wasm_in" ]] || { echo "Expected $wasm_in to exist" >&2; exit 1; }

# --- assemble ----------------------------------------------------------------

# Start clean so stale snippets from an earlier build cannot linger
rm -rf "$out_dir"
mkdir -p "$out_dir"

echo 'Generating JavaScript bindings...'
wasm-bindgen --target web --no-typescript --out-name fpapp --out-dir "$out_dir" "$wasm_in"

wasm_out="$out_dir/fpapp_bg.wasm"

# Check that a module still starts: instantiate it and grow the externref table
# the way the generated glue does on startup.
#
# This exists because wasm-opt can produce a module that is valid, smaller, and
# broken.  Binaryen before ~v131 mis-reindexes exports when a module has more
# than one table -- this one has two, a funcref table and an externref table --
# and points "__wbindgen_externrefs" at the funcref table, which is fixed-size.
# The glue's first act is to grow that table by 4, so the app dies on load with
# "failed to grow table by 4" while every other check passes: the file is there,
# it is the right size, and it is a valid wasm module.
#
# Needs node, which is not otherwise required to build; without it the check is
# skipped rather than made mandatory.
check_module_starts() {
    local module="$1" output
    if output="$(_instantiate_module "$module" 2>&1)"; then
        return 0
    fi
    # node prints a full stack trace; the first Error line is the part worth reading.
    printf '%s\n' "$output" | grep -m1 -E '[A-Za-z]*Error' || printf '%s\n' "$output" | head -1
    return 1
}

_instantiate_module() {
    node --input-type=module -e '
        import { readFileSync } from "node:fs";
        const mod = new WebAssembly.Module(readFileSync(process.argv[1]));
        const imports = {};
        for (const i of WebAssembly.Module.imports(mod)) {
            imports[i.module] ??= {};
            imports[i.module][i.name] =
                i.kind === "function" ? () => {} :
                i.kind === "memory"   ? new WebAssembly.Memory({ initial: 17 }) :
                i.kind === "table"    ? new WebAssembly.Table({ initial: 1, element: "anyfunc" }) :
                                        new WebAssembly.Global({ value: "i32", mutable: true }, 0);
        }
        const table = new WebAssembly.Instance(mod, imports).exports.__wbindgen_externrefs;
        if (!table) throw new Error("no __wbindgen_externrefs export");
        table.grow(4);
    ' "$1"
}

# wasm-opt is optional (it ships with binaryen).  It typically takes another
# 15-25% off the module; if the installed version chokes on a newer wasm
# feature, the unoptimised module is perfectly usable.
if [[ "$profile" == "release" ]]; then
    if command -v wasm-opt >/dev/null 2>&1; then
        echo "Optimising with wasm-opt ($(wasm-opt --version))..."
        if wasm-opt -Oz --output "$wasm_out.opt" "$wasm_out"; then
            # Keep the optimised module only if it still starts.  Older binaryen
            # breaks it silently (see check_module_starts above), and shipping the
            # unoptimised module is strictly better than shipping a blank page.
            if ! command -v node >/dev/null 2>&1; then
                echo 'node not found - cannot verify the optimised module, keeping it unchecked' >&2
                mv "$wasm_out.opt" "$wasm_out"
            elif error="$(check_module_starts "$wasm_out.opt")"; then
                mv "$wasm_out.opt" "$wasm_out"
            else
                echo "wasm-opt produced a module that will not start; keeping the unoptimised one." >&2
                echo "  $error" >&2
                echo "  This is a known bug in binaryen before v131. Upgrade it to get the smaller module." >&2
                rm -f "$wasm_out.opt"
            fi
        else
            echo 'wasm-opt failed; keeping the unoptimised module' >&2
            rm -f "$wasm_out.opt"
        fi
    else
        echo 'wasm-opt not found (optional) — skipping size optimisation'
    fi

    # Whatever is about to be deployed, optimised or not, has to start.
    if command -v node >/dev/null 2>&1; then
        if error="$(check_module_starts "$wasm_out")"; then
            echo 'Module starts cleanly.'
        else
            echo "The built module will not start:" >&2
            echo "  $error" >&2
            exit 1
        fi
    fi
fi

cp "$web_dir/index.html" "$out_dir/"
cp "$web_dir/htaccess" "$out_dir/.htaccess"
cp "$repo_root/egui/assets/icon-256.png" "$out_dir/"

# --- report ------------------------------------------------------------------

echo
echo "Built $out_dir"
du -h -a "$out_dir" | sort -hr | sed 's/^/  /'
echo
echo 'Copy the contents of that directory into your Apache document root.'
echo "To try it locally first:  python3 -m http.server 8080 --directory $out_dir"
