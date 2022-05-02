use crate::errors::*;
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

pub(crate) trait Convertible<T> {
    fn native_to_transferable(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    fn transferable_to_native(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    fn native_type(&self) -> T;
    // fn native_transferable_type(&self, direction: Direction) -> String;
    // fn rust_type(&self) -> String;
    fn quote_common_bridge(&self) -> TokenStream;
    fn quote_common_artifact(&self) -> Tokens<'static, T>;
}
