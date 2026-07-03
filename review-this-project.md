# Audit Report: fpapp (financial planning app)

## Context

Full audit of the workspace on branch `egui-ui`: the `accounts` simulation engine, the new `egui` UI (v5.0.0), and the deprecated Tauri app. Requested scope: bugs, logic flaws, performance bottlenecks, architectural simplifications, and modernizations. Every finding below was verified against the source (file:line given). The deliverable is this report; a prioritized remediation plan is at the end for when/if fixes are wanted.

---

## A. High-severity correctness bugs (simulation produces wrong numbers)

### A1. Negative or zero `net` is silently erased when rolling years forward
`Table::most_recent_populated_year` ([table.rs:52-61](accounts/src/simulation/table.rs#L52-L61)) filters to values `> f64::EPSILON`. `YearlyTotals::pull_value_forward` ([totals.rs:91-99](accounts/src/simulation/totals.rs#L91-L99)) uses it to carry `net`, `saving`, and `hsa` into each new year.
**Failure:** if `net` goes negative in year N (expenses exceed income — exactly the scenario a planning tool must model), year N+1 pulls forward the most recent *positive* year instead, resurrecting stale wealth and deleting the debt. A `net` of exactly 0 behaves the same. Every year after the first negative year is wrong. Account-level tables use a different rule (`most_recent_value`, keeps zeros/negatives), so the two roll-forward semantics disagree with each other.

### A2. Historical table entries mid-simulation reset `net` to zero for that year
The init pass seeds years that appear in any account's historical `table` with `add_year(year, false)` — no pull-forward ([runner.rs:53-57](accounts/src/runner.rs#L53-L57)). The main loop then skips `add_year` for years that already exist ([runner.rs:61-63](accounts/src/runner.rs#L61-L63)), so those years **never get `net`/`saving`/`hsa` pulled forward**.
**Failure:** "Track historic account balances" is an advertised feature; any table entry in a year after `yearStart` zeroes accumulated `net` for that year, and `saving` for that year contains only the seeded account's balance (all other accounts' balances vanish from the total). Everything downstream (tax, ColFracOfSavings withdrawals, the dashboard net line) is corrupted from that year on.

### A3. Account-table roll-forward copies values from the *future*
`SingleTable/LoanTables/SavingsTables::add_year` pull `self.value.most_recent_value()` — the **last entry in the BTreeMap by year**, with no `<= current year` check ([table_groups.rs:64-68, 210-213, 385-388](accounts/src/simulation/table_groups.rs#L60-L71)).
**Failure:** an account with table `{2020: 100k, 2025: 120k}` simulating 2021 copies **2025's** balance into 2021, then compounds earnings on it for 2021–2024. Contrast `Table::pull_value_forward` (used for totals) which at least checks `recent_year < year`.

### A4. Mortgage payoff never completes; balance can oscillate around escrow
In the payoff year, `get_payment` caps the payment at the full outstanding balance (payment derive, [account_payment_derive/src/lib.rs:20-33](accounts/src/account_payment_derive/src/lib.rs#L18-L33)), but only `payment − insurance − escrow` is applied to principal ([mortgage.rs:213-219](accounts/src/mortgage.rs#L213-L219)).
**Failure:** a residual ≈ `insurance + escrow` survives the payoff year (the `< 0.0001` zero-out never triggers). In following years the capped payment is smaller than escrow, so `remaining_payment` goes **negative and the balance grows** — the mortgage reaches a fixed point near the escrow value and the user "pays" a phantom expense every year until `endOut`.

### A5. HSA contributions are free money
`Hsa::simulate` returns `expense: 0` and `income_taxable: 0` ([hsa.rs:246-255](accounts/src/hsa.rs#L246-L255)).
**Failure:** employee HSA contributions increase the HSA balance but are never deducted from `net` (compare Savings/Retirement, where contributions flow through `expense`) and never reduce taxable income despite HSAs being pretax. The account's `tax_status` field ([hsa.rs:44](accounts/src/hsa.rs#L44)) is carried, shown in the UI, and completely ignored by `simulate`. Money is conjured from nothing every contribution year.

### A6. SSA taxability is computed before retirement income exists
`Ssa::simulate` reads `totals.get_income(year)` as "other income" ([ssa.rs:131-132](accounts/src/ssa.rs#L131-L132)), but SSA runs second in the fixed order (Income → Ssa → … → Retirement → Savings). Retirement/savings withdrawals — which count as `income` and dominate retirement years — haven't run yet.
**Failure:** during retirement (when SSA taxability matters), combined income is understated, usually landing below `breakpoints.low` → 0% of benefits taxed. Additionally, `settings.ssa.taxable_income_percentage.low` is configured, stored, editable in the UI — and **never referenced**; the code interpolates from 0 straight to `.high` ([ssa.rs:133-144](accounts/src/ssa.rs#L133-L144)), so the real two-tier 50%/85% structure isn't modeled.

### A7. College balances half-in, half-out of the savings pool
`College::init` seeds `saving: *value` for every historical year ([college.rs:115-128](accounts/src/college.rs#L115-L128)), but `simulate` deliberately returns `saving: 0` ("college funds are not part of the general savings pool", [college.rs:240](accounts/src/college.rs#L240)).
**Failure:** the initial 529 balance enters `totals.saving` once and **rolls forward forever** as a constant offset that never earns, never shrinks with 529 withdrawals — permanently inflating the `ColFracOfSavings` denominator used by every savings/retirement account's withdrawal calculation.

---

## B. Medium-severity logic flaws

| # | Finding | Location |
|---|---|---|
| B1 | **Nondeterministic results**: `account_order` iterates a `HashMap` within each type. Two HSAs splitting one healthcare expense, multiple SSA accounts (each sees prior SSAs' income), or `PercentOfIncome` without a link resolve in arbitrary order — same file can produce different numbers per run. | [runner.rs:8-15](accounts/src/runner.rs#L8-L15) |
| B2 | **`hsaLink` is decorative.** The Expense form offers an HSA link, the field is stored, but `Expense::link_id()` returns `None` and nothing reads `hsa_link` — healthcare expenses go to a shared pool paid by all HSAs regardless. | [expense.rs:44, 79-81](accounts/src/expense.rs#L79-L81) |
| B3 | **`Expense::init` throws away the historical table** (`SingleTable::default()`), unlike Income which seeds from it. Historical expense data is never plotted or used. | [expense.rs:93](accounts/src/expense.rs#L93) |
| B4 | **`taxCapitalGains` is a dead setting** — editable in the UI, stored, never used. `ContributeTaxedEarningsTaxed` taxes earnings at the full income rate instead (acknowledged by a `todo!` comment). | [retirement.rs:318-327](accounts/src/retirement.rs#L318-L327), [settings_view.rs:51](egui/src/settings_view.rs#L51) |
| B5 | **Negative taxable income produces a tax refund at the flat rate** with no floor: `tax_burden = income_taxable × rate` even when `income_taxable < 0` (pretax contributions with no income). | [totals.rs:132-139](accounts/src/simulation/totals.rs#L132-L139) |
| B6 | **Silent misconfiguration masking**: `IncomeLink` year resolution with a missing link silently yields year 0 (`unwrap_or_default()` chain); a non-numeric percent string silently becomes 0%; `YearInput::Calculate` wraps negative results to huge u32 years (`as i32 + delta) as u32`). | [year.rs:91-96](accounts/src/inputs/year.rs#L91-L96), [percent.rs:29](accounts/src/inputs/percent.rs#L25-L32), [year.rs:50](accounts/src/inputs/year.rs#L48-L51) |
| B7 | **Div-by-zero/NaN inputs are reachable from the UI**: `compound_time = 0` (mortgage interest), `home_value = 0` (LTV), `tax_income ≥ 100` (ColFracOfSavings gross-up `/(1 − tax/100)`). NaN then propagates through every table silently. | [mortgage.rs:188-201](accounts/src/mortgage.rs#L188-L201), [account_savings_derive/src/lib.rs:86](accounts/src/account_savings_derive/src/lib.rs) |
| B8 | **Exact float equality** `link_income == 0_f64` gates employer matching. | [retirement.rs:255](accounts/src/retirement.rs#L255) |
| B9 | Dead `col_scale` binding computed and never used inside `ColFracOfSavings` (the scaling correctly lives in expense.rs — the leftover invites a future double-scaling). | account_savings_derive `get_withdrawal` |
| B10 | **UI year fields commit partial input instantly.** `year_input` regenerates its text from JSON every frame and commits any parseable prefix: typing "2025" commits year "2", then "20"… each triggering a full re-simulation with absurd dates (e.g. `raise^(2024−2)` → `inf`). Unparseable keystrokes (`yearRetire+`) are discarded next frame, so expressions are nearly untypeable. Parser also lacks `yearEnd`/`incomeLink`, which the engine accepts. | [widgets.rs:53-67, 92-113](egui/src/widgets.rs#L53-L67) |

---

## C. Crash vectors (panics reachable from user data)

The whole engine leans on `unwrap()`; most are safe by construction, but these are reachable from a hand-edited JSON file (the format is advertised as human-editable) and take down the entire egui app since `analyze::run_analysis` only catches serde errors, not panics:

- **`Table<String> → Table<u32>`: `k.parse::<u32>().unwrap()`** ([table.rs:159](accounts/src/simulation/table.rs#L153-L163)) — one malformed year key (`"20x0"`) in any account table panics inside `user_data.into()`.
- **College rejects 3 of 4 tax statuses with `Err`** ([college.rs:245-256](accounts/src/college.rs#L245-L256)) — doesn't panic, but any 529 with a pretax status aborts the entire analysis; only the UI's single-option combo prevents it.
- `plot::domain`'s `.min()/.max().unwrap()` and `Table::min_key/max_key` panic on empty tables ([table.rs:99-119](accounts/src/simulation/table.rs#L99-L119)).
- `dates.year_in/year_out.unwrap()` throughout every `simulate` — safe only because `init` always runs first; nothing enforces that ordering in the type system.

---

## D. Performance

### D1. The one that matters: full re-simulation every frame while dragging
`dirty` is set by any widget `.changed()` — which egui reports **every frame during a slider drag** — and processed at the top of the next frame ([app.rs:59-64](egui/src/app.rs#L59-L64)). Each run: deep-clone of the entire JSON blob + full re-parse + re-boxing of every account + complete multi-decade simulation ([analyze.rs:9](egui/src/analyze.rs#L9)). Dragging a DragValue re-runs all of that at frame rate. Fix is small: debounce (run only when the drag releases / value stable for ~150 ms) or run analysis on `drag_stopped()`-style semantics.

### D2. `Table` helpers are allocation-heavy
- `years()` collects then **sorts keys that a BTreeMap already stores sorted** ([table.rs:133-137](accounts/src/simulation/table.rs#L133-L137)); the dashboard calls it 7×/frame.
- `min/max_value`, `min/max_key`, `most_recent_populated_year` each collect a `Vec` just to fold/min/max ([table.rs:52-119](accounts/src/simulation/table.rs#L52-L119)); `BTreeMap` gives `last_key_value()`/`iter().rev().find()` for free. `pull_value_forward` is called per year → O(years²) scans.
- `IntoIterator` collects the whole map into a `Vec` first ([table.rs:140-151](accounts/src/simulation/table.rs#L140-L151)).

### D3. Per-frame UI allocations (minor, easy wins)
Sidebar rebuilds all (uuid, type, name) strings ×9 type filters every frame ([nav.rs:55-76](egui/src/nav.rs#L55-L76)); `show_account` clones the whole account `Value` per frame ([forms/mod.rs:60](egui/src/forms/mod.rs#L60)); settings clones the settings object per frame ([settings_view.rs:15](egui/src/settings_view.rs#L15)); `table_editor` rebuilds and re-sorts its rows per frame ([widgets.rs:223-234](egui/src/widgets.rs#L223-L234)); `runner.rs:65` clones the `link_id` String per account-year.

---

## E. Architectural simplifications

1. **Delete the three proc-macro crates.** `account_savings_derive`, `account_payment_derive`, `account_expense_derive` exist only to paste identical method bodies into structs that share field names. A plain trait with default methods over a small accessor trait (`fn contribution_value(&self) -> f64; fn analysis(&self) -> &SavingsTables; …`) achieves the same with zero codegen, drops `syn`/`quote` (syn 1.0, two majors old), and makes the withdrawal/contribution math testable and steppable. Bonus: kills the duplication with the commented-out `PaymentOptions::value`/`ExpenseOptions::value` corpses in [inputs/payment.rs:19-72](accounts/src/inputs/payment.rs#L19-L72) and [inputs/expense.rs:19-33](accounts/src/inputs/expense.rs#L19-L33).
2. **Dead code to remove**: `AccountType::order()` (byte-for-byte duplicate of `AccountWrapper::order()`, zero call sites, [lib.rs:177-190](accounts/src/lib.rs#L177-L190)); `Account::get_inputs` (returns `"Hello"` in 8 of 9 impls) + `IncomeInput`; `MortgagePlot` trait; `UserData::write_tables`; `Table::most_recent_populated_value`; `YearlyTotals.income_during_retirement`; Tauri placeholder commands `my_custom_command`/`do_a_thing`/`RequestBody`; stale `accounts/bindings/AccountWrapper.ts` (no longer generated — the Rust type lost its `#[ts(export)]`).
3. **Decouple from the deprecated Tauri tree.** The workspace still builds `tauri/src-tauri` (the entire Tauri/webview dependency tree) on every `cargo build --workspace`, and [egui/src/main.rs:26](egui/src/main.rs#L26) `include_bytes!`s its icon from `tauri/resources/`. Move the icon into `egui/`, drop the member (or move it to an excluded dir). Once Tauri is gone, `ts-rs` and every `#[derive(TS)]`/`#[ts(export)]` in `accounts` can go too — the egui UI never touches the TypeScript bindings.
4. **Consolidate the four near-identical forms** (`retirement.rs`/`hsa.rs`/`college.rs`/`savings.rs` are ~80% copy-paste including verbatim tooltip prose) into a shared builder; merge `income_link_combo`/`hsa_link_combo` ([forms/mod.rs:124-202](egui/src/forms/mod.rs#L124-L202)) into one parameterized combo.
5. **Deterministic ordering**: sort `account_order` by (type, name, uuid) when building it — one line, fixes B1.
6. **Cargo hygiene**: `accounts` declares unused deps `flexi_logger` and `toml`; `float-cmp` is listed in both `[dependencies]` and `[dev-dependencies]` (should be dev-only). No `[workspace.dependencies]` — serde/serde_json/log/image versions are pinned independently per crate.

---

## F. Modernization

- **Error handling**: everything is `Box<dyn Error>` + `format!` strings. A `thiserror` enum in `accounts` plus `TryFrom` (not `From`+panic) for `Table<String> → Table<u32>` turns C-class crashes into the UI's existing error banner.
- **Edition**: all crates are on edition 2021; edition 2024 is current. No `rust-toolchain.toml` at the root (only a legacy `stable` file inside the tauri tree).
- **No CI**: no `.github/workflows`, no fmt/clippy/test gate. Given how subtle the numeric code is, a minimal workflow (`cargo fmt --check`, `clippy -D warnings`, `cargo test`) would pay for itself immediately.
- **egui housekeeping**: `egui_plot` 0.36 rides ahead of egui/eframe 0.35 (works, but pin-bump together); two `rfd` versions in the tree (0.15 transitive + 0.17 direct).

## G. Test coverage

Existing tests cover: savings contribution/withdrawal math (8 good tests), one expense test, inputs (year/percent/settings), and one sample-plan deserialization test. **Zero coverage** for: `runner::run` end-to-end, `YearlyTotals` settlement (tax/expense/healthcare/roll-forward — where A1/A2 live), mortgage/loan amortization (A4), SSA taxability (A6), HSA (A5), retirement employer matching and tax-status branches, and the entire egui crate. The `inputs/payment.rs` test module is commented out wholesale. The README's own TODO ("verify calculations for all account types") agrees.

---

## Prioritized remediation plan (if/when fixes are requested)

**Phase 1 — correctness of results (accounts crate)**
1. A1: make totals roll-forward use last entry ≤ year regardless of sign (align with account tables); delete the positive-only filter.
2. A2: in `runner.rs`, always `add_year(year, true)` semantics — pull forward first, then let seeded impacts *override* (insert) rather than skip.
3. A3: give `most_recent_value` a `<= year` bound (share one roll-forward implementation between `Table` and the table groups).
4. A4: cap mortgage payment at `balance + insurance + escrow` (or apply the cap after deducting escrow/insurance).
5. A5: route HSA employee contributions through `expense` and, per `tax_status`, `income_taxable`.
6. A6/A7: document or fix SSA ordering (two-pass year or move SSA last with income snapshot); use `taxable_income_percentage.low`; stop seeding college balances into `saving`.
7. B1: sort `account_order`.
Each fix lands with a regression test in the currently untested area (Section G).

**Phase 2 — robustness**: `TryFrom` for table keys, thiserror error enum, input validation (compound_time/home_value/tax rates), fix year_input commit-on-parse-prefix (commit on lost focus / Enter).

**Phase 3 — performance**: debounce analysis (D1), `Table` helper cleanup (D2).

**Phase 4 — structure**: drop tauri workspace member + move icon, delete dead code, replace derive crates with default trait methods, consolidate forms, add CI.

## Verification
- `cargo test -p accounts` (new regression tests for A1–A7 must fail before/pass after).
- Run the egui app (`cargo run -p fpapp-egui`) with `examples/sample_plan.json`: confirm net line goes negative and stays negative when expenses exceed income; add a historical table entry mid-simulation and confirm net continuity; confirm mortgage balance reaches exactly 0 and payments stop.
- Drag a DragValue and confirm the sim runs once on release, not per frame (log line in `run_analysis`).
