export const taxStatusTypeOptions = [
  {
    value: 0,
    label: 'contribute taxed income - earnings taxed deferred',
    description:
      'payed with taxed income, earnings are tax deferred, withdrawals are not taxed',
  },
  {
    value: 1,
    label: 'contribute taxed income - earings are capital gains',
    description:
      'payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)',
  },
  {
    value: 2,
    label: 'not implemented',
    description:
      'NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed',
  },
  {
    value: 3,
    label: 'contribute pretax income - taxed as income when used',
    description: 'payed pretax and taxed in year of use as income',
  },
  {
    value: 4,
    label: 'contribute pretax income - withdrawal not taxed as income (HSA)',
    description: 'payed pretax and not taxed as income (use with HSA)',
  },
];

export const contributionTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'fixed dollar amount',
  },
  {
    value: 'percent_of_income',
    label: 'percent of income',
    description: 'percent of cost of current living',
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description:
      'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
  },
];

export const expenseTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'fixed dollar amount',
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description:
      'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
  },
];

export const withdrawalTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'Take out a fixed dollar amount',
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description: 'Take out a fixed dollar amount with inflation compensation',
  },
  {
    value: 'end_at_zero',
    label: 'end at zero',
    description:
      'take money out in equal amounts each year such that the balance at endOut is zero',
  },
  {
    value: 'col_frac_of_savings',
    label: 'cost of living fraction of total savings',
    description:
      'Take out the current cost of living * (this accounts value / total savings)',
  },
  {
    value: 'percent_of_income',
    label: 'percent of income',
    description: 'Take out a percent of income in each year',
  },
];

export const paymentTypeOptions = [
  {
    value: 'fixed',
    label: 'fixed',
    description: 'fixed dollar amount',
  },
  {
    value: 'fixed_with_inflation',
    label: 'fixed with inflation',
    description:
      'fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)',
  },
];
