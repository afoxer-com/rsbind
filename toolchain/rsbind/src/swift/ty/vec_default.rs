use proc_macro2::TokenStream;
use rstgen::swift::Swift;
use rstgen::{swift, Tokens};

use crate::ast::types::{AstBaseType, AstType};
use crate::base::lang::{Convertible, Direction};
use crate::swift::ty::basic::quote_free_swift_ptr;

pub(crate) struct VecDefault {
    pub(crate) ty: AstType,
}

impl VecDefault {
    fn native_type_str(&self) -> String {
        match self.ty.clone() {
            AstType::Vec(AstBaseType::Boolean) => "[Bool]",
            AstType::Vec(AstBaseType::String) => "[String]",
            _ => "",
        }
        .to_string()
    }
}

impl<'a> Convertible<Swift<'a>> for VecDefault {
    fn native_to_transferable(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        body.append(toks!("autoreleasepool { () -> CInt8Array in",));
        nested!(body, "let encoder = JSONEncoder()");
        nested!(body, "let data = try! encoder.encode(", origin, ")");
        nested!(body, "let str = String(data: data, encoding: .utf8)!");
        nested!(body, "var buffer: UnsafeMutablePointer<Int8>? = nil");
        nested!(body, "var count : Int32 = 0");
        nested!(body, "let cstr = str.cString(using: .utf8)");
        nested!(body, "cstr?.withUnsafeBufferPointer({ bufferPointer in");
        nested!(body, |tt| {
            nested!(
                tt,
                "buffer = UnsafeMutablePointer<Int8>.allocate(capacity: bufferPointer.count)"
            );
            nested!(
                tt,
                "buffer?.initialize(from: bufferPointer.baseAddress!, count: bufferPointer.count)"
            );
            nested!(tt, "count = Int32(bufferPointer.count)");
        });
        nested!(body, "})");
        nested!(body, quote_free_swift_ptr("Int8"));
        nested!(
            body,
            "return CInt8Array(ptr: buffer!, len: count, cap: count, free_ptr: free_ptr)"
        );
        push!(body, "}");
        body
    }

    fn transferable_to_native(
        &self,
        origin: String,
        _direction: Direction,
    ) -> Tokens<'static, Swift<'a>> {
        let mut body = Tokens::new();
        let swift_ty = self.native_type_str();
        body.append(toks_f!("{{ () -> {} in", swift_ty));
        push_f!(body, "let str = String(cString:{}.ptr!)", origin);
        push_f!(
            body,
            "({}.free_ptr)(UnsafeMutablePointer(mutating: {}.ptr!), {}.len, {}.cap)",
            origin,
            origin,
            origin,
            origin
        );
        push_f!(body, "var result:{}?", swift_ty);
        push!(body, "autoreleasepool {");
        nested!(body, "let data = str.data(using: .utf8)!");
        nested!(body, "let decoder = JSONDecoder()");
        nested_f!(
            body,
            "result = try! decoder.decode({}.self, from: data)",
            swift_ty
        );
        push!(body, "}");
        push!(body, "return result!");
        push!(body, "}()");
        body
    }

    fn rust_to_transferable(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {{
            let tmp_json = serde_json::to_string(&#origin);
            let cstr = CString::new(tmp_json.unwrap()).unwrap();
            let bytes = cstr.as_bytes_with_nul();
            let array = CInt8Array {
                ptr: bytes.as_ptr() as (*const i8),
                len: bytes.len() as i32,
                cap: bytes.len() as i32,
                free_ptr: free_str
            };
            std::mem::forget(cstr);
            array
        }}
    }

    fn transferable_to_rust(&self, origin: TokenStream, _direction: Direction) -> TokenStream {
        quote! {{
            let slice = unsafe {std::slice::from_raw_parts(#origin.ptr as (*const u8), #origin.len as usize).to_vec()};
            let cstr = unsafe {CStr::from_bytes_with_nul_unchecked(&slice)};
            let json_str = cstr.to_string_lossy().to_string();
            let object = serde_json::from_str(&json_str).unwrap();
            (#origin.free_ptr)(#origin.ptr as (*mut i8), #origin.len, #origin.len);
            object
        }}
    }

    fn native_type(&self) -> Swift<'a> {
        swift::local(self.native_type_str())
    }

    fn native_transferable_type(&self, direction: Direction) -> Swift<'a> {
        swift::local("CInt8Array")
    }

    fn rust_transferable_type(&self, direction: Direction) -> TokenStream {
        quote!(CInt8Array)
    }

    fn quote_common_bridge(&self) -> TokenStream {
        quote! {}
    }

    fn quote_common_artifact(&self) -> Tokens<'static, Swift<'a>> {
        Tokens::new()
    }
}
