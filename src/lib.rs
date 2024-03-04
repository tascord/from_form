use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident};

fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("FromForm can only be implemented for structs"),
    };

    (input, name, data)
}

#[proc_macro_derive(FromForm, attributes(skip, rename))]
pub fn ff_derive(input: TokenStream) -> TokenStream {
    let (_, name, data) = preamble(parse_macro_input!(input as DeriveInput));
    let fields = data
        .fields
        .iter()
        .filter(|f| !f.attrs.iter().any(|a| a.path().is_ident("skip")))
        .map(|f| {
            // #[rename("Username")] <-- Honour rename field attr for form name
            // username: Username,   <-- Get field type

            let form_name = f
                .attrs
                .iter()
                .find(|a| a.path().is_ident("rename"))
                .and_then(|attr| attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value()))
                .unwrap_or_else(|| f.ident.clone().unwrap().to_string());

            let field_type = f.ty.clone();

            (f.ident.clone(), form_name, field_type)
        });

    let field_names = fields.clone().map(|(name, _, _)| name.unwrap());
    let form_names = fields.clone().map(|(_, name, _)| name);
    let field_types = fields.clone().map(|(_, _, ty)| ty);

    let implimentation = quote! {
        impl TryFrom<std::collections::HashMap<String,String>> for #name {
            type Error = String;
            fn try_from(form_data: std::collections::HashMap<String,String>) -> Result<Self, Self::Error> {
                Ok(
                    #name {
                        #(
                            #field_names: #field_types::try_from(
                                form_data.get(#form_names).ok_or_else(|| format!("{} not found", #form_names))?.to_string()
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                    }
                )
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

/// Skip a field from being parsed from the form
#[proc_macro_attribute]
pub fn skip(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
