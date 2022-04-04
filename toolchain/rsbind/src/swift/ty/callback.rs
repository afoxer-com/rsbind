use crate::ast::types::AstType;
use crate::base::Convertible;
use crate::ident;
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) struct Callback {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Swift<'a>> for Callback {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Callback(base) => {
                push_f!(
                    body,
                    "Internal{}.callbackToModel(callback: {})",
                    base,
                    origin
                );
            }
            _ => {}
        }
        body
    }

    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        match self.ty.clone() {
            AstType::Callback(base) => {
                nested_f!(body, "Internal{}.modelToCallback(model:{})", base, origin);
            }
            _ => {}
        }
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream {
        let box_to_model_fn_name = ident!(&format!("box_to_model_{}", self.ty.origin()));
        quote! {
            #box_to_model_fn_name(#origin)
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream {
        let model_to_box_fn = ident!(&format!("model_to_box_{}", self.ty.origin()));
        quote! {
             #model_to_box_fn(#origin)
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
