# Financial Planner — egui

Native desktop app built with eframe 0.35. Owns the UI layer only; all financial logic lives in the shared `accounts` crate.

## Modules

| File | Responsibility |
|---|---|
| `main.rs` | Loads the app icon from `tauri/resources/icons/256x256.png` at compile time, configures the 1280×800 viewport, and calls `eframe::run_native`. |
| `app.rs` | Defines `FpApp` (all app state) and implements `eframe::App`. The `fn ui()` method is the frame entry point: checks the dirty flag, runs analysis, shows the delete-confirmation modal, delegates to `nav` and the current page. |
| `analyze.rs` | Bridge between the live `serde_json::Value` state and the typed `accounts` library. Deserializes the JSON blob into `UserData<AccountWrapper>`, converts to `UserData<Box<dyn Account>>`, runs the full simulation, and returns `(plot_data, yearly_totals)`. |
| `nav.rs` | Left sidebar (230 px). File open / save / save-as buttons, a scrollable account tree grouped by type with per-group collapse headers and "+ Add" buttons that insert a default JSON account object. |
| `dashboard.rs` | Four `egui_plot::Plot` charts drawn from `YearlyTotals`: Net / Income / Expense, Savings balance, Cost of Living, Healthcare & Tax Burden. |
| `forms.rs` | One edit form per account type, dispatched by account `type` field. All edits write directly into `app.data["accounts"][uuid]` as JSON and set `dirty = true`. Each form also renders per-account plot data from `app.plot_data[uuid]`. |
| `settings_view.rs` | Global settings form: ages, year born, simulation start year, inflation rate, income and capital-gains tax rates, cost-of-living in retirement, and SSA breakpoints. |
| `widgets.rs` | Reusable field widgets that read from and write to a `&mut serde_json::Value`: `string_field`, `f64_field`, `u32_field`, `year_input`, `percent_input`, `combo_field`, `table_editor`, `plot_datasets`. |

## App state — `FpApp`

| Field | Type | Purpose |
|---|---|---|
| `data` | `serde_json::Value` | The entire open data file, held as live JSON. All UI writes go here directly; deserialization into typed structs happens only during analysis. |
| `selected` | `Page` | `Dashboard \| Settings \| Account(uuid)` — drives which panel the central area renders. |
| `dirty` | `bool` | Set to `true` on any edit. The next call to `fn ui()` re-runs analysis and clears the flag. |
| `yearly_totals` | `Option<YearlyTotals>` | Aggregate simulation output. `None` when no file is open or the last analysis errored. |
| `plot_data` | `HashMap<String, Vec<PlotDataSet>>` | Per-account plot series keyed by UUID, produced alongside `yearly_totals` after each analysis run. |
| `confirm_delete` | `Option<String>` | UUID of the account awaiting delete confirmation. A modal overlay is shown while this is `Some`; cancelled automatically if the user navigates away. |
| `error` | `Option<String>` | Parse or simulation error string, shown in the sidebar footer. |

## Rendering loop

1. **Dirty check** — if `dirty` is set and `data` is not null, calls `analyze::run_analysis(&data)`. On success, stores `(plot_data, yearly_totals)` and clears `error`. On failure, stores the error string and clears both result fields.
2. **Delete modal** — if `confirm_delete` is `Some`, renders an `egui::Window` overlay. Confirming removes the account UUID from the JSON map and clears any `incomeLink` or `hsaLink` references pointing to it.
3. **Left panel** — `nav::show_nav()` draws a 230 px `Panel::left` with the account tree and file controls.
4. **Central panel** — `egui::CentralPanel` dispatches to `dashboard`, `settings_view`, or `forms::show_account` based on `self.selected`.