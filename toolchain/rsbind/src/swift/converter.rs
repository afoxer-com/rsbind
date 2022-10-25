use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::swift::ty::basic::{Basic, Bool};
use crate::swift::ty::callback::Callback;
use crate::swift::ty::str::Str;
use crate::swift::ty::struct_::Struct;
use crate::swift::ty::vec_base::VecBase;
use crate::swift::ty::vec_default::VecDefault;
use crate::swift::ty::vec_struct::VecStruct;
use crate::swift::ty::void::Void;

pub(crate) enum ConvertEnum {
    Void(Void),
    Basic(Basic),
    Bool(Bool),
    Str(Str),
    Struct(Struct),
    VecBase(VecBase),
    VecDefault(VecDefault),
    VecStruct(VecStruct),
    Callback(Callback),
}

impl<'a> ConvertEnum {
    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<Swift<'a>>) -> R,
    {
        match self {
            ConvertEnum::Void(c) => f(c),
            ConvertEnum::Basic(c) => f(c),
            ConvertEnum::Bool(c) => f(c),
            ConvertEnum::Str(c) => f(c),
            ConvertEnum::Struct(c) => f(c),
            ConvertEnum::VecBase(c) => f(c),
            ConvertEnum::VecDefault(c) => f(c),
            ConvertEnum::VecStruct(c) => f(c),
            ConvertEnum::Callback(c) => f(c),
        }
    }
}

pub(crate) struct SwiftConvert {
    pub(crate) ty: AstType,
}

impl<'a> SwiftConvert {
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
            AstType::Vec(AstBaseType::Byte(_))
            | AstType::Vec(AstBaseType::Short(_))
            | AstType::Vec(AstBaseType::Int(_))
            | AstType::Vec(AstBaseType::Long(_))
            | AstType::Vec(AstBaseType::Float(_))
            | AstType::Vec(AstBaseType::Double(_)) => {
                ConvertEnum::VecBase(VecBase { ty: ty.clone() })
            }
            AstType::Vec(AstBaseType::Struct(ref base)) => {
                ConvertEnum::VecStruct(VecStruct { ty: ty.clone() })
            }
            AstType::Vec(_) => ConvertEnum::VecDefault(VecDefault { ty: ty.clone() }),
            AstType::Callback(_) => ConvertEnum::Callback(Callback { ty: ty.clone() }),
            AstType::Struct(_) => ConvertEnum::Struct(Struct { ty: ty.clone() }),
        }
    }

    fn handle<R, F>(&self, f: F) -> R
    where
        F: Fn(&dyn Convertible<Swift<'a>>) -> R,
    {
        self.get_convert(&self.ty).handle(f)
    }
}

impl<'a> Convertible<Swift<'a>> for SwiftConvert {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        self.handle(|c| c.native_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        self.handle(|c| c.transferable_to_native(origin.clone(), direction.clone()))
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.rust_to_transferable(origin.clone(), direction.clone()))
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        self.handle(|c| c.transferable_to_rust(origin.clone(), direction.clone()))
    }

    fn native_type(&self) -> Swift<'a> {
        self.handle(|c| c.native_type())
    }

    fn native_transferable_type(&self, direction: Direction) -> Swift<'a> {
        self.handle(|c| c.native_transferable_type(direction.clone()))
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        self.handle(|c| c.rust_transferable_type(direction.clone()))
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        self.handle(|c| c.quote_common_in_bridge())
    }

    fn quote_common_in_native(&self) -> Tokens<'static, Swift<'a>> {
        self.handle(|c| c.quote_common_in_native())
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        self.handle(|c| c.quote_in_common_rs())
    }
}
