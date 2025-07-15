use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, parse_str, DeriveInput, Ident, LitStr, Token};

mod helpers;
use helpers::*;

#[allow(dead_code)]
struct KV {
    key: String,
    eq_token: Token![=],
    value: LitStr,
}

impl Parse for KV {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(KV {
            key: {
                match input.parse::<Ident>() {
                    Ok(v) => v.to_string(),
                    Err(_) => input.parse::<Token![crate]>().unwrap().to_token_stream().to_string(),
                }
            },
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[proc_macro_derive(FromForm, attributes(from_str, rename, ff))]
pub fn ff_derive(input: TokenStream) -> TokenStream {
    let (cont, name, data) = preamble(parse_macro_input!(input as DeriveInput));

    let attrs = cont
        .attrs
        .into_iter()
        .filter(|v| {
            v.path()
                .get_ident()
                .map(|v| v.to_string() == "ff")
                .unwrap_or(false)
        })
        .map(|v| v.parse_args::<KV>().unwrap())
        .map(|v| (v.key.to_string(), v.value.value()))
        .collect::<HashMap<String, String>>();

    let crate_path = attrs
        .get("path")
        .cloned()
        .unwrap_or("::from_form".to_string());

    let crate_path = parse_str::<syn::Path>(&crate_path).unwrap();
    let fields = data.fields.into_iter();
    let fields_str = fields
        .clone()
        .map(|f| {
            let ident = f.ident.unwrap();
            LitStr::new(&ident.to_string(), ident.span())
        })
        .collect::<Vec<LitStr>>();

    // Standard (From<String>) impls
    let ss = filter_opts(fields.clone(), false, false);
    let so = filter_opts(fields.clone(), true, false).map(|mut f| {
        f.ty = get_inner_type(f.ty.clone());
        f
    });

    // Non-Standard (FromStr) impls
    let nss = filter_opts(fields.clone(), false, true);
    let nso = filter_opts(fields.clone(), true, true).map(|mut f| {
        f.ty = get_inner_type(f.ty.clone());
        f
    });

    let (ss_field, ss_name, ss_type) = collect(ss);
    let (so_field, so_name, so_type) = collect(so);

    let (nss_field, nss_name, nss_type) = collect(nss);
    let (nso_field, nso_name, nso_type) = collect(nso);

    let implimentation = quote! {
        impl TryFrom<::std::collections::HashMap<String,String>> for #name {
            type Error = String;
            fn try_from(form_data: ::std::collections::HashMap<String,String>) -> Result<Self, Self::Error> {
                Ok({
                    use ::std::str::FromStr as _;
                    #name {
                        #(
                            #ss_field: <#ss_type>::try_from(
                                form_data.get(#ss_name).ok_or_else(|| format!("{} not found", #ss_name))?.to_string()
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                        #(
                            #so_field: match form_data.get(#so_name).ok_or_else(|| format!("{} not found", #so_name))?.to_string().is_empty() {
                                false => Some(<#so_type>::try_from(form_data.get(#so_name).unwrap().to_string()).map_err(|e| format!("{:?}", e))?),
                                true => None,
                            },
                        )*
                        #(
                            #nss_field: <#nss_type>::from_str(
                                form_data.get(#nss_name).ok_or_else(|| format!("{} not found", #nss_name))?
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                        #(
                            #nso_field: match form_data.get(#nso_name).ok_or_else(|| format!("{} not found", #nso_name))?.to_string().is_empty() {
                                false => Some(<#nso_type>::from_str(form_data.get(#nso_name).unwrap()).map_err(|e| format!("{:?}", e))?),
                                true => None,
                            },
                        )*
                    }
                })
            }
        }

        impl TryFrom<::std::collections::HashMap<::std::sync::Arc<str>,::std::sync::Arc<str>>> for #name {
            type Error = String;
            fn try_from(form_data: ::std::collections::HashMap<::std::sync::Arc<str>,::std::sync::Arc<str>>) -> Result<Self, Self::Error> {
                Ok({
                    use ::std::str::FromStr as _;
                    #name {
                        #(
                            #ss_field: <#ss_type>::try_from(
                                form_data.get(#ss_name).map(|v| v.as_ref()).ok_or_else(|| format!("{} not found", #ss_name))?.to_string()
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                        #(
                            #so_field: match form_data.get(#so_name).map(|v| v.as_ref()).ok_or_else(|| format!("{} not found", #so_name))?.to_string().is_empty() {
                                false => Some(<#so_type>::try_from(form_data.get(#so_name).map(|v| v.as_ref()).unwrap().to_string()).map_err(|e| format!("{:?}", e))?),
                                true => None,
                            },
                        )*
                        #(
                            #nss_field: <#nss_type>::from_str(
                                form_data.get(#nss_name).map(|v| v.as_ref()).ok_or_else(|| format!("{} not found", #nss_name))?
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                        #(
                            #nso_field: match form_data.get(#nso_name).map(|v| v.as_ref()).ok_or_else(|| format!("{} not found", #nso_name))?.to_string().is_empty() {
                                false => Some(<#nso_type>::from_str(form_data.get(#nso_name).map(|v| v.as_ref()).unwrap()).map_err(|e| format!("{:?}", e))?),
                                true => None,
                            },
                        )*
                    }
                })
            }
        }

        impl #crate_path::FromForm for #name {
            const COLUMNS: &'static [&'static str] = &[ #(#fields_str),* ];
        }

    };

    TokenStream::from(implimentation)
}

/// Expect a different name coming from the form
#[proc_macro_attribute]
pub fn rename(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Use FromStr rather than TryFrom<String>
#[proc_macro_attribute]
pub fn from_str(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
