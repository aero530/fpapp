# Financial Planner — egui

App built with eframe 0.35, running natively on the desktop and as WebAssembly in a browser. Owns the UI layer only; all financial logic lives in the shared `accounts` crate.

Both builds come from this one crate — see [web/README.md](web/README.md) for building and hosting the browser version. The differences are confined to `platform/` and to the target-specific dependency sets in `Cargo.toml` (native gets rfd/env_logger/image and the wgpu renderer; wasm gets the glow/WebGL2 renderer and web-sys).

## Modules

| File | Responsibility |
|---|---|
| `main.rs` | Both entry points. Natively: loads the app icon from `egui/assets/icon-256.png` at compile time, configures the 1280×800 viewport, and calls `eframe::run_native`. On wasm: the module's start function, which finds the page's canvas, starts `eframe::WebRunner`, and clears (or replaces with an error) the page's loading message. |
| `platform/` | Everything that differs between desktop and browser: file dialogs, reading/writing the plan, and the calendar year. Both sides report results through one `FileIo` queue that `app.rs` drains each frame, so the rest of the UI has no `cfg` branches. `platform/file_io.js` is the browser half — File System Access API where available, `<input type="file">` plus a download everywhere else. |
| `app.rs` | Defines `FpApp` (all app state) and implements `eframe::App`. The `fn ui()` method is the frame entry point: applies finished file operations, checks the dirty flag, runs analysis, shows the delete-confirmation modal, delegates to `nav` and the current page. |
| `analyze.rs` | Bridge between the live `serde_json::Value` state and the typed `accounts` library. Deserializes the JSON blob by reference (no clone) directly into `UserData<SimAccount>`, runs the full simulation, and returns `(plot_data, yearly_totals)`. |
| `logger.rs` | Logger that tees warn-level records into a shared buffer the sidebar displays, and forwards everything on to env_logger (desktop) or eframe's `WebLogger` (browser console). |
| `nav.rs` | Left sidebar (230 px). File open / save / save-as buttons that hand off to `platform`, a scrollable account tree grouped by type with per-group collapse headers and "+ Add" buttons that insert a default JSON account object. |
| `dashboard.rs` | Four `egui_plot::Plot` charts drawn from `YearlyTotals`: Net / Income / Expense, Savings & HSA, Cost of Living, Healthcare & Tax. |
| `forms/` | One edit form per account type (`forms/<type>.rs`), dispatched by account `type` field from `forms/mod.rs`; shared rows for the four savings-style forms live in `forms/common.rs`. All edits write directly into `app.data["accounts"][uuid]` as JSON and set `dirty = true`. Each form also renders per-account plot data from `app.plot_data[uuid]`. |
| `settings_view.rs` | Global settings form: ages, year born, simulation start year, inflation rate, income and capital-gains tax rates, cost-of-living in retirement, and SSA breakpoints. |
| `widgets.rs` | Reusable field widgets that read from and write to a `&mut serde_json::Value`: `string_field`, `f64_field`, `u32_field`, `year_input`, `percent_input`, `combo_field`, `table_editor`, `plot_datasets`. |

## App state — `FpApp`

| Field | Type | Purpose |
|---|---|---|
| `data` | `serde_json::Value` | The entire open data file, held as live JSON. All UI writes go here directly; deserialization into typed structs happens only during analysis. |
| `selected` | `Page` | `Dashboard \| Settings \| Account(uuid)` — drives which panel the central area renders. |
| `dirty` | `bool` | Set to `true` on any edit. Analysis re-runs once the edit settles (250 ms debounce, and never mid-drag) rather than on every frame. |
| `yearly_totals` | `Option<YearlyTotals>` | Aggregate simulation output. `None` when no file is open or the last analysis errored. |
| `plot_data` | `HashMap<String, Vec<PlotDataSet>>` | Per-account plot series keyed by UUID, produced alongside `yearly_totals` after each analysis run. |
| `confirm_delete` | `Option<String>` | UUID of the account awaiting delete confirmation. A modal overlay is shown while this is `Some`; cancelled automatically if the user navigates away. |
| `error` | `Option<String>` | Parse, file, or simulation error string, shown in the sidebar footer. |
| `status` | `Option<String>` | Confirmation of the last file operation ("Saved plan.json"), shown when there is no error instead. Mainly for the browser build, where a save is otherwise silent. |
| `warnings` | `Vec<String>` | Engine warnings (worked-around misconfigurations) from the most recent analysis, shown in the sidebar footer. |
| `file_path` | `Option<PathBuf>` | Where the open plan lives. Always `None` in the browser, which never hands the page a path. |
| `file_name` | `Option<String>` | File name of the open plan; seeds the next save dialog. |
| `file_io` | `FileIo` | Queue of requested and completed file operations (see `platform/`). |

## Rendering loop

1. **File events** — `apply_file_events` folds in any open/save that finished since the last frame: parses an opened file into `data`, records the path/name of a save, or turns a failure into `error`.
2. **Dirty check** — if `dirty` is set, `data` is not null, the pointer is up, and 250 ms have passed since the flag was raised, calls `analyze::run_analysis(&data)`. On success, stores `(plot_data, yearly_totals)` and clears `error`. On failure, stores the error string and clears both result fields.
3. **Delete modal** — if `confirm_delete` is `Some`, renders an `egui::Window` overlay. Confirming removes the account UUID from the JSON map and clears any `incomeLink` references pointing to it.
4. **Left panel** — `nav::show_nav()` draws a 230 px `Panel::left` with the account tree and file controls.
5. **Central panel** — `egui::CentralPanel` dispatches to `dashboard`, `settings_view`, or `forms::show_account` based on `self.selected`.