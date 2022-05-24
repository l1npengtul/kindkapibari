use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput};

#[proc_macro_derive(AttrString)]
pub fn attr_string(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut fields = vec![];
    let mut field = vec![];

    let mut parse_noarg = vec![];
    let mut parse_onearg = vec![];
    let mut parse_plain_string_from_noarg = vec![];
    let mut parse_plain_string_from_onearg = vec![];
    if let syn::Data::Enum(DataEnum { variants, .. }) = data {
        variants.into_iter().for_each(|variant| {
            // Enum -> String
            match variant.fields {
                syn::Fields::Named(n) => {
                    let flds = n.named.into_iter().next().unwrap();

                    parse_onearg.push(quote! {
                        {
                            <variant>{flds.ident.unwrap(): subtag.parse()?}
                        }
                    });
                    parse_plain_string_from_onearg.push(variant.ident.to_string());
                    fields.push(quote! {
                        {
                            let mut base_str = <variant.ident>::to_string();
                            format!("{base_str}({})", flds.to_string())
                        }
                    });
                }
                syn::Fields::Unnamed(u) => {
                    let flds = u.unnamed.into_iter().next().unwrap();

                    parse_onearg.push(quote! {
                        {
                            <variant>(subtag.parse()?)
                        }
                    });
                    parse_plain_string_from_onearg.push(variant.ident.to_string());

                    fields.push(quote! {
                        {
                            let mut base_str = <variant.ident>::to_string();
                            format!("{base_str}({})", flds.to_string())
                        }
                    });
                }
                syn::Fields::Unit => {
                    parse_noarg.push(quote! {
                        {
                            variant
                        }
                    });
                    parse_plain_string_from_noarg.push(variant.ident.to_string());
                    fields.push(quote! {
                        {
                            <variant.ident>::to_string()
                        }
                    });
                }
            }
            field.push(variant.ident);
        })
    }

    let out = quote! {
        impl Debug for #ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.to_attr_string()
                )
            }
        }

        impl Display for #ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        impl FromStr for #ident {
            type Err = Self::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s.replace([ALLOWED_CHARS, &['(', ')']].concat(), "") != "" {
                    return Err(Self::Err::from("not allowed characters".to_string()));
                }

                let splitted = s.split(&['(', ')']).collect::<Vec<String>>();
                if splitted.len() == 1 {
                    Ok(match splitted[0].as_str() {
                        #(#parse_plain_string_from_noarg => #parse_noarg),*
                    })
                } else splitted.len() == 2 {
                    let subtag = splitted[1];

                    Ok(match splitted[0].as_str() {
                        #(#parse_plain_string_from_onearg => #parse_onearg),*
                    })
                } else {
                    return Err(Self::Err::from("not found".to_string()));
                }
            }
        }

        impl #ident {
            pub fn to_attr_string(&self) -> String {
                match &self {
                    #(#field => #fields),*
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
