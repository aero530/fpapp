# Financial Planner in the browser

The same egui app, compiled to WebAssembly and served as a handful of static
files. There is no backend: the server hands the browser an HTML page and a wasm
module, and everything after that — editing, simulating, reading and writing
plan files — happens on the client.

Live at **<https://aero530.github.io/fpapp/>**, deployed from `main` by
[`.github/workflows/pages.yml`](../../.github/workflows/pages.yml). The same
`dist/` also drops straight into an Apache document root; both are covered below.

| File | What it is |
|---|---|
| `index.html` | The page. Creates the canvas and starts the wasm module. |
| `htaccess` | Apache directives, copied to `dist/.htaccess` by the build. |
| `build.ps1` / `build.sh` | Compile + assemble `dist/`. Windows / Linux. |
| `dist/` | Build output — the directory you copy to the server. Not in git. |

## Build

Needs the Rust toolchain, and nothing else installed up front — the script adds
the `wasm32-unknown-unknown` target and the matching `wasm-bindgen` CLI if they
are missing.

```powershell
.\egui\web\build.ps1              # Windows
```

```bash
./egui/web/build.sh               # Linux / macOS
```

That produces `egui/web/dist`:

```
index.html
fpapp.js                                  wasm-bindgen glue
fpapp_bg.wasm                             the app (~6.5 MB, ~2 MB gzipped)
snippets/fpapp-egui-<hash>/…/file_io.js   the browser file-open/save shim
icon-256.png
.htaccess
```

Try it locally before deploying:

```bash
python -m http.server 8080 --directory egui/web/dist
# http://localhost:8080/
```

Two optional extras, both picked up automatically when present:

- **`wasm-opt`** (from [binaryen](https://github.com/WebAssembly/binaryen)) —
  trims another ~22% off the module. **Use v131 or newer.** Older binaryen
  mis-reindexes exports in a module with more than one table — this one has two,
  a funcref table and an externref table — and repoints `__wbindgen_externrefs`
  at the fixed-size funcref table. The result is a valid, smaller module that
  dies on load with `failed to grow table by 4`. Distribution packages are often
  old enough to do this (Ubuntu's is), so prefer a
  [release build](https://github.com/WebAssembly/binaryen/releases).
  The build script instantiates the optimised module and grows that table before
  accepting it, so a bad `wasm-opt` costs you the size saving and a warning
  rather than a broken deploy. That check needs `node`; without it the optimised
  module is kept unverified.
- **`cargo-binstall`** — fetches the `wasm-bindgen` CLI as a prebuilt binary
  instead of compiling it, when the version needs to change.

`build.ps1 -DebugBuild` / `build.sh --debug` compile far faster but produce a
module several times larger; use them only for debugging.

## Deploy to GitHub Pages

Pushing to `main` builds `dist/` and publishes it; there is nothing to do by
hand. The workflow can also be run from the Actions tab against the current
`main`.

Two things differ from the Apache deployment:

- **`.htaccess` is dropped from the artifact.** Pages serves through its own CDN,
  which already sends `application/wasm` and compresses on the fly, so the file
  would only be dead weight.
- **The site lives in a subdirectory** (`/fpapp/`). Nothing has to change for
  that: every reference the page makes is relative, including the
  `new URL('fpapp_bg.wasm', import.meta.url)` in the generated glue and the
  `./snippets/…` import.

Because Pages is https, the demo gets the better half of the save behaviour
described below — in Chrome or Edge, **Save** overwrites the plan file in place
exactly as the desktop build does.

The first run needs the repository's Pages source set to "GitHub Actions"; the
workflow asks for that itself (`configure-pages` with `enablement: true`), so it
normally sorts itself out. If that call is refused, set it once under
Settings → Pages → Source.

## Deploy to Apache

Copy the contents of `dist/` into a document root or a subdirectory of one:

```bash
rsync -a --delete egui/web/dist/ user@server:/var/www/html/planner/
```

Nothing else is required — no modules, no CGI, no rewrite rules. The app runs
happily from a subdirectory (all its references are relative) and over plain
`http://` on an intranet host.

The included `.htaccess` sets the two things worth setting:

- **`AddType application/wasm .wasm`** — Apache older than 2.4.53 doesn't know
  the type, and browsers refuse to stream a wasm module that arrives as
  anything else. Without it the app still loads (the glue falls back to a
  non-streaming path) but more slowly.
- **`DEFLATE`** for wasm/JS/HTML — the module is mostly compressible, so this is
  the difference between a ~6.5 MB and a ~2 MB download.

If the server runs `AllowOverride None`, `.htaccess` is ignored; paste the same
directives into the vhost instead:

```apache
<Directory /var/www/html/planner>
    AddType application/wasm .wasm
    AddOutputFilterByType DEFLATE application/wasm text/javascript text/html text/css
    Options -Indexes
</Directory>
```

For a busy server, precompressing is cheaper than compressing per request:
`gzip -k9 dist/fpapp_bg.wasm` plus `mod_rewrite` + `Content-Encoding: gzip`.
For a handful of users, `mod_deflate` is fine.

## Opening and saving plans

Plans are read from and written to the machine the browser is running on. The
file never leaves it: nothing is uploaded, and the app makes no network requests
at all after the page has loaded. Edits live in the tab's memory until **Save**
or **Save As...** is clicked — closing or reloading the tab discards them, the
same way quitting the desktop app without saving does.

How the buttons behave depends on what the browser supports:

| Browser | Open… | Save | Save As… |
|---|---|---|---|
| Chrome / Edge (over `https://` or `localhost`) | File picker | Writes back to the file the plan was opened from | File picker |
| Firefox, Safari, or any browser over plain `http://` | File picker | Downloads the file | Downloads the file |

The first row uses the [File System Access
API](https://developer.mozilla.org/en-US/docs/Web/API/File_System_API), which
browsers only expose in a
[secure context](https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts).
That is the only reason to prefer https here: with it, **Save** overwrites the
plan in place exactly like the desktop build; without it, every save is a
download into the browser's download directory, and re-opening means picking the
file again.

The first save of a session always asks where to put the file, since the browser
has to be told which file the page may write to.

## Requirements and limitations

- **WebAssembly and WebGL2.** Any browser from the last few years has both.
  The build deliberately uses WebGL2 rather than WebGPU, because WebGPU is
  unavailable over plain `http://` — a self-hosted intranet page would render a
  blank canvas.
- **First load pulls ~2 MB** (gzipped). Cached afterwards, revalidated on each
  visit so a redeploy is picked up.
- **No persistence between visits.** The app stores nothing in the browser: no
  cookies, no local storage. Your plan file is the only state.
- **One tab, one plan.** There is no shared or server-side state, so several
  people can use the same URL at once without interfering with each other.

## Troubleshooting

| Symptom | Cause |
|---|---|
| Page says "Could not start the app" | WebGL2 is disabled or unavailable (check `chrome://gpu`, or a hardened browser profile). |
| Page stays on "Loading Financial Planner…" | The wasm or JS file 404'd — check that `snippets/` came across, and look at the browser console. |
| Console warns about the wasm MIME type | `AddType application/wasm .wasm` isn't in effect; the app still works, just loads slower. |
| Build fails with a wasm-bindgen schema mismatch | The CLI and the crate versions diverged; rerun the build script, which reinstalls the matching CLI. |
| "Save" downloads a copy instead of overwriting | Expected outside Chrome/Edge on https — see the table above. |
