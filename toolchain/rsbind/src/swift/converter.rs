use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;
use syn::token::Token;

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::ident;
use crate::swift::mapping::SwiftMapping;
use crate::swift::ty::basic::{Basic, Bool};
use crate::swift::ty::callback::Callback;
use crate::swift::ty::str::Str;
use crate::swift::ty::struct_::Struct;
use crate::swift::ty::vec_base::VecBase;
use crate::swift::ty::vec_default::VecDefault;
use crate::swift::ty::vec_struct::VecStruct;
use crate::swift::types::SwiftType;

pub(crate) struct SwiftConvert {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for SwiftConvert {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Void => push_f!(body, origin),
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => {
                push!(
                    body,
                    Basic {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Boolean => {
                push!(body, Bool {}.artifact_to_transfer(origin, direction));
            }
            AstType::String => {
                push!(body, Str {}.artifact_to_transfer(origin, direction));
            }
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                push!(
                    body,
                    VecBase {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        struct_name: base.to_string(),
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Vec(_) => {
                push!(
                    body,
                    VecDefault {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Callback(ref base) => {
                push!(
                    body,
                    Callback {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Struct(_) => {
                push!(
                    body,
                    Struct {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
        }
        body
    }

    fn transfer_to_artifact(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Void => push_f!(body, origin),
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => {
                push!(
                    body,
                    Basic {
                        ty: self.ty.clone()
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }
            AstType::Boolean => {
                push!(body, Bool {}.transfer_to_artifact(origin, direction));
            }
            AstType::String => push!(body, Str {}.transfer_to_artifact(origin, direction)),
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                push!(
                    body,
                    VecBase {
                        ty: self.ty.clone()
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        struct_name: base.to_string(),
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }
            AstType::Vec(_) => push!(
                body,
                VecDefault {
                    ty: self.ty.clone()
                }
                .transfer_to_artifact(origin, direction)
            ),

            AstType::Callback(ref base) => {
                push!(
                    body,
                    Callback {
                        ty: self.ty.clone()
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }

            AstType::Struct(ref base) => {
                push!(
                    body,
                    Struct {
                        ty: self.ty.clone()
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }
        }
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Void => {
                quote! {
                    #origin;
                }
            }
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => Basic {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote!(#origin), direction),
            AstType::Boolean => Bool {}.rust_to_transfer(quote!(#origin), direction),
            AstType::String => Str {}.rust_to_transfer(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Struct(base)) => VecStruct {
                struct_name: base.to_string(),
            }
            .rust_to_transfer(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => VecBase {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote!(#origin), direction),
            AstType::Vec(_) => VecDefault {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote!(#origin), direction),
            AstType::Callback(_) => Callback {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote! {callback_index}, direction),
            AstType::Struct(_) => Struct {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote!(#origin), direction),
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match self.ty.clone() {
            AstType::Void => {
                quote! {#origin}
            }
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => Basic {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote!(#origin), direction),
            AstType::Boolean => Bool {}.transfer_to_rust(quote!(#origin), direction),
            AstType::String => Str {}.transfer_to_rust(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Byte(ref base))
            | AstType::Vec(AstBaseType::Short(ref base))
            | AstType::Vec(AstBaseType::Int(ref base))
            | AstType::Vec(AstBaseType::Long(ref base)) => VecBase {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Struct(ref base)) => VecStruct {
                struct_name: base.to_string(),
            }
            .transfer_to_rust(quote!(#origin), direction),
            AstType::Vec(_) => VecDefault {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote!(#origin), direction),
            AstType::Callback(_) => Callback {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote! {#origin}, direction),
            AstType::Struct(_) => Struct {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote! {#origin}, direction),
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
