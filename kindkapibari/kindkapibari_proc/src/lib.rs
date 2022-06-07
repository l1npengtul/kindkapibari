#![allow(unused_variables)]

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
                syn::Fields::Named(ref n) => {
                    let flds = n.named.iter().next().unwrap();
                    let flds_ident = flds.ident.as_ref().unwrap();
                    let flds_str = format!("{flds:?}");
                    let flds_typ = &flds.ty;

                    parse_onearg.push(quote! {
                        {
                            #variant(Result::map_err(<#flds_typ>::parse(subtag), || ())?)
                        }
                    });
                    parse_plain_string_from_onearg.push(variant.ident.to_string());
                    fields.push(quote! {
                        {
                            format!("{}({})", core::stringify!(#variant), #flds_str)
                        }
                    });
                }
                syn::Fields::Unnamed(ref u) => {
                    let flds = u.unnamed.iter().next().unwrap();
                    let flds_str = format!("{flds:?}");
                    let flds_typ = &flds.ty;

                    parse_onearg.push(quote! {
                        {
                            #variant(Result::map_err(<#flds_typ>::parse(subtag), || ())?)
                        }
                    });
                    parse_plain_string_from_onearg.push(variant.ident.to_string());

                    fields.push(quote! {
                        {
                            format!("{}({})", core::stringify!(#variant), #flds_str)
                        }
                    });
                }
                syn::Fields::Unit => {
                    parse_noarg.push(quote! {
                        {
                            #variant
                        }
                    });
                    parse_plain_string_from_noarg.push(variant.ident.to_string());
                    fields.push(quote! {
                        {
                            core::stringify!(#variant)
                        }
                    });
                }
            }
            field.push(variant.ident);
        })
    }

    let out = quote! {
        impl core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    Self::to_attr_string(self)
                )
            }
        }

        impl core::fmt::Display for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl core::str::FromStr for #ident {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                const ALLOWED_CHARS: &[char] = &[
                    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
                    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
                    '5', '6', '7', '8', '9',
                ];

                let std::str::replace(s, std::slice::concat([ALLOWED_CHARS, &['(', ')']])) != "" {
                    _ => return Err(()),
                }

                let splitted = std::iter::Iterator::collect::<Vec<String>>(std::str::replace(s, &['(', ')']));
                if Vec::len(splitted) == 1 {
                    Ok(String::from(match String::as_ref(splitted[0]) {
                        #(#parse_plain_string_from_noarg => #parse_noarg),*
                        _ => return Err(()),
                    }))
                } else if Vec::len(splitted) == 2 {
                    let subtag = splitted[1];

                    Ok(String::from(match String::as_ref(splitted[0]) {
                        #(#parse_plain_string_from_onearg => #parse_onearg),*
                        _ => return Err(()),
                    }))
                } else {
                    _ => return Err(()),
                }
            }
        }

        impl #ident {
            pub fn to_attr_string(&self) -> String {
                match &self {
                    #(#field => #fields),*
                    _ => String::from(""),
                }
            }

            pub fn to_snake_case(&self) -> String {
                use convert_case::{Case, Casing};
                <Self::to_attr_string(self) as Casing>::to_case(Case::Snake)
            }

            pub fn to_camel_case(&self) -> String {
                use convert_case::{Case, Casing};
                <Self::to_attr_string(self) as Casing>::to_case(Case::Camel)
            }

            pub fn to_pascal_case(&self) -> String {
                use convert_case::{Case, Casing};
                <Self::to_attr_string(self) as Casing>::to_case(Case::Pascal)
            }

            pub fn to_kebab_case(&self) -> String {
                use convert_case::{Case, Casing};
                <Self::to_attr_string(self) as Casing>::to_case(Case::Kebab)
            }
        }
    };

    proc_macro::TokenStream::from(out)
}
