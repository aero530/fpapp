#!/usr/bin/env bash
# Builds the browser version of the Financial Planner into egui/web/dist.
#
# Same job as build.ps1, for building on the Linux box that runs Apache.
# Everything it produces is static; copy dist/ into the document root.
#
#   ./egui/web/build.sh            # release build
#   ./egui/web/build.sh --debug    # fast to compile, big and slow to load
set -euo pipefail

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
    have_version="$(wasm-bindgen --version | awk '{print $NF}')"
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
        cargo binstall --no-confirm "wasm-bindgen-cli@$want_version"
    else
        cargo install --locked wasm-bindgen-cli --version "$want_version"
    fi
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

# wasm-opt is optional (it ships with binaryen).  It typically takes another
# 15-25% off the module; if the installed version chokes on a newer wasm
# feature, the unoptimised module is perfectly usable.
if [[ "$profile" == "release" ]]; then
    if command -v wasm-opt >/dev/null 2>&1; then
        echo 'Optimising with wasm-opt...'
        if wasm-opt -Oz --output "$wasm_out.opt" "$wasm_out"; then
            mv "$wasm_out.opt" "$wasm_out"
        else
            echo 'wasm-opt failed; keeping the unoptimised module' >&2
            rm -f "$wasm_out.opt"
        fi
    else
        echo 'wasm-opt not found (optional) — skipping size optimisation'
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
