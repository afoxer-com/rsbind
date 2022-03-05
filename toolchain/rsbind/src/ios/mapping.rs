use ast::types::{AstBaseType, AstType};
use rsgen::swift;
use rsgen::swift::Swift;
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::TokenStreamExt;

pub(crate) struct SwiftMapping {}

impl<'a> SwiftMapping {
    pub(crate) fn map_sig_type(ty: &'a AstType) -> Swift<'static> {
        match &ty {
            AstType::Void => swift::local("()"),
            AstType::Byte(_) => swift::BYTE,     // Int8
            AstType::Int(_) => swift::INTEGER,   // Int32
            AstType::Long(_) => swift::LONG,     // Int64
            AstType::Float(_) => swift::FLOAT,   // Float
            AstType::Double(_) => swift::DOUBLE, // Double
            AstType::Boolean => swift::BOOLEAN,  // Bool
            AstType::String => swift::local("String"),
            AstType::Vec(base) => Swift::Array {
                inner: Box::new(SwiftMapping::map_sig_type(&base.clone().into())),
            },
            AstType::Callback(origin) => swift::local(origin.clone()),
            AstType::Struct(origin) => swift::local(origin.clone()),
        }
    }

    pub(crate) fn map_cb_closure_sig_type(ty: &'a AstType) -> String {
        match &ty {
            AstType::Void => "()",
            AstType::Byte(_) => "Int8",
            AstType::Int(_) => "Int32",
            AstType::Long(_) => "Int64",
            AstType::Float(_) => "Float",
            AstType::Double(_) => "Double",
            AstType::Boolean => "Int32",
            AstType::String => "UnsafePointer<Int8>?",
            AstType::Vec(AstBaseType::Byte(_)) => "CInt8Array",
            AstType::Vec(_) => "UnsafePointer<Int8>?",
            AstType::Callback(origin) => origin,
            AstType::Struct(_) => "UnsafePointer<Int8>?"
        }.to_string()
    }
}

pub(crate) struct RustMapping {}

impl<'a> RustMapping {
    pub(crate) fn map_sig_arg_type(ty: &'a AstType) -> TokenStream {
        match &ty {
            AstType::Void => quote!(()),
            AstType::Byte(_) => quote!(i8),
            AstType::Int(_) => quote!(i32),
            AstType::Long(_) => quote!(i64),
            AstType::Float(_) => quote!(f32),
            AstType::Double(_) => quote!(f64),
            AstType::Boolean => quote!(i32),
            AstType::String => quote!(* const c_char),
            AstType::Vec(AstBaseType::Byte(_)) => quote!(CInt8Array),
            AstType::Vec(_) => quote!(* const c_char),
            AstType::Callback(_) => quote!(()),  // not expected to call here!
            AstType::Struct(_) => quote!(* const c_char)
        }
    }
    pub(crate) fn map_sig_return_type(ty: &'a AstType) -> TokenStream {
        match &ty {
            AstType::Void => quote!(()),
            AstType::Byte(_) => quote!(i8),
            AstType::Int(_) => quote!(i32),
            AstType::Long(_) => quote!(i64),
            AstType::Float(_) => quote!(f32),
            AstType::Double(_) => quote!(f64),
            AstType::Boolean => quote!(i32),
            AstType::String => quote!(* mut c_char),
            AstType::Vec(_) => quote!(* mut c_char),
            AstType::Callback(_) => quote!(()),  // not expected to call here!
            AstType::Struct(_) => quote!(* mut c_char)
        }
    }
}

