use crate::ast::types::{AstBaseType, AstType};
use crate::base::Convertible;
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
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;
use syn::token::Token;

impl<'a> Convertible<Swift<'a>> for AstType {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self {
            AstType::Void => push_f!(body, origin),
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => {
                push!(body, Basic { ty: self.clone() }.swift_to_transfer(origin));
            }
            AstType::Boolean => {
                push!(body, Bool {}.swift_to_transfer(origin));
            }
            AstType::String => {
                push!(body, Str {}.swift_to_transfer(origin));
            }
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                push!(body, VecBase { ty: self.clone() }.swift_to_transfer(origin));
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        struct_name: base.to_string(),
                    }
                    .swift_to_transfer(origin.clone())
                );
            }
            AstType::Vec(_) => {
                push!(
                    body,
                    VecDefault { ty: self.clone() }.swift_to_transfer(origin)
                );
            }
            AstType::Callback(ref base) => {
                push!(
                    body,
                    Callback { ty: self.clone() }.swift_to_transfer(origin)
                );
            }
            AstType::Struct(_) => {
                push!(body, Struct { ty: self.clone() }.swift_to_transfer(origin));
            }
        }
        body
    }

    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self {
            AstType::Void => push_f!(body, origin),
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => {
                push!(body, Basic { ty: self.clone() }.transfer_to_swift(origin));
            }
            AstType::Boolean => {
                push!(body, Bool {}.transfer_to_swift(origin));
            }
            AstType::String => push!(body, Str {}.transfer_to_swift(origin)),
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                push!(body, VecBase { ty: self.clone() }.transfer_to_swift(origin));
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                push!(
                    body,
                    VecStruct {
                        struct_name: base.to_string(),
                    }
                    .transfer_to_swift(origin)
                );
            }
            AstType::Vec(_) => push!(
                body,
                VecDefault { ty: self.clone() }.transfer_to_swift(origin)
            ),

            AstType::Callback(ref base) => {
                push!(
                    body,
                    Callback { ty: self.clone() }.transfer_to_swift(origin)
                );
            }

            AstType::Struct(ref base) => {
                push!(body, Struct { ty: self.clone() }.transfer_to_swift(origin));
            }
        }
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream {
        match self {
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
            | AstType::Double(_) => Basic { ty: self.clone() }.rust_to_transfer(quote!(#origin)),
            AstType::Boolean => Bool {}.rust_to_transfer(quote!(#origin)),
            AstType::String => Str {}.rust_to_transfer(quote!(#origin)),
            AstType::Vec(AstBaseType::Struct(base)) => VecStruct {
                struct_name: base.to_string(),
            }
            .rust_to_transfer(quote!(#origin)),
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_)) => {
                VecBase { ty: self.clone() }.rust_to_transfer(quote!(#origin))
            }
            AstType::Vec(_) => VecDefault { ty: self.clone() }.rust_to_transfer(quote!(#origin)),
            AstType::Callback(_) => {
                Callback { ty: self.clone() }.rust_to_transfer(quote! {callback_index})
            }
            AstType::Struct(_) => Struct { ty: self.clone() }.rust_to_transfer(quote!(#origin)),
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream {
        match self {
            AstType::Void => {
                quote! {#origin}
            }
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => Basic { ty: self.clone() }.transfer_to_rust(quote!(#origin)),
            AstType::Boolean => Bool {}.transfer_to_rust(quote!(#origin)),
            AstType::String => Str {}.transfer_to_rust(quote!(#origin)),
            AstType::Vec(AstBaseType::Byte(ref base))
            | AstType::Vec(AstBaseType::Short(ref base))
            | AstType::Vec(AstBaseType::Int(ref base))
            | AstType::Vec(AstBaseType::Long(ref base)) => {
                VecBase { ty: self.clone() }.transfer_to_rust(quote!(#origin))
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => VecStruct {
                struct_name: base.to_string(),
            }
            .transfer_to_rust(quote!(#origin)),
            AstType::Vec(_) => VecDefault { ty: self.clone() }.transfer_to_rust(quote!(#origin)),
            AstType::Callback(_) => {
                Callback { ty: self.clone() }.transfer_to_rust(quote! {#origin})
            }
            AstType::Struct(_) => Struct { ty: self.clone() }.transfer_to_rust(quote! {#origin}),
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
