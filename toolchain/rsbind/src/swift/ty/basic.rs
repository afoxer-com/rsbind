use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::ident;
use crate::swift::mapping::{RustMapping, SwiftMapping};

pub(crate) struct Bool {}

impl<'a> Convertible<Swift<'a>> for Bool {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks_f!("{} ? Int32(1) : Int32(0)", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks_f!("{} > 0 ? true : false", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            if #origin {1} else {0}
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            if #origin > 0 {true} else {false}
        }
    }

    fn native_type(&self) -> Swift<'a> {
        swift::BOOLEAN
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}

pub(crate) struct Basic {
    pub(crate) ty: AstType,
}

impl Basic {
    fn native_type_str(&self) -> String {
        match self.ty.clone() {
            AstType::Void => "()",
            AstType::Byte(_) => "Int8",
            AstType::Short(_) => "Int16",
            AstType::Int(_) => "Int32",
            AstType::Long(_) => "Int64",
            AstType::Float(_) => "Float",
            AstType::Double(_) => "Double",
            _ => "",
        }
        .to_string()
    }
}

impl<'a> Convertible<Swift<'a>> for Basic {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks_f!("{}({})", self.native_type_str(), origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        toks_f!("{}({})", self.native_type_str(), origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let ty_ident = RustMapping::map_base_transfer_type(&self.ty);
        quote! {
            #origin as #ty_ident
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let origin_type_ident = ident!(&self.ty.origin());
        quote! {
            #origin as #origin_type_ident
        }
    }

    fn native_type(&self) -> Swift<'a> {
        swift::local(self.native_type_str())
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}

pub(crate) fn quote_free_swift_ptr(ty: &str) -> Tokens<'static, Swift<'static>> {
    let mut t = Tokens::new();
    push_f!(
        t,
        "let free_ptr : @convention(c) (UnsafeMutablePointer<{}>?, Int32) -> () = {{",
        ty
    );
    nested_f!(t, |tt| {
        nested_f!(tt, "(ptr, count) in");
        nested_f!(tt, "ptr?.deinitialize(count: Int(count))");
        nested_f!(tt, "ptr?.deallocate()");
    });
    push_f!(t, "}");
    t
}
