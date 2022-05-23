use proc_macro::TokenStream;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};
use quote::quote;

#[proc_macro_derive(AttrString)]
pub fn AttrString(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = parse_macro_input!(input);

    let mut fields = vec![];
    if let syn::Data::Enum(DataEnum {variants, ..}) = data {
        variants.into_iter().for_each(|variant| {
            let festr = variant.fields.into_iter().map(|enum_f| {
                
            })
        })

    }

    let out = quote! {
        impl #ident {
            pub fn to_attr_string(&self) -> String {
                match &self {
                    #(#fields => <#fields>::to_string()),*
                }
            }

            pub fn to_snake_case(&self) -> String {
                use convert_case::{Case, Casing};

                self.to_attr_string().to_case(Case::Snake)
            }

            pub fn to_camel_case(&self) -> String {
                use convert_case::{Case, Casing};

                self.to_attr_string().to_case(Case::Camel)
            }

            pub fn to_pascal_case(&self) -> String {
                use convert_case::{Case, Casing};

                self.to_attr_string().to_case(Case::Pascal)
            }

            pub fn to_kebab_case(&self) -> String {
                use convert_case::{Case, Casing};

                self.to_attr_string().to_case(Case::Kebab)
            }
        }
    };

    proc_macro::TokenStream::from(out)
}
