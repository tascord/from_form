use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident, Type};

fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("FromForm can only be implemented for structs"),
    };

    (input, name, data)
}

fn get_inner_type(t: Type) -> Type {
    match t {
        Type::Path(p) => {
            let ty = match p.path.segments.first().unwrap().arguments.clone() {
                syn::PathArguments::AngleBracketed(t) => t,
                _ => panic!("Unexpected path arguments"),
            }
            .args;

            let ty = match ty.first().unwrap() {
                syn::GenericArgument::Type(t) => t,
                _ => panic!("Unexpected non-type generic argument"),
            };

            ty.clone()
        }
        _ => panic!("Cant get inner type of non-path"),
    }
}

#[proc_macro_derive(FromForm, attributes(skip, rename))]
pub fn ff_derive(input: TokenStream) -> TokenStream {
    let (_, name, data) = preamble(parse_macro_input!(input as DeriveInput));

    let fields = data
        .fields
        .iter()
        .filter(|f| !f.attrs.iter().any(|a| a.path().is_ident("skip")));

    let standard_fields = fields
        .clone()
        .filter(|f| match &f.ty {
            Type::Path(p) => match p.path.segments.first().map(|p| p.ident.clone()) {
                Some(p) => p.to_string() != "Option".to_string(),
                None => true,
            },
            _ => true,
        })
        .map(|f| {
            let form_name = f
                .attrs
                .iter()
                .find(|a| a.path().is_ident("rename"))
                .and_then(|attr| attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value()))
                .unwrap_or_else(|| f.ident.clone().unwrap().to_string());

            let field_type = f.ty.clone();

            (f.ident.clone(), form_name, field_type)
        })
        .collect::<Vec<_>>();

    let optional_fields = fields
        .clone()
        .filter(|f| match &f.ty {
            Type::Path(p) => match p.path.segments.first().map(|p| p.ident.clone()) {
                Some(p) => p.to_string() == "Option".to_string(),
                None => false,
            },
            _ => false,
        })
        .map(|f| {
            let form_name = f
                .attrs
                .iter()
                .find(|a| a.path().is_ident("rename"))
                .and_then(|attr| attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value()))
                .unwrap_or_else(|| f.ident.clone().unwrap().to_string());

            let field_type = get_inner_type(f.ty.clone());

            (f.ident.clone(), form_name, field_type)
        })
        .collect::<Vec<_>>();

    let std_field = standard_fields
        .iter()
        .map(|(name, _, _)| name.clone().unwrap());
    let std_name = standard_fields.iter().map(|(_, name, _)| name);
    let std_type = standard_fields.iter().map(|(_, _, ty)| ty);

    let opt_field = optional_fields
        .iter()
        .map(|(name, _, _)| name.clone().unwrap());
    let opt_name = optional_fields.iter().map(|(_, name, _)| name);
    let opt_type = optional_fields.iter().map(|(_, _, ty)| ty);

    let implimentation = quote! {
        impl TryFrom<std::collections::HashMap<String,String>> for #name {
            type Error = String;
            fn try_from(form_data: std::collections::HashMap<String,String>) -> Result<Self, Self::Error> {
                Ok(
                    #name {
                        #(
                            #std_field: <#std_type>::try_from(
                                form_data.get(#std_name).ok_or_else(|| format!("{} not found", #std_name))?.to_string()
                            ).map_err(|e| format!("{:?}", e))?,
                        )*
                        #(
                            #opt_field: match form_data.get(#opt_name).ok_or_else(|| format!("{} not found", #opt_name))?.to_string().is_empty() {
                                false => Some(<#opt_type>::try_from(form_data.get(#opt_name).unwrap().to_string()).map_err(|e| format!("{:?}", e))?),
                                true => None,
                            },
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
