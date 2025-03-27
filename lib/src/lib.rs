pub use ff_macro::*;
pub trait FromForm: TryFrom<std::collections::HashMap<String, String>> {
    const COLUMNS: &'static [&'static str];
}
