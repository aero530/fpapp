extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AccountSavings)]
pub fn savings_type_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_savings_type(&ast)
}

fn impl_savings_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AccountSavings for #name<u32> {
            fn get_contribution(&self, year:u32, totals: &YearlyTotals, settings: &Settings ) -> f64 {
                match self.contribution_type {
                    ContributionOptions::Fixed => {
                        // set the contribution amount to the value input
                        self.contribution_value
                    }
                    ContributionOptions::PercentOfIncome => {
                        // calculate the contribution using the total income for the year
                        totals.get_income(year) * self.contribution_value / 100_f64
                    }
                    ContributionOptions::FixedWithInflation => {
                        // increase the value by inflation
                        fixed_with_inflation(self.contribution_value, year, settings)
                    }
                }
            }
            fn get_withdrawal(&self, year:u32, totals: &YearlyTotals, settings: &Settings ) -> f64 {
                let output = match self.withdrawal_type {
                    WithdrawalOptions::Other => {
                        0_f64
                    }
                    WithdrawalOptions::Fixed => {
                        self.withdrawal_value
                    }
                    WithdrawalOptions::FixedWithInflation => {
                        //let start = self.dates.year_in.unwrap().start;
                        fixed_with_inflation(self.withdrawal_value, year, settings)
                    }
                    WithdrawalOptions::EndAtZero => {
                        let end_out = self.dates.year_out.unwrap().end;
                        let account_value = self.analysis.value.get(year).unwrap();
                        if year <= end_out {
                            // if the year to stop taking money out of the account is beyond or equal to the current year
                            // calculate the fraction of the account balance to withdraw
                            account_value / (end_out - year + 1) as f64
                        } else {
                            0_f64
                        }
                    }
                    WithdrawalOptions::ColFracOfSavings => {
                        let prev_account_value = self.analysis.value.get(year - 1).unwrap_or_default();
                        let prev_savings = totals.get_saving(year - 1);
                        let col_scale = match settings.is_retired(year) {
                            true => settings.retirement_cost_of_living / 100_f64,
                            false => 1_f64,
                        };
                        let col = totals.get_col(year);
                        if prev_account_value > 0_f64 && prev_savings > 0_f64 {
                            // if there is money left in the account
                            // withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                            // total expenses this year is reduced by the income during retirement for the year.
                            // incomeDuringRetirement is tracked because withdrawals from retirement accounts go into the income table but we want to
                            // pay for expenses from money earned in this year before pulling from retirement accounts.
                            //      const totalExpensesThisYear = Object.values(expenseTotal[yearCurrent]).reduce((acc, cur) => acc + cur, 0) - incomeDuringRetirement[yearCurrent];
                            //      withdrawal = (totalExpensesThisYear * account.table[yearCurrent - 1]) / savingsTotalTable[yearCurrent - 1];
        
                            match self.tax_status {
                                TaxStatus::ContributePretaxTaxedWhenUsed => {
                                    // add extra to amount withdrawal value to account for taxes.
                                    // col * (prev_account_value / prev_savings) * (tax_income / 100_f64 + 1_f64)
                                    col * (prev_account_value / prev_savings)
                                        / (1_f64 - settings.tax_income / 100_f64)
                                }
                                _ => col * (prev_account_value / prev_savings),
                            }
                        } else {
                            0_f64
                        }
                    }
                };

                // if there is not a value logged for this year, set to zero so that we can't remove any money
                let account_value = self.analysis.value.get(year).unwrap_or_default();
                match output > account_value {
                    true => account_value,
                    false => output,
                }
            }
        }
    };
    gen.into()
}
