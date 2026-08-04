# financial planning app

A financial planning & simulation application.

### [Try it in your browser →](https://aero530.github.io/fpapp/)

The full app, nothing to install. There is no backend: your plan file is read
from and written to your own computer and is never uploaded anywhere.

![screenshot_loan](https://github.com/aero530/fpapp/raw/main/egui/screenshot.png "Retirement")

[User Manual](https://github.com/aero530/fpapp/raw/main/USER_MANUAL.md)

## Features

- Simulate income and expenses through retirement
- Track historic account balances
- Support multiple account types
  - Income
  - Retirement (IRA, Roth IRA, 401K)
  - Social Security
  - College Savings (529)
  - Expenses (such as grocery, car, utilities, insurance, entertainment, rent, etc.)
  - Loans (student, car, etc.)
  - Mortgage
  - Savings
  - Health Savings Account (HSA)
- Make pretty graphs
- Financial data saved locally as human readable json file
- Run as a desktop app, or self-host it as a WebAssembly page on any web server


## Installing it

**Windows** — download `FinancialPlanner-<version>-x64.msi` from the
[latest release](https://github.com/aero530/fpapp/releases/latest) and run it.

It installs for the current user only, into
`%LOCALAPPDATA%\Programs\Financial Planner`, so there is no UAC prompt. A new
version replaces the old one in place. The installer is not code-signed, so
SmartScreen warns the first time it is downloaded. If the retired Tauri version
(4.x, "fpapp") is still installed, uninstall it separately — it lives in
Program Files under a different name and this package leaves it alone.

To build the MSI locally:

```powershell
.\installer\build.ps1            # -> target\installer\FinancialPlanner-5.1.0-x64.msi
.\installer\verify.ps1           # installs it, launches it, uninstalls it again
```

`build.ps1` provisions a pinned, checksummed WiX 3.14 into `target/` on first
run; nothing is installed system-wide and no administrator rights are needed.
The wizard artwork, the application icon and the licence page are all generated
from `egui/assets/icon-256.png` and `LICENSE` at build time, so there is no
second copy of either to drift.

Pushing a `v*` tag runs [.github/workflows/release.yml](.github/workflows/release.yml),
which builds and verifies the MSI on a Windows runner and attaches it to the
matching GitHub release. It can also be run by hand from the Actions tab.


## Running it

**Desktop** — `cargo run --release` starts the native app (eframe/wgpu).

**Browser** — the same app also compiles to WebAssembly. The
[online demo](https://aero530.github.io/fpapp/) is that build, published to
GitHub Pages from `main` by
[.github/workflows/pages.yml](.github/workflows/pages.yml).

It can equally be self-hosted as static files on any web server:

```powershell
.\egui\web\build.ps1        # or ./egui/web/build.sh on Linux
```

then copy `egui/web/dist/` into the document root. There is no backend and no
uploading: plans are opened from and saved back to the machine running the
browser. See [egui/web/README.md](egui/web/README.md) for Apache configuration
and the details of how saving works per browser.


## Computation Flow

- Build simulation order: collect UUIDs grouped by AccountType in fixed sequence
  (Income → Expense → HSA → Mortgage → Loan → College → Retirement → Savings → SSA),
  sorted by account name within each type so results are deterministic.
  SSA runs last so its taxable-benefit calculation sees all other income for
  the year, including retirement/savings withdrawals.
- For each account, call init(linked_dates, settings):
    - Seeds internal year tables from historical user data
    - Resolves the account's active date ranges
- Main loop over each year y in [year_start, year_die]:
  - Open year: initialize all accumulators; carry net forward from y-1
    (zero, negative, and positive balances all roll forward)
  - For each account (in type order):
    - Resolve linked_value (income balance of the linked income account, if any)
    - Call account.simulate(y, &totals, &settings, linked_value) → YearlyImpact
      (all internal logic — earnings, contributions, withdrawals, payments,
       expense amounts, tax treatment — is encapsulated here)
    - Accumulate YearlyImpact into YearlyTotals immediately
      (later accounts in the same year see the updated totals)
  - Set totals.saving = Σ balances of Savings + Retirement accounts and
    totals.hsa = Σ balances of HSA accounts (College balances are excluded
    from the savings pool — they are earmarked for education)
  - End-of-year settlement:
    - Add income → net
    - Deduct max(0, income_taxable) × tax_income
      + max(0, capital_gains) × tax_capital_gains from net; record as tax_burden
    - Deduct expense from net
    - Deduct remaining healthcare_expense from net (zero it out;
      HSA already covered what it could during its simulate() call)
- Collect per-account plot data
- Return (plot_data, yearly_totals)

## Data file format

The app reads and writes a single JSON file. Top-level structure:

```json
{
  "settings": {
    "ageRetire": 65,
    "ageDie": 90,
    "yearBorn": 1975,
    "yearStart": 2020,
    "inflationBase": 3.0,
    "taxIncome": 22.0,
    "taxCapitalGains": 15.0,
    "retirementCostOfLiving": 80.0,
    "ssa": {
      "breakpoints":             { "low": 25000, "high": 34000 },
      "taxableIncomePercentage": { "low": 50,    "high": 85    }
    }
  },
  "accounts": {
    "<uuid>": {
      "type": "retirement",
      "name": "My 401k",
      "startIn": "yearStart",
      "endIn": { "base": "yearRetire", "delta": -1 },
      "startOut": "yearRetire",
      "endOut": "yearDie",
      "contributionValue": 19500,
      "contributionType": "fixed",
      "yearlyReturn": 6.0,
      "withdrawalType": "end_at_zero",
      "taxStatus": "contribute_pretax_taxed_when_used",
      "table": { "2020": 85000 },
      "incomeLink": "<uuid> | null"
    }
  }
}
```

The UI holds this entire blob as a `serde_json::Value`. It is only deserialised into typed Rust structs when `analyze::run_analysis()` is called.


## To do

- [ ] Improve tests to verify calculations for all account types


## Revision History ##

### v0.0.1 - 8.3.12 ###

- Initial development in Octave

### v0.0.2 - 8.27.12 ###

- Convert to SciLab.

### v0.0.3 - 9.1.12 ###

- Update input numbers

### v0.0.4 - 12.22.12 ###

- Update input numbers

### v0.0.5 - 10.27.13 ###

- Update input numbers - <http://money.msn.com/retirement/retirement-calculator.aspx>

### v0.1.0 - 12.30.13 ###

- Convert to Python

### v0.1.1 - 6.1.14 ###

- Update input numbers

### v0.1.2 - 12.7.14 ###

- Update input numbers

### v0.1.3 - 12.1.15 ###

- Update input numbers

### v0.1.4 - 3.10.17 ###

- Update input numbers

### v1.0.0 - 10.5.18 ###

- Convert to JS / electron
- Save user data as json instead of at the beginning of the code file
- Release v1.0.0

### v1.0.1 - 10.9.18 ###

- Added social security account type

### v1.0.2 - 10.10.18 ###

- Update with new theme

### v1.0.3 - 11.14.18 ###

- Update to electron 3.0

### v1.0.4 - 12.29.18 ###

- Update to babel 7
- Migrate from 2 package.json to single package.json
- Replace react-router-redux with connected-react-router
- Remove unused dependencies

### v2.0.0 - 10.3.19 ###

- Change to new project template
- Update dependencies
- Fix calculation bugs
- Add social security income source

### v2.1.0 - 10.17.19 ###

- Add file-new
- Refresh pages on file-open or file-new

### v2.1.1 - 12.9.19 ###

- Fix data type storage bug from MUI Editable table fields

### v3.0.0 - ________ ###

- Convert to Rust & Tauri

### v4.0.0 - ________ ###

- Update to Tauri 2.0

### v5.0.0 - 6/30/26 ###

- Convert to egui
- Leave depricated tauri version in place for now

### v5.1.0 - 8/4/26 ###

- Add a WebAssembly build of the egui app, self-hostable as static files
- Open and save plans from the browser without uploading them anywhere
- Retire the deprecated tauri version (tagged `tauri-final` before removal)
- Add a per-user Windows installer (MSI), built and verified by CI on a `v*` tag
- Publish the web build to GitHub Pages as an online demo