use proc_macro2::TokenStream;
use rstgen::{java, Java, Tokens};

use crate::ast::types::AstType;
use crate::base::{Convertible, Direction};
use crate::ident;

pub(crate) struct Bool {}

impl<'a> Convertible<Java<'a>> for Bool {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{} ? 1 : 0", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{} > 0 ? true: false", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {
            if #origin {1} else {0}
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => {
                quote! {
                    if #origin > 0 {true} else {false}
                }
            }
            Direction::Up => {
                quote! {
                    match #origin {
                        Ok(JValue::Int(value)) => if value > 0 {true} else {false},
                        _ => panic!("Wrong bool types.")
                    }
                }
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        java::BOOLEAN
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}

pub(crate) struct Basic {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<Java<'a>> for Basic {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{}", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        toks_f!("{}", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        let ty = basic_ty_to_tokens(self.ty.clone());
        quote! {
            #origin as #ty
        }
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        match direction {
            Direction::Down => match self.ty.clone() {
                AstType::Byte(ref base)
                | AstType::Int(ref base)
                | AstType::Short(ref base)
                | AstType::Long(ref base)
                | AstType::Float(ref base)
                | AstType::Double(ref base) => {
                    let origin_ident = ident!(base);
                    quote! {
                        #origin as #origin_ident
                    }
                }
                _ => {
                    quote! {}
                }
            },
            Direction::Up => {
                let ty = match self.ty.clone() {
                    AstType::Byte(_) => quote! {Byte},
                    AstType::Int(_) => quote! {Int},
                    AstType::Short(_) => quote! {Short},
                    AstType::Long(_) => quote! {Long},
                    AstType::Float(_) => quote! {Float},
                    AstType::Double(_) => quote! {Double},
                    _ => quote! {},
                };
                let origin_ident = match self.ty.clone() {
                    AstType::Byte(ref base)
                    | AstType::Int(ref base)
                    | AstType::Short(ref base)
                    | AstType::Long(ref base)
                    | AstType::Float(ref base)
                    | AstType::Double(ref base) => {
                        let base_ident = ident!(base);
                        quote! {#base_ident}
                    }
                    _ => quote! {},
                };
                quote! {
                    match #origin {
                        Ok(JValue::#ty(value)) => value as #origin_ident,
                        _ => panic!("value is not wrong type.")
                    }
                }
            }
        }
    }

    fn native_type(&self) -> Java<'a> {
        match self.ty.clone() {
            AstType::Byte(_) => java::BYTE,
            AstType::Int(_) => java::INTEGER,
            AstType::Short(_) => java::SHORT,
            AstType::Long(_) => java::LONG,
            AstType::Float(_) => java::FLOAT,
            AstType::Double(_) => java::DOUBLE,
            _ => java::local(""),
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        Tokens::new()
    }
}

pub(crate) fn basic_ty_to_tokens(ast_type: AstType) -> TokenStream {
    match ast_type {
        AstType::Byte(_) => quote!(i8),
        AstType::Short(_) => quote!(i16),
        AstType::Int(_) => quote!(i32),
        AstType::Long(_) => quote!(i64),
        AstType::Float(_) => quote!(f32),
        AstType::Double(_) => quote!(f64),
        AstType::Boolean => quote!(u8),
        _ => quote! {},
    }
}
