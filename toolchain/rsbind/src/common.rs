use proc_macro2::{Ident, Span};

#[macro_export]
macro_rules! ident {
    ($name:expr) => {
        proc_macro2::Ident::new($name, proc_macro2::Span::call_site())
    };
}
