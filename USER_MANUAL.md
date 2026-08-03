# Financial Planner — User Manual

The Financial Planner models your income, savings, expenses, and debt over your lifetime and projects the result year by year. You describe your financial picture by adding accounts, configure a few global settings, and the app re-simulates everything automatically whenever anything changes.

---

## Interface layout

```
┌─────────────────┬──────────────────────────────────────────────┐
│   Sidebar       │   Main panel                                  │
│                 │                                               │
│  📊 Dashboard   │   (Dashboard / Settings / Account form)       │
│  ⚙  Settings    │                                               │
│  ─────────────  │                                               │
│  ▸ Income (1)   │                                               │
│  ▸ Savings (2)  │                                               │
│  ▸ Expense (3)  │                                               │
│    …            │                                               │
│  ─────────────  │                                               │
│  New Open Save… │                                               │
└─────────────────┴──────────────────────────────────────────────┘
```

The **sidebar** on the left is always visible. It contains:
- Navigation links to the Dashboard and Settings pages
- A scrollable account tree, grouped by account type
- File controls at the bottom

The **main panel** shows whichever page is selected. All edits take effect immediately — the simulation re-runs in the background every time you change a value.

---

## Files

All data is stored in a single JSON file. The app does not auto-save.

| Button | Action |
|---|---|
| **New** | Start a new plan with default settings and no accounts |
| **Open…** | Open an existing data file |
| **Save** | Save to the currently open file (disabled until a plan is open) |
| **Save As…** | Save to a new file |

Start fresh by clicking **New** (also offered on the welcome screen). The Settings page opens with sensible defaults — adjust them, add your accounts, then click **Save As…** to choose where the file lives. You can maintain separate files for different scenarios.

---

## Settings

Click **⚙ Settings** in the sidebar to configure the global parameters that apply to all accounts.

### Life timeline

| Field | Description |
|---|---|
| **Year Born** | Your birth year. Used to calculate `yearRetire` and `yearDie`. |
| **Year Start** | First year of the simulation, typically the current year. |
| **Age at Retire** | The age at which you plan to retire. Sets the `yearRetire` reference used in account date fields. |
| **Age at Death** | The age used as the end of the simulation. Sets `yearDie`. |

The derived values **Retire in** and **End year** are shown below the form for quick reference.

### Tax and inflation

| Field | Description |
|---|---|
| **Income Tax Rate (%)** | Flat marginal rate applied to ordinary income and traditional retirement withdrawals. |
| **Capital Gains Tax Rate (%)** | Rate applied to investment earnings in taxable accounts. |
| **Inflation Rate (%)** | Expected annual inflation, used when any account input is set to *Fixed + Inflation*. |
| **Retirement Cost of Living (%)** | Your expected spending in retirement as a percentage of pre-retirement spending. `80` means you expect to spend 80 cents for every dollar you spend today. Applies to expenses that have *Scales with retirement cost-of-living* checked. |

### Social Security (SSA)

The taxable fraction of Social Security benefits is interpolated linearly between two income breakpoints.

| Field | Description |
|---|---|
| **SSA Breakpoint Low ($)** | Combined income below this amount → 0% of SSA benefits are taxable. |
| **SSA Breakpoint High ($)** | Combined income above this amount → the maximum taxable fraction applies. |
| **SSA Taxable % (Low)** | Taxable fraction when income is between the two breakpoints. |
| **SSA Taxable % (High)** | Taxable fraction when income is above the high breakpoint. |

For 2024 the IRS thresholds are $25,000 / $34,000 for single filers and $32,000 / $44,000 for married filing jointly, with taxable fractions of 50% and 85%.

---

## Accounts

### Adding and deleting accounts

Accounts are listed in the sidebar under their type. Click the **▸** arrow next to a type to expand it and see the accounts in that group. Click **+ Add** inside any group to create a new account of that type with default values.

To delete an account, open it in the main panel and click the red **Delete** button in the top right. A confirmation dialog appears before anything is removed. Any other accounts that reference the deleted account via an Income Link will have that link cleared automatically.

### Date fields — year input format

All start and end year fields accept three formats:

| Input | Example | Meaning |
|---|---|---|
| A literal year | `2035` | That exact calendar year |
| A keyword | `yearRetire` | Resolved from your settings at simulation time |
| A keyword with an offset | `yearRetire-1`, `yearStart+5` | Keyword ± a whole number of years |

