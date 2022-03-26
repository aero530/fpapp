import type { AccountWrapperUI as Account } from "../src-tauri/src/accounts/bindings/AccountWrapperUI";

export let defaultCollege: Account ={
    type: "college",
    name: "",
    table: {},
    contributions: {},
    earnings: {},
    withdrawals: {},
    startIn: 0,
    endIn: 0,
    startOut: 0,
    endOut: 0,
    contributionValue: 0,
    contributionType: "fixed",
    yearlyReturn: "",
    withdrawalType: "fixed",
    withdrawalValue: 0,
    taxStatus: "contribute_taxed_earnings_untaxed_when_used",
    notes: ""
}
export let defaultExpense: Account ={
    type: "expense",
    name: "",
    table: {},
    startOut: 0,
    endOut: 0,
    expenseType: "fixed",
    expenseValue: 0,
    isHealthcare: false,
    hsaLink: "",
    notes: ""
}
export let defaultHsa: Account ={
    type: "hsa",
    name: "",
    table: {},
    startIn: 0,
    endIn: 0,
    startOut: 0,
    endOut: 0,
    contributionValue: 0,
    contributionType: "fixed",
    employerContribution: 0,
    yearlyReturn: "",
    withdrawalType: "fixed",
    withdrawalValue: 0,
    taxStatus: "contribute_taxed_earnings_untaxed_when_used",
    notes: ""
}
export let defaultIncome: Account ={
    type: "income",
    name: "",
    table: {},
    base: 0,
    startIn: 0,
    endIn: 0,
    raise: "",
    notes: ""
}
export let defaultLoan: Account ={
    type: "loan",
    name: "",
    table: {},
    startOut: 0,
    endOut: 0,
    paymentType: "fixed",
    paymentValue: 0,
    rate: "",
    notes: ""
}
export let defaultMortgage: Account ={
    type: "mortgage",
    name: "",
    table: {},
    startOut: 0,
    endOut: 0,
    paymentType: "fixed",
    paymentValue: 0,
    rate: "",
    compoundTime: 0,
    mortgageInsurance: 0,
    ltvLimit: 0,
    escrowValue: 0,
    homeValue: 0,
    notes: ""
}
export let defaultRetirement: Account ={
    type: "retirement",
    name: "",
    table: {},
    contributions: {},
    earnings: {},
    withdrawals: {},
    employerContributions: {},
    startIn: 0,
    endIn: 0,
    startOut: 0,
    endOut: 0,
    contributionValue: 0,
    contributionType: "fixed",
    yearlyReturn: "",
    withdrawalType: "fixed",
    withdrawalValue: 0,
    taxStatus: "contribute_taxed_earnings_untaxed_when_used",
    incomeLink: "",
    matching: {},
    notes: ""
}
export let defaultSavings: Account ={
    type: "savings",
    name: "",
    table: {},
    contributions: {},
    earnings: {},
    withdrawals: {},
    startIn: 0,
    endIn: 0,
    startOut: 0,
    endOut: 0,
    contributionValue: 0,
    contributionType: "fixed",
    yearlyReturn: "",
    withdrawalType: "fixed",
    withdrawalValue: 0,
    taxStatus: "contribute_taxed_earnings_untaxed_when_used",
    notes: ""
}
export let defaultSsa: Account ={
    type: "ssa",
    name: "",
    base: 0,
    startIn: 0,
    endIn: 0,
    notes: ""
}