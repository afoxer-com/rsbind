use crate::ast::types::AstType;
use crate::base::Convertible;
use crate::ident;
use crate::swift::mapping::{RustMapping, SwiftMapping};
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) struct Bool {}

impl<'a> Convertible<Swift<'a>> for Bool {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        push_f!(body, "{} ? Int32(1) : Int32(0)", origin);
        body
    }

    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        nested_f!(body, "{} > 0 ? true : false", origin);
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream {
        quote! {
            if #origin {1} else {0}
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream {
        quote! {
            if #origin > 0 {true} else {false}
        }
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

impl<'a> Convertible<Swift<'a>> for Basic {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let ty = SwiftMapping::map_swift_sig_type_str(&self.ty);
        push_f!(body, "{}({})", ty, origin);
        body
    }

    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let ty = SwiftMapping::map_base_transfer_type(&self.ty);
        nested_f!(body, "{}({})", ty, origin);
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream {
        let ty_ident = RustMapping::map_base_transfer_type(&self.ty);
        quote! {
            #origin as #ty_ident
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream {
        let origin_type_ident = ident!(&self.ty.origin());
        quote! {
            #origin as #origin_type_ident
        }
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
