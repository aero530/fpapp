/**
 * @const {Object}
 * @description specifies what account fields are shown for each account type
 */

export const show = {
  income: {
    name: true,
    table: true,
    startIn: true,
    endIn: true,
    raise: true,
    base: true,
    notes: true,
  },
  ssa: {
    name: true,
    startIn: true,
    endIn: true,
    base: true,
    notes: true,
  },
  retirement: {
    name: true,
    table: true,
    startIn: true,
    endIn: true,
    startOut: true,
    endOut: true,
    yearlyContribution: true,
    contributionType: true,
    yearlyReturn: true,
    withdrawalType: true,
    withdrawalValue: true,
    taxStatus: true,
    incomeLink: true,
    employerMatch: true,
    matchLimit: true,
    notes: true,
  },
  hsa: {
    name: true,
    table: true,
    startIn: true,
    endIn: true,
    startOut: true,
    endOut: true,
    yearlyContribution: true,
    employerContribution: true,
    contributionType: true,
    yearlyReturn: true,
    taxStatus: true,
    notes: true,
  },
  college: {
    name: true,
    table: true,
    startIn: true,
    endIn: true,
    startOut: true,
    endOut: true,
    yearlyContribution: true,
    contributionType: true,
    yearlyReturn: true,
    withdrawalType: true,
    withdrawalValue: true,
    taxStatus: true,
    notes: true,
  },
  expense: {
    name: true,
    table: true,
    startOut: true,
    endOut: true,
    expenseType: true,
    expenseValue: true,
    notes: true,
    isHealthcare: true,
    hsaLink: true,
  },
  loan: {
    name: true,
    table: true,
    startOut: true,
    endOut: true,
    paymentType: true,
    paymentValue: true,
    rate: true,
    notes: true,
  },
  mortgage: {
    name: true,
    table: true,
    startOut: true,
    endOut: true,
    paymentType: true,
    paymentValue: true,
    rate: true,
    compoundTime: true,
    mortgageInsurance: true,
    ltvLimit: true,
    escrowValue: true,
    value: true,
    notes: true,
  },
  savings: {
    name: true,
    table: true,
    startIn: true,
    endIn: true,
    startOut: true,
    endOut: true,
    yearlyContribution: true,
    contributionType: true,
    yearlyReturn: true,
    withdrawalType: true,
    withdrawalValue: true,
    taxStatus: true,
    notes: true,
  },
};

export default show;
