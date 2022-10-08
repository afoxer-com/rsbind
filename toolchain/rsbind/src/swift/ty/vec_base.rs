use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::ident;
use crate::swift::ty::basic::quote_free_swift_ptr;

pub(crate) struct VecBase {
    pub(crate) ty: AstType,
}

impl VecBase {
    fn native_base_type_str(&self) -> String {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(_)) => "Int8",
            AstType::Vec(AstBaseType::Short(_)) => "Int16",
            AstType::Vec(AstBaseType::Int(_)) => "Int32",
            AstType::Vec(AstBaseType::Long(_)) => "Int64",
            AstType::Vec(AstBaseType::Float(_)) => "Float32",
            AstType::Vec(AstBaseType::Double(_)) => "Float64",
            _ => panic!("Error type found."),
        }
        .to_string()
    }

    fn rust_base_transfer_type(&self) -> TokenStream {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(_)) => quote!(i8),
            AstType::Vec(AstBaseType::Short(_)) => quote!(i16),
            AstType::Vec(AstBaseType::Int(_)) => quote!(i32),
            AstType::Vec(AstBaseType::Long(_)) => quote!(i64),
            AstType::Vec(AstBaseType::Float(_)) => quote!(f32),
            AstType::Vec(AstBaseType::Double(_)) => quote!(f64),
            _ => panic!("Error type found."),
        }
    }
}

impl<'a> Convertible<Swift<'a>> for VecBase {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let transfer_ty = self.native_transferable_type(direction);
        let base_ty = self.native_base_type_str();
        body.append(toks!("{ () -> ", transfer_ty.clone(), " in"));
        nested_f!(
            body,
            "let tmp_ptr = UnsafeMutablePointer<{}>.allocate(capacity: {}.count)",
            base_ty,
            origin
        );
        nested_f!(body, "{}.withUnsafeBufferPointer {{ buffer in", origin);
        nested_f!(body, |t| {
            nested_f!(
                t,
                "tmp_ptr.initialize(from: buffer.baseAddress!, count: buffer.count)"
            )
        });
        nested_f!(body, "}");
        nested_f!(body, quote_free_swift_ptr(&base_ty));
        nested!(
            body,
            "return ",
            transfer_ty.clone(),
            "(ptr: tmp_ptr, len: Int32(",
            origin,
            ".count), cap: Int32(",
            origin,
            ".capacity), free_ptr: free_ptr)",
        );
        push_f!(body, "}()");
        body
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let ty = format!("[{}]", self.native_base_type_str());
        body.append(toks_f!("{{ () -> {} in", ty));
        nested_f!(body, |t| {
            nested_f!(
                t,
                "let array = {}(UnsafeBufferPointer(start: {}.ptr, count: Int({}.len)))",
                ty,
                origin,
                origin
            );
            nested_f!(
                t,
                "({}.free_ptr)(UnsafeMutablePointer(mutating: {}.ptr), {}.len, {}.cap)",
                origin,
                origin,
                origin,
                origin
            );
            nested_f!(t, "return array");
        });
        nested_f!(body, "}()");
        body
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let base_ty = self.rust_base_transfer_type();
        let c_array_ty = self.rust_transferable_type(Direction::Down);
        let free_ptr = match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref _base)) => {
                ident!("free_i8_array")
            }
            AstType::Vec(AstBaseType::Short(ref _base)) => {
                ident!("free_i16_array")
            }
            AstType::Vec(AstBaseType::Int(ref _base)) => {
                ident!("free_i32_array")
            }
            AstType::Vec(AstBaseType::Long(ref _base)) => {
                ident!("free_i64_array")
            }
            AstType::Vec(AstBaseType::Float(ref base)) => {
                ident!("free_f32_array")
            }
            AstType::Vec(AstBaseType::Double(ref base)) => {
                ident!("free_f64_array")
            }
            _ => {
                ident!("free_i8_array")
            }
        };

        quote! {{
            let mut copy = #origin.clone();
            let ptr_name = copy.as_ptr();
            let len_name = copy.len();
            let cap_name = copy.capacity();
            let array = #c_array_ty {
                ptr: ptr_name as (*const #base_ty),
                len: len_name as i32,
                cap: cap_name as i32,
                free_ptr: #free_ptr
            };
            std::mem::forget(copy);
            array
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let transfer_ty = self.rust_base_transfer_type();
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref base))
            | AstType::Vec(AstBaseType::Short(ref base))
            | AstType::Vec(AstBaseType::Int(ref base))
            | AstType::Vec(AstBaseType::Long(ref base))
            | AstType::Vec(AstBaseType::Float(ref base))
            | AstType::Vec(AstBaseType::Double(ref base)) => {
                let origin_ident = ident!(base);
                quote! {{
                    let vec = unsafe { std::slice::from_raw_parts(#origin.ptr as (* mut #origin_ident), #origin.len as usize).to_vec() };
                    (#origin.free_ptr)(#origin.ptr as (*mut #transfer_ty), #origin.len, #origin.cap);
                    vec
                }}
            }
            _ => quote! {},
        }
    }

    fn native_type(&self) -> Swift<'a> {
        swift::local(format!("[{}]", self.native_base_type_str()))
    }

    fn native_transferable_type(&self, direction: Direction) -> Swift<'a> {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(_)) => swift::local("CInt8Array"),
            AstType::Vec(AstBaseType::Short(_)) => swift::local("CInt16Array"),
            AstType::Vec(AstBaseType::Int(_)) => swift::local("CInt32Array"),
            AstType::Vec(AstBaseType::Long(_)) => swift::local("CInt64Array"),
            AstType::Vec(AstBaseType::Float(_)) => swift::local("CFloat32Array"),
            AstType::Vec(AstBaseType::Double(_)) => swift::local("CFloat64Array"),
            _ => swift::local(""),
        }
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(_)) => quote!(CInt8Array),
            AstType::Vec(AstBaseType::Short(_)) => quote!(CInt16Array),
            AstType::Vec(AstBaseType::Int(_)) => quote!(CInt32Array),
            AstType::Vec(AstBaseType::Long(_)) => quote!(CInt64Array),
            AstType::Vec(AstBaseType::Float(_)) => quote!(CFloat32Array),
            AstType::Vec(AstBaseType::Double(_)) => quote!(CFloat64Array),
            _ => quote! {},
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
