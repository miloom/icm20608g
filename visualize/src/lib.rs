extern crate proc_macro;

use proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

#[proc_macro_derive(PrintTable)]
pub fn visualize(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    
    let struct_name = &input.ident;

    let check_trait = quote! {
        impl #struct_name {
            fn assert_impl_ReadRegister() where Self: ReadRegister {}
        }
    };

    let expanded = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields) = data_struct.fields {
            let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let table_rows: Vec<_> = field_names.iter().map(|name| {
                quote! {
                    vec![stringify!(#name).cell(), self.#name.cell()]
                }
            }).collect();

            quote! {
                impl #struct_name {
                    pub fn print_table(&self, i2c: &mut I2c) -> Result<()> {
                        Self::assert_impl_ReadRegister();
                        println!("{}", stringify!(#struct_name));
                        let table = vec![
                            #(#table_rows),*
                        ]
                        .table()
                        .title(vec![
                            "Field".cell().bold(true),
                            "Value".cell().bold(true),
                        ])
                        .bold(true);
                        print_stdout(table)?;
                        Ok(())
                    }
                }
            }

        } else {
            proc_macro2::TokenStream::new()
        }
    } else {
        proc_macro2::TokenStream::new()
    };
    // Combine the generated code
    let output = quote! {
        #check_trait
        #expanded
    };

    output.into()
}
