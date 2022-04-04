use crate::ast::contract::desc::TraitDesc;
use crate::errors::*;
use proc_macro2::{Ident, Span, TokenStream};
use rstgen::swift;
use rstgen::swift::Swift;

use crate::ast::types::{AstBaseType, AstType};
use crate::ident;

pub(crate) struct SwiftMapping {}

impl<'a> SwiftMapping {
    /// Get the swift method signature argument types.
    pub(crate) fn map_swift_sig_type(ty: &'a AstType) -> Swift<'static> {
        match &ty {
            AstType::Void => swift::local("()"),
            AstType::Byte(_) => swift::BYTE,     // Int8
            AstType::Short(_) => swift::SHORT,   // Int16
            AstType::Int(_) => swift::INTEGER,   // Int32
            AstType::Long(_) => swift::LONG,     // Int64
            AstType::Float(_) => swift::FLOAT,   // Float
            AstType::Double(_) => swift::DOUBLE, // Double
            AstType::Boolean => swift::BOOLEAN,  // Bool
            AstType::String => swift::local("String"),
            AstType::Vec(base) => Swift::Array {
                inner: Box::new(SwiftMapping::map_swift_sig_type(&base.clone().into())),
            },
            AstType::Callback(origin) => swift::local(origin.clone()),
            AstType::Struct(origin) => swift::local(origin.clone()),
        }
    }

    /// Get the swift method signature argument types.
    pub(crate) fn map_swift_sig_type_str(ty: &'a AstType) -> String {
        let mut buffer_str = "".to_string();
        match &ty {
            AstType::Void => "()",
            AstType::Byte(_) => "Int8",
            AstType::Short(_) => "Int16",
            AstType::Int(_) => "Int32",
            AstType::Long(_) => "Int64",
            AstType::Float(_) => "Float",
            AstType::Double(_) => "Double",
            AstType::Boolean => "Bool", // Bool
            AstType::String => "String",
            AstType::Vec(AstBaseType::Byte(ref origin)) => "[Int8]",
            AstType::Vec(AstBaseType::Short(ref origin)) => "[Int16]",
            AstType::Vec(AstBaseType::Int(ref origin)) => "[Int32]",
            AstType::Vec(AstBaseType::Long(ref origin)) => "[Int64]",
            AstType::Vec(AstBaseType::Struct(ref origin)) => {
                buffer_str = format!("[{}]", origin);
                buffer_str.as_str()
            }
            AstType::Vec(ref base) => {
                let sub = SwiftMapping::map_swift_sig_type_str(&AstType::from(base.clone()));
                buffer_str = format!("[{}]", sub);
                buffer_str.as_str()
            }
            AstType::Callback(ref origin) => origin,
            AstType::Struct(ref origin) => origin,
        }
        .to_string()
    }

    /// Get the swift argument types for transferring to C.
    pub(crate) fn map_base_transfer_type(ty: &'a AstType) -> String {
        let mut buffer_str = "".to_string();
        match &ty {
            AstType::Void => "()",
            AstType::Byte(_) => "Int8",
            AstType::Short(_) => "Int16",
            AstType::Int(_) => "Int32",
            AstType::Long(_) => "Int64",
            AstType::Float(_) => "Float",
            AstType::Double(_) => "Double",
            AstType::Boolean => "Int32",
            AstType::String => "CInt8Array",
            AstType::Vec(AstBaseType::Byte(_)) => "CInt8Array",
            AstType::Vec(AstBaseType::Short(_)) => "CInt16Array",
            AstType::Vec(AstBaseType::Int(_)) => "CInt32Array",
            AstType::Vec(AstBaseType::Long(_)) => "CInt64Array",
            AstType::Vec(AstBaseType::Struct(ref origin)) => {
                buffer_str = format!("C{}Array", origin);
                buffer_str.as_ref()
            }
            AstType::Vec(_) => "CInt8Array",
            AstType::Callback(origin) => origin,
            AstType::Struct(origin) => {
                buffer_str = format!("Proxy{}", origin);
                buffer_str.as_str()
            }
        }
        .to_string()
    }

    pub(crate) fn map_transfer_type(ty: &AstType, callbacks: &[&TraitDesc]) -> String {
        match ty.clone() {
            AstType::Callback(origin) => {
                let mut callback_trait = None;
                for callback in callbacks.iter() {
                    if callback.name == origin.clone() {
                        callback_trait = Some(callback);
                        break;
                    }
                }
                format!(
                    "{}_{}_Model",
                    &callback_trait.unwrap().mod_name,
                    &callback_trait.unwrap().name
                )
            }
            _ => SwiftMapping::map_base_transfer_type(ty),
        }
    }
}

pub(crate) struct RustMapping {}

impl<'a> RustMapping {
    pub(crate) fn map_base_transfer_type(ty: &'a AstType) -> TokenStream {
        match &ty {
            AstType::Void => quote!(()),
            AstType::Byte(_) => quote!(i8),
            AstType::Short(_) => quote!(i16),
            AstType::Int(_) => quote!(i32),
            AstType::Long(_) => quote!(i64),
            AstType::Float(_) => quote!(f32),
            AstType::Double(_) => quote!(f64),
            AstType::Boolean => quote!(i32),
            AstType::String => quote!(CInt8Array),
            AstType::Vec(AstBaseType::Byte(_)) => quote!(CInt8Array),
            AstType::Vec(AstBaseType::Short(_)) => quote!(CInt16Array),
            AstType::Vec(AstBaseType::Int(_)) => quote!(CInt32Array),
            AstType::Vec(AstBaseType::Long(_)) => quote!(CInt64Array),
            AstType::Vec(AstBaseType::Struct(origin)) => {
                let struct_array_name = ident!(&format!("C{}Array", origin));
                quote!(#struct_array_name)
            }
            AstType::Vec(_) => quote!(CInt8Array),
            AstType::Callback(_) => quote!(()), // not expected to call here!
            AstType::Struct(ref origin) => {
                let struct_ident = ident!(&format!("Proxy{}", origin));
                quote!(#struct_ident)
            }
        }
    }

    pub(crate) fn map_transfer_type(ty: &AstType, callbacks: &[&TraitDesc]) -> TokenStream {
        match ty.clone() {
            AstType::Callback(origin) => {
                let mut callback_trait = None;
                for callback in callbacks.iter() {
                    if callback.name == origin.clone() {
                        callback_trait = Some(callback);
                        break;
                    }
                }
                let callback_str = &format!(
                    "{}_{}_Model",
                    &callback_trait.unwrap().mod_name,
                    &callback_trait.unwrap().name
                );
                let callback_ident = ident!(callback_str);
                quote!(#callback_ident)
            }

            _ => RustMapping::map_base_transfer_type(ty),
        }
    }
}
