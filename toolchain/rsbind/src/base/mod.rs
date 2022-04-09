use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) mod process;

pub(crate) enum Direction {
    Invoke,
    Push,
}

pub(crate) trait Convertible<T> {
    fn artifact_to_transfer(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    fn transfer_to_artifact(&self, origin: String, direction: Direction) -> Tokens<'static, T>;
    fn rust_to_transfer(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    fn transfer_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream;
    fn quote_common_bridge(&self) -> TokenStream;
    fn quote_common_artifact(&self) -> Tokens<'static, T>;
}
