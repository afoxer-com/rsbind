use proc_macro2::TokenStream;
use rstgen::{Java, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::{Convertible, Direction};
use crate::java::ty::basic::{Basic, Bool};
use crate::java::ty::callback::Callback;
use crate::java::ty::str::Str;
use crate::java::ty::struct_::Struct;
use crate::java::ty::vec_byte::VecByte;
use crate::java::ty::vec_default::VecDefault;
use crate::java::ty::vec_struct::VecStruct;
use crate::java::ty::void::Void;

pub(crate) struct JavaConvert {
    pub(crate) ty: AstType,
}

pub(crate) enum ConvertEnum {
    Void(Void),
    Basic(Basic),
    Bool(Bool),
    Str(Str),
    Struct(Struct),
    VecByte(VecByte),
    VecDefault(VecDefault),
    VecStruct(VecStruct),
    Callback(Callback),
}

impl<'a> ConvertEnum {
    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<Java<'a>>) -> R,
    {
        match self {
            ConvertEnum::Void(c) => f(c),
            ConvertEnum::Basic(c) => f(c),
            ConvertEnum::Bool(c) => f(c),
            ConvertEnum::Str(c) => f(c),
            ConvertEnum::Struct(c) => f(c),
            ConvertEnum::VecByte(c) => f(c),
            ConvertEnum::VecDefault(c) => f(c),
            ConvertEnum::VecStruct(c) => f(c),
            ConvertEnum::Callback(c) => f(c),
        }
    }
}

impl<'a> JavaConvert {
    fn get_convert(&self, ty: &AstType) -> ConvertEnum {
        match ty.clone() {
            AstType::Void => ConvertEnum::Void(Void {}),
            AstType::Byte(_)
            | AstType::Int(_)
            | AstType::Short(_)
            | AstType::Long(_)
            | AstType::Float(_)
            | AstType::Double(_) => ConvertEnum::Basic(Basic { ty: ty.clone() }),
            AstType::Boolean => ConvertEnum::Bool(Bool {}),
            AstType::String => ConvertEnum::Str(Str {}),
            AstType::Vec(AstBaseType::Byte(_)) => ConvertEnum::VecByte(VecByte { ty: ty.clone() }),
            AstType::Vec(AstBaseType::Struct(_)) => {
                ConvertEnum::VecStruct(VecStruct { ty: ty.clone() })
            }
            AstType::Vec(_) => ConvertEnum::VecDefault(VecDefault { ty: ty.clone() }),
            AstType::Callback(_) => ConvertEnum::Callback(Callback { ty: ty.clone() }),
            AstType::Struct(_) => ConvertEnum::Struct(Struct { ty: ty.clone() }),
        }
    }

    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<Java<'a>>) -> R,
    {
        self.get_convert(&self.ty).handle(f)
    }
}

impl<'a> Convertible<Java<'a>> for JavaConvert {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        self.handle(|c| c.native_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Java<'a>> {
        self.handle(|c| c.transferable_to_native(origin.clone(), direction.clone()))
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.rust_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.transferable_to_rust(origin.clone(), direction.clone()))
    }

    fn native_type(&self) -> Java<'a> {
        self.handle(|c| c.native_type())
    }

    fn quote_common_bridge(&self) -> TokenStream {
        self.handle(|c| c.quote_common_bridge())
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Java<'a>> {
        self.handle(|c| c.quote_common_artifact())
    }
}
