use proc_macro2::{Ident, Span};

#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        Ident::new($name, Span::call_site())
    };
}