Available keywords: `yearStart`, `yearRetire`, `yearDie`, and `yearEnd` (same as `yearDie`). Accounts with an Income Link also accept `incomeLink`, which copies the corresponding start or end year from the linked income account.

Using keywords means you only need to update the Settings page when your plans change — all accounts that reference `yearRetire` update automatically.

If a start year resolves to a year *after* its matching end year (for example a contribution window ending at `yearRetire-1` when you are already retired), the window is simply empty and no money flows during it. This is not an error.

---

### Income

Salary, wages, or any other regular earned income.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Base Pay ($)** | Annual income in today's dollars. |
| **Start Year** | First year income is received. |
| **End Year** | Last year income is received (inclusive). Typically `yearRetire`. |
| **Yearly Raise (%)** | Annual percentage increase in pay. Can be a number (`3.5`) or `inflationBase` to track the inflation rate. |
| **Notes** | Free text for your own reference. |

---

### Social Security (SSA)

Social Security retirement benefits. The taxable portion is calculated each year based on your total income and the SSA breakpoints in Settings.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Base Benefit ($)** | Annual benefit amount in today's dollars. |
| **Start Year** | Year benefits begin. Typically `yearRetire`. |
| **End Year** | Year benefits end. Typically `yearDie`. |
| **Notes** | Free text for your own reference. |

---

### Retirement

Tax-advantaged savings accounts: 401(k), IRA, Roth IRA, etc.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Contribution Start / End** | Years when money flows into the account. |
| **Withdrawal Start / End** | Years when money flows out of the account. |
| **Contribution Type** | How the contribution amount is interpreted (see below). |
| **Contribution Value** | Dollar amount or percentage, depending on Contribution Type. |
| **Yearly Return (%)** | Expected annual investment return. |
| **Withdrawal Type** | How withdrawals are calculated (see below). |
| **Withdrawal Value** | Dollar amount used when Withdrawal Type is Fixed or Fixed + Inflation. |
| **Tax Status** | How contributions and withdrawals are taxed (see below). |
| **Income Link** | Optional: link to an Income account to base percentage contributions on that account's income. |
| **Employer Matching** | Check to enable employer matching. Enter the income percentage cap and the matching percentage. |
| **Notes** | Free text for your own reference. |

**Contribution types:**

