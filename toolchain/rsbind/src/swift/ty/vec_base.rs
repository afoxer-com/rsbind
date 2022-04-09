use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::ident;
use crate::swift::mapping::{RustMapping, SwiftMapping};
use crate::swift::ty::basic::quote_free_swift_ptr;

pub(crate) struct VecBase {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for VecBase {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let transfer_ty = SwiftMapping::map_base_transfer_type(&self.ty);
        let base_ty = match self.ty.clone() {
            AstType::Vec(base) => {
                SwiftMapping::map_base_transfer_type(&AstType::from(base.clone()))
            }
            _ => "".to_string(),
        };
        push_f!(body, "{{ () -> {} in", transfer_ty);
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
        nested_f!(
            body,
            "return {}(ptr: tmp_ptr, len: Int32({}.count), free_ptr: free_ptr)",
            transfer_ty,
            origin
        );
        push_f!(body, "}()");
        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let ty = SwiftMapping::map_swift_sig_type_str(&self.ty);
        nested_f!(body, "{{ () -> {} in", ty);
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
                "({}.free_ptr)(UnsafeMutablePointer(mutating: {}.ptr), {}.len)",
                origin,
                origin,
                origin
            );
            nested_f!(t, "return array");
        });
        nested_f!(body, "}()");
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let base_ty = match self.ty.clone() {
            AstType::Vec(base) => RustMapping::map_base_transfer_type(&AstType::from(base.clone())),
            _ => quote!(),
        };
        let c_array_ty = RustMapping::map_base_transfer_type(&self.ty);
        let free_ptr = match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref base)) => {
                ident!("free_i8_array")
            }
            AstType::Vec(AstBaseType::Short(ref base)) => {
                ident!("free_i16_array")
            }
            AstType::Vec(AstBaseType::Int(ref base)) => {
                ident!("free_i32_array")
            }
            AstType::Vec(AstBaseType::Long(ref base)) => {
                ident!("free_i64_array")
            }
            _ => {
                ident!("free_i8_array")
            }
        };

        quote! {
            {
                let mut copy = #origin.clone();
                copy.shrink_to_fit();
                let ptr_name = copy.as_ptr();
                let len_name = copy.len();
                let array = #c_array_ty {
                    ptr: ptr_name as (*const #base_ty),
                    len: len_name as i32,
                    free_ptr: #free_ptr
                };
                std::mem::forget(copy);
                array
            }
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        let transfer_ty = match self.ty.clone() {
            AstType::Vec(base) => RustMapping::map_base_transfer_type(&AstType::from(base.clone())),
            _ => quote! {},
        };
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Byte(ref base))
            | AstType::Vec(AstBaseType::Short(ref base))
            | AstType::Vec(AstBaseType::Int(ref base))
            | AstType::Vec(AstBaseType::Long(ref base)) => {
                let origin_ident = ident!(base);
                quote! {
                    {
                        let vec = unsafe { std::slice::from_raw_parts(#origin.ptr as (* mut #origin_ident), #origin.len as usize).to_vec() };
                        (#origin.free_ptr)(#origin.ptr as (*mut #transfer_ty), #origin.len);
                        vec
                    }
                }
            }
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
