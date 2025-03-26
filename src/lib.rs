use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod helpers;
use helpers::*;

#[proc_macro_derive(FromForm, attributes(from_str, rename))]
pub fn ff_derive(input: TokenStream) -> TokenStream {
    let (_, name, data) = preamble(parse_macro_input!(input as DeriveInput));

    let fields = data.fields.into_iter();
    let len = fields.len();

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
        impl #name {
            pub const fn len() -> usize {
                return #len;
            }
        }

        impl TryFrom<std::collections::HashMap<String,String>> for #name {
            type Error = String;
            fn try_from(form_data: std::collections::HashMap<String,String>) -> Result<Self, Self::Error> {
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
