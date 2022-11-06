use crate::ast::types::AstType;
use crate::ast::AstResult;
use crate::base::lang::{Convertible, Direction};
use crate::js::ty::array::Array;
use crate::js::ty::big_int::BigInt;
use crate::js::ty::boolean::Boolean;
use crate::js::ty::callback::Callback;
use crate::js::ty::number::Number;
use crate::js::ty::string::Str;
use crate::js::ty::struct_::Struct;
use proc_macro2::TokenStream;
use rstgen::{JavaScript, Tokens};

pub(crate) struct JsConverter {
    pub(crate) ty: AstType,
    pub(crate) ast: AstResult,
}

pub(crate) enum ConvertEnum {
    Number(Number),
    BigInt(BigInt),
    Boolean(Boolean),
    Str(Str),
    Array(Array),
    Struct(Struct),
    Callback(Callback),
}

impl<'a> ConvertEnum {
    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<JavaScript<'static>>) -> R,
    {
        match self {
            ConvertEnum::Number(c) => f(c),
            ConvertEnum::BigInt(c) => f(c),
            ConvertEnum::Boolean(c) => f(c),
            ConvertEnum::Str(c) => f(c),
            ConvertEnum::Array(c) => f(c),
            ConvertEnum::Struct(c) => f(c),
            ConvertEnum::Callback(c) => f(c),
        }
    }
}

impl<'a> JsConverter {
    fn get_convert(&self, ty: &AstType, ast: &AstResult) -> ConvertEnum {
        match ty.clone() {
            AstType::Byte(ref base)
            | AstType::Short(ref base)
            | AstType::Int(ref base)
            | AstType::Long(ref base)
            | AstType::Float(ref base)
            | AstType::Double(ref base) => ConvertEnum::Number(Number { ty: ty.clone() }),
            AstType::Boolean => ConvertEnum::Boolean(Boolean { ty: ty.clone() }),
            AstType::String => ConvertEnum::Str(Str { ty: ty.clone() }),
            AstType::Vec(ref base) => ConvertEnum::Array(Array {
                ty: ty.clone(),
                ast: ast.clone(),
            }),
            AstType::Struct(ref base) => ConvertEnum::Struct(Struct {
                ty: ty.clone(),
                ast: ast.clone(),
            }),
            AstType::Callback(ref base) => ConvertEnum::Callback(Callback {
                ty: ty.clone(),
                ast: ast.clone(),
            }),
            _ => ConvertEnum::Number(Number { ty: ty.clone() }),
        }
    }

    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<JavaScript<'static>>) -> R,
    {
        self.get_convert(&self.ty, &self.ast).handle(f)
    }
}

impl<'a> Convertible<JavaScript<'a>> for JsConverter {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'a>> {
        self.handle(|c| c.native_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'a>> {
        self.handle(|c| c.transferable_to_native(origin.clone(), direction.clone()))
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.rust_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.transferable_to_rust(origin.clone(), direction.clone()))
    }

    fn native_type(&self) -> JavaScript<'a> {
        self.handle(|c| c.native_type())
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'a> {
        self.handle(|c| c.native_transferable_type(direction.clone()))
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        self.handle(|c| c.rust_transferable_type(direction.clone()))
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        self.handle(|c| c.quote_common_in_bridge())
    }

    fn quote_common_in_native(&self) -> Tokens<'static, JavaScript<'a>> {
        self.handle(|c| c.quote_common_in_native())
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        self.handle(|c| c.quote_in_common_rs())
    }
}
