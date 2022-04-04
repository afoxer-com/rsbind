use crate::base::Convertible;
use crate::swift::ty::basic::quote_free_swift_ptr;
use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::Tokens;

pub(crate) struct Str {}

impl<'a> Convertible<Swift<'a>> for Str {
    fn swift_to_transfer(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        push_f!(body, " { () -> CInt8Array in");
        nested_f!(body, |t| {
            push_f!(t, "var buffer: UnsafeMutablePointer<Int8>? = nil");
            push_f!(t, "var count : Int32 = 0");
            push_f!(t, "let cstr = {}.cString(using: .utf8)", origin);
            push_f!(t, "cstr?.withUnsafeBufferPointer({ bufferPointer in");
            nested_f!(t, |tt| {
                // We allocate a buffer in swift, need deallocate in swift too.
                push_f!(
                    tt,
                    "buffer = UnsafeMutablePointer<Int8>.allocate(capacity: bufferPointer.count)"
                );
                push_f!(
                tt,
                "buffer?.initialize(from: bufferPointer.baseAddress!, count: bufferPointer.count)"
            );
                push_f!(tt, "count = Int32(bufferPointer.count)");
            });
            push_f!(t, "})");
            push_f!(t, quote_free_swift_ptr("Int8"));
            push_f!(
                t,
                "return CInt8Array(ptr: buffer!, len: count, free_ptr: free_ptr)"
            );
        });
        push_f!(body, "}()");
        body
    }

    fn transfer_to_swift(&self, origin: String) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        push_f!(body, "{ () -> String in");
        nested_f!(body, "let str = String(cString: {}.ptr!)", origin);
        nested_f!(body, "print(\"begin free str from swift: transfer_to_swift\")");
        nested_f!(
            body,
            "({}.free_ptr)(UnsafeMutablePointer(mutating: {}.ptr!), Int32({}.len))",
            origin,
            origin,
            origin
        );
        nested_f!(body, "return str");
        push_f!(body, "}()");
        body
    }

    fn rust_to_transfer(&self, origin: TokenStream) -> TokenStream {
        quote! {
            {
                let cstr = CString::new(#origin).unwrap();
                let bytes = cstr.as_bytes_with_nul();
                let array = CInt8Array {
                    ptr: bytes.as_ptr() as (*const i8),
                    len: bytes.len() as i32,
                    free_ptr: free_str
                };
                std::mem::forget(cstr);
                array
            }
        }
    }

    fn transfer_to_rust(&self, origin: TokenStream) -> TokenStream {
        quote! {
            {
                let slice = unsafe {std::slice::from_raw_parts(#origin.ptr as (*const u8), #origin.len as usize).to_vec()};
                let cstr = unsafe {CStr::from_bytes_with_nul_unchecked(&slice)};
                println!("begin free str from rust");
                let str = cstr.to_string_lossy().to_string();
                (#origin.free_ptr)(#origin.ptr as (*mut i8), #origin.len);
                str
            }
        }
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
