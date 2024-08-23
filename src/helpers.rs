use syn::{Data, DataStruct, DeriveInput, Field, Ident, Type};

pub fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("FromForm can only be implemented for structs"),
    };

    (input, name, data)
}

pub fn get_inner_type(t: Type) -> Type {
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

pub fn filter_opts(
    f: impl Iterator<Item = Field>,
    use_opts: bool,
    use_str: bool,
) -> impl Iterator<Item = Field> {
    f.filter(move |f| match f.ty.clone() {
        Type::Path(p) => match p.path.segments.first().map(|p| p.ident.clone()) {
            Some(p) => match use_opts {
                true => p.to_string() == "Option".to_string(),
                false => p.to_string() != "Option".to_string(),
            },
            None => !use_opts,
        },
        _ => !use_opts,
    })
    .filter(
        move |f| match f.attrs.iter().find(|a| a.path().is_ident("from_str")).is_some() {
            true => use_str,
            false => !use_str,
        },
    )
}

pub fn collect(f: impl Iterator<Item = Field>) -> (Vec<Option<Ident>>, Vec<String>, Vec<Type>) {
    let f = f.map(|f| {
        let form_name = f
            .attrs
            .iter()
            .find(|a| a.path().is_ident("rename"))
            .and_then(|attr| attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value()))
            .unwrap_or_else(|| f.ident.clone().unwrap().to_string());

        let field_type = f.ty.clone();

        (f.ident.clone(), (form_name, field_type))
    });

    let (ident, r): (Vec<_>, (Vec<_>, Vec<_>)) = f.unzip();
    let (name, ty) = r;

    (ident, name, ty)
}
