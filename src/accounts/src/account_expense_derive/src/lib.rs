extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AccountExpense)]
pub fn expense_type_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_expense_type(&ast)
}

fn impl_expense_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AccountExpense for #name<u32> {
            fn get_expense(&self, year:u32, settings: &Settings ) -> f64 {
                match self.expense_type {
                    ExpenseOptions::Fixed => self.expense_value,
                    ExpenseOptions::FixedWithInflation => {
                        fixed_with_inflation(self.expense_value, year, settings)
                    }
                }
            }
        }
    };
    gen.into()
}
