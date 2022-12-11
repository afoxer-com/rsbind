use crate::ast::types::AstType;
use crate::base::lang::{Convertible, Direction};
use proc_macro2::{Ident, TokenStream};
use rstgen::{js, JavaScript, Tokens};

pub(crate) struct Str {
    pub(crate) ty: AstType,
}

impl<'a> Convertible<JavaScript<'static>> for Str {
    fn native_to_transferable(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{}", origin)
    }

    fn transferable_to_native(
        &self,
        origin: String,
        direction: Direction,
    ) -> Tokens<'static, JavaScript<'static>> {
        toks_f!("{}", origin)
    }

    fn rust_to_transferable(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {{
            let mut j_result = ptr::null_mut();
            let cstr = std::ffi::CString::new(#origin).unwrap();
            // napi_create_string_utf8 don't need null terminated buffer.
            let bytes = cstr.as_bytes();
            let arg = bytes.as_ptr() as (*const std::os::raw::c_char);
            let len = bytes.len();
            let ok = unsafe {napi_sys::napi_create_string_utf8(env, arg, len, &mut j_result)};
            if ok == 0 {
                j_result
            } else {
                panic!("Can't create string for string.js.");
            }
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, direction: Direction) -> TokenStream {
        quote! {{
            let mut buf_size = 0;
            let mut buf = ptr::null_mut();
            // first get the buffer size.
            let ok = unsafe {napi_sys::napi_get_value_string_utf8(env, #origin, buf, 0, &mut buf_size)};
            if ok == 0 {
                let mut bytes = Vec::with_capacity(buf_size + 1);
                buf = bytes.as_mut_ptr();
                let ok = unsafe {napi_sys::napi_get_value_string_utf8(
                    env,
                    #origin,
                    buf,
                    buf_size + 1,
                    &mut buf_size,
                )};
                if ok == 0 {
                    let slice = unsafe {
                        std::slice::from_raw_parts(
                            buf as (*const u8),
                            buf_size + 1,
                        )
                        .to_vec()
                    };
                    let cstr = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(&slice) };
                    cstr.to_string_lossy().to_string()
                } else {
                    panic!("Can't get utf8 string size for string.");
                }
            } else {
                panic!("Can't get utf8 string for string.");
            }
        }}
    }

    fn native_type(&self) -> JavaScript<'static> {
        js::local("string")
    }

    fn native_transferable_type(&self, direction: Direction) -> JavaScript<'static> {
        js::local("string")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        quote! {napi_sys::napi_value}
    }

    fn quote_common_in_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_in_native(&self) -> Tokens<'static, JavaScript<'static>> {
        toks!()
    }

    fn quote_in_common_rs(&self) -> TokenStream {
        quote! {}
    }
}
