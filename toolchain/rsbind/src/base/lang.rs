use crate::ast::contract::desc::{ArgDesc, MethodDesc, StructDesc, TraitDesc};
use crate::ast::imp::desc::ImpDesc;
use crate::ast::types::AstType;
use crate::errors::*;
use crate::AstResult;
use proc_macro2::TokenStream;
use rstgen::Tokens;
use std::path::Path;

pub(crate) trait LangGen {
    fn gen_bridge(&self, path: &Path) -> Result<()>;
    fn gen_native(&self, path: &Path) -> Result<()>;
}

///Function directions of data transferring.
#[derive(Clone)]
pub(crate) enum Direction {
    /// Direction from native language type to rust type.
    /// If we call a function from native to rust, it's direction is Down.
    Down,
    /// Direction from rust type to native language type.
    /// If we call a function from rust to native, it's direction is Up.
    Up,
}

///Type that can be transferred between Rust and native lang.
pub(crate) trait Convertible<T> {
    /// Change data from native lang to transferable type.
    fn native_to_transferable(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    /// Change data from transferable type to native lang type.
    fn transferable_to_native(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    /// Change data from rust type to transferable type.
    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    /// Change data from transferable type to rust type.
    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    /// Native lang types.
    fn native_type(&self) -> T;
    /// Native transferable type.
    fn native_transferable_type(&self, direction: Direction) -> T;
    /// Rust transferable type.
    fn rust_transferable_type(&self, direction: Direction) -> TokenStream;
    // fn rust_type(&self) -> String;
    /// Quote code in bridge file.
    fn quote_common_in_bridge(&self) -> TokenStream;
    /// Quote code in native artifact bridge file.
    fn quote_common_in_native(&self) -> Tokens<'static, T>;
    /// Quote code in common.rs.
    fn quote_in_common_rs(&self) -> TokenStream;
}

// pub(crate) struct TraitContext<'a, Extra> {
//     pub trait_: &'a TraitDesc,
//
// }

pub(crate) struct ArgumentContext<'a, Lang, Extra> {
    pub arg: &'a ArgDesc,
    pub method_ctx: &'a MethodContext<'a, Lang, Extra>,
}

pub(crate) struct MethodContext<'a, Lang, Extra> {
    pub method: &'a MethodDesc,
    pub service_ctx: &'a ServiceContext<'a, Lang, Extra>,
}

pub(crate) struct ServiceContext<'a, Lang, Extra> {
    pub trait_: &'a TraitDesc,
    pub imp: &'a ImpDesc,
    pub mod_ctx: &'a ModContext<'a, Lang, Extra>,
}

pub(crate) struct CallbackContext<'a, Lang, Extra> {
    pub callback: &'a TraitDesc,
    pub mod_ctx: &'a ModContext<'a, Lang, Extra>,
}

pub(crate) struct StructContext<'a, Lang, Extra> {
    pub struct_: &'a StructDesc,
    pub mod_ctx: &'a ModContext<'a, Lang, Extra>,
}

pub(crate) struct ModContext<'a, Lang, Extra> {
    pub traits: &'a Vec<TraitDesc>,
    pub structs: &'a Vec<StructDesc>,
    pub imps: &'a Vec<ImpDesc>,
    pub mod_name: String,
    pub bridge_ctx: &'a BridgeContext<'a, Lang, Extra>,
}

pub(crate) struct BridgeContext<'a, Lang, Extra> {
    pub ast: &'a AstResult,
    pub crate_name: String,
    pub extra: &'a Extra,
    pub lang_imp: &'a Box<dyn LangImp<Lang, Extra>>,
    pub lang_name: String,
}

pub(crate) trait LangImp<Lang, Extra> {
    fn quote_sdk_file(&self, context: &BridgeContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_common_file(&self, context: &BridgeContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_use_part(&self, context: &ModContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_common_part(&self, context: &ModContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_method_sig(&self, context: &MethodContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_for_one_struct(&self, context: &StructContext<Lang, Extra>) -> Result<TokenStream>;
    fn quote_for_one_callback(&self, context: &CallbackContext<Lang, Extra>)
        -> Result<TokenStream>;
    fn provide_converter(&self, ty: &AstType) -> Box<dyn Convertible<Lang>>;
}
