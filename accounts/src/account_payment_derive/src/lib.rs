extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AccountPayment)]
pub fn payment_type_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_payment_type(&ast)
}

fn impl_payment_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AccountPayment for #name<u32> {
            fn get_payment(&self, year:u32, settings: &Settings ) -> f64 {
                let output = match self.payment_type {
                    PaymentOptions::Fixed => self.payment_value,
                    PaymentOptions::FixedWithInflation => {
                        fixed_with_inflation(self.payment_value, year, settings)
                    }
                };
                let outstanding_balance = self.analysis.value.get(year).unwrap();
                if output > outstanding_balance {
                    outstanding_balance
                } else {
                    output
                }
            }
        }
    };
    gen.into()
}
