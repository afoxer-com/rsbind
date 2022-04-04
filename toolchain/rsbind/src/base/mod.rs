use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) mod process;

pub(crate) trait Convertible<T> {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, T>;
    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, T>;
    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream;
    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream;
    fn quote_common_bridge(&self) -> TokenStream;
    fn quote_common_artifact(&self) -> Tokens<'static, T>;
}