| Option | Behaviour |
|---|---|
| Fixed Amount | The same dollar amount every year (in today's dollars). |
| % of Income | A percentage of the linked income account's value for that year. Without an Income Link, the percentage applies to the total income for the year instead. |
| Fixed + Inflation | The amount grows with the inflation rate from Year Start. |

**Withdrawal types:**

| Option | Behaviour |
|---|---|
| Fixed Amount | The same dollar amount every year. |
| Fixed + Inflation | The amount grows with inflation from Year Start. |
| Draw Down to Zero | Calculates equal annual withdrawals so the balance reaches zero by the Withdrawal End year. |
| Fraction of Savings | Withdraws an amount proportional to this account's share of total savings, scaled by your cost-of-living spending. |
| Other | No automatic withdrawals. Use for accounts you only contribute to. |

**Tax status:**

| Option | Description |
|---|---|
| Roth | Contributions from after-tax income; withdrawals are tax-free. |
| Taxed Both Ways | Taxed on the way in and on the way out. |
| Traditional | Contributions reduce taxable income; withdrawals are taxed as ordinary income. |
| Tax-Free (HSA/529) | Pre-tax contributions; withdrawals are tax-free. |

---

### HSA (Health Savings Account)

Funds contributed pre-tax and used to pay healthcare expenses tax-free. HSA accounts are automatically applied against the year's healthcare expenses (any Expense account with *Healthcare expense* checked); whatever the HSAs cannot cover is paid from your cash balance.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Contribution Start / End** | Years when money flows into the account. |
| **Withdrawal Start / End** | Years when HSA funds can be used to cover healthcare costs. |
| **Contribution Type** | Fixed Amount, % of Income, or Fixed + Inflation. |
| **Contribution Value** | Your annual contribution in today's dollars. |
| **Employer Contribution ($)** | Employer's fixed annual contribution in today's dollars. |
| **Yearly Return (%)** | Investment return on the HSA balance. |
| **Tax Status** | Fixed to *Tax-Free (HSA)* — pre-tax contributions (a deduction), tax-free withdrawals for healthcare. |
| **Notes** | Free text for your own reference. |

To have an HSA pay for a cost, open the Expense account and check **Healthcare expense** — all HSA accounts are drawn from automatically.

---

### College (529 / Education Savings)

Savings earmarked for education expenses. Contributions are made with after-tax dollars; qualified withdrawals are tax-free.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Contribution Start / End** | Years when money is deposited. |
| **Withdrawal Start / End** | Years when funds are withdrawn for education costs. |
| **Contribution Type** | Fixed Amount, % of Income, or Fixed + Inflation. |
| **Contribution Value** | Annual contribution in today's dollars. |
| **Yearly Return (%)** | Expected investment return. |
| **Withdrawal Type** | Fixed Amount, Fixed + Inflation, Draw Down to Zero, or Other. |
| **Withdrawal Value** | Annual withdrawal amount when not drawing down to zero. |
| **Tax Status** | Fixed at *Post-tax contributions, tax-free withdrawals (529-style)*. |
| **Notes** | Free text for your own reference. |

College account balances are tracked separately and do not contribute to the general savings pool shown in the Dashboard.

> **Don't double-count tuition.** The simulation assumes college withdrawals pay education costs directly — tuition never touches your cash balance or yearly expenses. Do **not** also create an Expense account for the same tuition, or the cost will be counted twice.

---

### Expense

Any recurring cost: utilities, insurance, subscriptions, groceries, etc.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Start Year** | First year the expense occurs. |
| **End Year** | Last year the expense occurs (inclusive). |
| **Expense Type** | Fixed Amount or Fixed + Inflation. |
| **Expense Value ($)** | Annual cost in today's dollars. |
| **Healthcare expense** | Check if this is a medical cost that should be covered by HSA accounts first; any remainder is paid from your cash balance. |
| **Scales with retirement cost-of-living factor** | If checked, the expense is reduced in retirement by the cost-of-living percentage set in Settings. |
| **Notes** | Free text for your own reference. |

---

### Loan

Any non-mortgage debt: auto loans, student loans, personal loans.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Start Year** | First year payments are made. |
| **End Year** | Last year payments are made. |
| **Payment Type** | Fixed Amount or Fixed + Inflation. |
| **Annual Payment ($)** | Total yearly payment amount in today's dollars. |
| **Interest Rate (%)** | Annual percentage rate (APR). Accepts a number or the `inflationBase` keyword. |
| **Notes** | Free text for your own reference. |

The simulation calculates how much of each payment covers interest versus principal. The outstanding balance is tracked internally.

If the payment does not cover the interest (the balance grows), or the End Year arrives with a balance still outstanding, a warning appears in the sidebar — the simulation continues either way. The same warnings apply to mortgages.

---

### Mortgage

Home loan with support for escrow, mortgage insurance (PMI), and LTV-based PMI removal.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Start Year** | First year payments are made. |
| **End Year** | Loan payoff year. |
| **Payment Type** | Fixed Amount or Fixed + Inflation. |
| **Annual Payment ($)** | Total yearly payment including principal, interest, escrow, and mortgage insurance. |
| **Interest Rate (%)** | APR, compounded at the frequency set by Compound Periods/Year. Accepts a number or the `inflationBase` keyword. |
| **Compound Periods/Year** | `12` for monthly compounding (standard), `1` for annual. |
| **Mortgage Insurance ($)** | Annual PMI premium in today's dollars. Applied each year until the LTV drops below the limit. |
| **LTV Limit (%)** | Loan-to-value ratio at which PMI is no longer required (typically 80%). |
| **Escrow ($/yr)** | Annual property tax and insurance amount held in escrow. |
| **Home Value ($)** | Current home value, used to calculate the loan-to-value ratio for PMI. |
| **Notes** | Free text for your own reference. |

---

### Savings

General-purpose savings or brokerage accounts: taxable investment accounts, money market, etc.

| Field | Description |
|---|---|
| **Name** | Label shown in the sidebar and charts. |
| **Contribution Start / End** | Years when money flows into the account. |
| **Withdrawal Start / End** | Years when money flows out. |
| **Contribution Type** | Fixed Amount, % of Income, or Fixed + Inflation. |
| **Contribution Value** | Annual contribution in today's dollars. |
| **Yearly Return (%)** | Expected annual return. |
| **Withdrawal Type** | Fixed Amount, Fixed + Inflation, Draw Down to Zero, Fraction of Savings, or Other. |
| **Withdrawal Value** | Annual withdrawal when not drawing down to zero. |
| **Tax Status** | Roth, Taxed Both Ways, Traditional, or Tax-Free. |
| **Notes** | Free text for your own reference. |

---

## Historical data

Every account except Social Security has a **Historical Data** table at the bottom of its form. Use it to enter known past or current values:

- Add a row with the **+ Add row** button
- Set the **Year** and the **Amount** for that year

What the amount means, and whether it can be negative, depends on the account type:

- **Balance accounts** (Retirement, Savings, HSA, College, Loan, Mortgage): the account balance, which must be zero or greater. For a year *before* the simulation starts, enter the end-of-year balance — it becomes the opening balance when the simulation begins. For a year *inside* the simulation window, the value is used as that year's **opening** balance, and the year's earnings, contributions, withdrawals, and payments are applied on top of it.
- **Flow accounts** (Income, Expense): the actual amount earned or spent that year. A recorded actual replaces the computed value for that year — even if the year falls outside the account's start/end window. These can be negative: a net-loss year for an Income account, or a refund/credit year for an Expense account, are both legitimate. The one exception is an Expense marked **Healthcare expense** — its historical amounts must be zero or greater, since a negative healthcare cost cannot be reconciled against HSA coverage.

Historical data is particularly useful for existing retirement accounts, HSA balances, or outstanding loan principals. Years before the first simulated year appear in the account's Projection chart but are otherwise only used to seed opening balances.

---

## Dashboard

Click **📊 Dashboard** to see the summary charts. Charts appear once a data file is open and at least one account is configured.

| Chart | What it shows |
|---|---|
| **Net / Income / Expense** | Your liquid cash balance (net), total income, and total expenses year by year. The net line rolls forward — a positive and growing net line means you're accumulating cash. |
| **Savings & HSA** | The combined balance of all Retirement and Savings accounts (College balances are excluded), plus a separate line for the combined HSA balance. |
| **Cost of Living** | Total annual lifestyle spending tracked from your Expense accounts. Drops in retirement if Retirement Cost of Living is set below 100%. |
| **Healthcare & Tax Burden** | Annual healthcare costs and income tax paid (income tax plus capital-gains tax). |

Each account's own chart is shown at the bottom of its form page under **Projection**.

---

## Model assumptions and simplifications

The simulator favors clarity over tax-code fidelity. Keep these simplifications in mind when comparing its output against other tools:

- **Flat tax rates.** A single marginal income-tax rate and a single capital-gains rate apply to every year — no brackets, standard deduction, or payroll (FICA) taxes. Deductions can offset other income in the same year, but a negative taxable total never produces a refund.
- **Social Security combined income.** The taxable-benefit calculation counts *all* income for the year, including Roth withdrawals, which the real IRS rules exclude. Plans funded mainly from Roth accounts in retirement will show more SSA tax than the actual rules produce.
- **Employer match follows the account's tax status.** A match into a Roth account is modeled as Roth money (never taxed). Real employer matches are pre-tax and taxed at withdrawal.
- **"Taxed Both Ways" accounts realize gains annually.** All investment earnings are taxed in the year they are earned at the capital-gains rate — there is no deferral or cost-basis tracking.
- **College withdrawals settle off-ledger.** Withdrawals are assumed to exactly pay education costs; see the note in the College section about not double-counting tuition.
- **Annual timing.** All flows happen once per year: a full year of interest accrues before the year's loan or mortgage payment is applied, so debt interest is slightly overstated compared to monthly payments. Escrow stays constant over time and stops when the mortgage is paid off.
- **Non-amortizing debt is flagged, not fixed.** If a loan or mortgage payment doesn't cover its interest, or the payment window ends with a balance outstanding, the simulation continues and a warning appears in the sidebar.

---

## Tips

**Link retirement contributions to income.** Set Contribution Type to *% of Income* and select your salary under Income Link. This keeps contributions proportional to your pay automatically.

**Use "Draw Down to Zero" for retirement accounts.** When you don't know your exact withdrawal amount, this option calculates equal annual withdrawals that exhaust the balance precisely by the Withdrawal End year.

**Mark medical costs as healthcare expenses.** Create your HSA account, then open each medical Expense and check *Healthcare expense*. The simulation will draw from your HSA accounts first and charge any remainder to your cash balance.

**Hover over any field label for a tooltip** explaining what the field does and what units it expects.

**All dollar amounts are in today's dollars** unless stated otherwise. The simulation applies inflation automatically when you select *Fixed + Inflation* as a contribution or expense type.
