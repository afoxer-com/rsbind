use proc_macro2::TokenStream;
use rstgen::{Java, Tokens};
use syn::token::Token;

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::ident;
use crate::java::ty::basic::{Basic, Bool};
use crate::java::ty::callback::Callback;
use crate::java::ty::str::Str;
use crate::java::ty::struct_::Struct;
use crate::java::ty::vec_byte::VecByte;
use crate::java::ty::vec_default::VecDefault;
use crate::java::ty::vec_struct::VecStruct;

pub(crate) struct JavaConvert {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for JavaConvert {
    fn artifact_to_transfer(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Void => {}
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
            AstType::Vec(AstBaseType::Byte(_)) => {
                push!(
                    body,
                    VecByte {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin, direction)
                );
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        ty: self.ty.clone()
                    }
                    .artifact_to_transfer(origin.clone(), direction)
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
    ) -> Tokens<'static, Java<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Void => {}
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
            AstType::Vec(AstBaseType::Byte(_)) => {
                push!(
                    body,
                    VecByte {
                        ty: self.ty.clone()
                    }
                    .transfer_to_artifact(origin, direction)
                );
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        ty: self.ty.clone()
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
            AstType::Vec(AstBaseType::Struct(_base)) => VecStruct {
                ty: self.ty.clone(),
            }
            .rust_to_transfer(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Byte(_)) => VecByte {
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
            .rust_to_transfer(quote! {#origin}, direction),
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
            AstType::Vec(AstBaseType::Byte(ref base)) => VecByte {
                ty: self.ty.clone(),
            }
            .transfer_to_rust(quote!(#origin), direction),
            AstType::Vec(AstBaseType::Struct(ref base)) => VecStruct {
                ty: self.ty.clone(),
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

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}
