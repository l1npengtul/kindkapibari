#![allow(unused_variables)]

use proc_macro::TokenStream;
use quote::{quote, format_ident};
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
            let subtag = format_ident!("subtag");
            // Enum -> String
            match variant.fields {
                syn::Fields::Named(ref n) => {
                    let flds = n.named.iter().next().unwrap();
                    let flds_ident = flds.ident.as_ref().unwrap();
                    let flds_str = format!("{flds:?}");
                    let flds_ident_str = format!("{flds_ident:?}");
                    let flds_typ = &flds.ty;

                    if let syn::Type::Path(p) = flds_typ {
                        let flds_typ_ident = p.path.get_ident().unwrap();

                        parse_onearg.push(quote! {
                            {
                                Result::map_err(#ident::#variant{ #flds_ident : <#flds_typ_ident as core::str::FromStr>::from_str(#subtag) }, || -> () { () })? 
                            }
                        });
                        parse_plain_string_from_onearg.push(variant.ident.to_string());
                        fields.push(quote! {
                            {
                                format!("{}({}: {})", core::stringify!(#variant), #flds_ident_str, #flds_str)
                            }
                        });
                    }
                }
                syn::Fields::Unnamed(ref u) => {
                    let flds = u.unnamed.iter().next().unwrap();
                    let flds_str = format!("{flds:?}");
                    let flds_typ = &flds.ty;

                    if let syn::Type::Path(p) = flds_typ {
                        let flds_typ_ident = p.path.get_ident().unwrap();
                        println!("{flds_typ_ident:?}");
                        let variant_typ_ident = &variant.ident;
                        parse_onearg.push(quote! {
                            {
                                Result::map_err(#ident::#variant_typ_ident(<#flds_typ_ident as core::str::FromStr>::from_str(#subtag), || -> () { () }))?
                            }
                        });
                        parse_plain_string_from_onearg.push(variant.ident.to_string());
    
                        fields.push(quote! {
                            {
                                format!("{}({})", core::stringify!(#variant), #flds_str)
                            }
                        });
                    }
                }
                syn::Fields::Unit => {
                    parse_noarg.push(quote! {
                        {
                            #ident::#variant
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
                    #ident::to_attr_string(&self)
                )
            }
        }

        impl core::fmt::Display for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", &self)
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

                if str::replace(s, [ALLOWED_CHARS, &['(', ')']].concat()) != "" {
                    return Err(())
                }

                let splitted = s.replace(&['(', ')']).collect::<Vec<String>>();
                if Vec::len(splitted) == 1 {
                    Ok(match splitted[0] {
                        #(#parse_plain_string_from_noarg => #parse_noarg),*
                        _ => return Err(()),
                    })
                } else if Vec::len(splitted) == 2 {
                    let subtag = splitted[1];

                    Ok(match splitted[0] {
                        #(#parse_plain_string_from_onearg => #parse_onearg),*
                        _ => return Err(()),
                    })
                } else {
                    return Err(())
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
                self.to_attr_string().to_case(Case::Kebab)
            }

            pub fn to_camel_case(&self) -> String {
                use convert_case::{Case, Casing};
                self.to_attr_string().to_case(Case::Kebab)
            }

            pub fn to_pascal_case(&self) -> String {
                use convert_case::{Case, Casing};
                self.to_attr_string().to_case(Case::Kebab)
            }

            pub fn to_kebab_case(&self) -> String {
                use convert_case::{Case, Casing};
                self.to_attr_string().to_case(Case::Kebab)
            }
        }
    };

    proc_macro::TokenStream::from(out)
}
