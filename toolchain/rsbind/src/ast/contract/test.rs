#[cfg(test)]
mod tests {
    use syn::token::As;

    use crate::ast::contract::parser::parse_from_str;
    use crate::ast::types::{AstBaseType, AstType};

    #[test]
    fn parse_contract_works() {
        let contract_str = "
            pub struct FfiStruct {
                pub arg1: i32,
                pub arg2: u32,
                pub arg3: i16,
                pub arg4: u16,
                pub arg5: i8,
                pub arg6: u8,
                pub arg7_str: String,
                pub arg8_false: bool,
                pub arg9: f32,
                pub arg10: f64,
                pub arg11: Vec<u8>,
                pub arg12: Vec<u16>,
                pub arg13: Vec<u32>,
                pub arg14: Vec<u64>,
                pub arg15: Vec<f32>,
                pub arg16: Vec<f64>,
                pub arg17: Vec<i8>,
                pub arg18: Vec<i16>,
                pub arg19: Vec<i32>,
                pub arg20: Vec<i64>,
                pub arg21_true: Vec<bool>,
            }

            pub trait FfiContract : Send + Sync {
                fn return_vec() -> Vec<u8>;
                fn arg_vec(command: i32, data: Vec<u8>) -> i32;
                fn arg_callback(command: i32, callback: Box<dyn FfiCallback>) -> i32;
                fn return_callback() -> Box<dyn FfiCallback>;
            }

            pub trait FfiCallback : Sync + Send {
                fn callback_vec(&self, command: i32, data: Vec<u8>) -> i32;
                fn test_self(self);
            }
        ";

        let (trait_desc, struct_desc) =
            parse_from_str("demo_crate", "demo_mod", contract_str, "contract").unwrap();
        assert_eq!(trait_desc.len(), 2);
        assert_eq!(struct_desc.len(), 1);

        assert_eq!(struct_desc[0].fields.len(), 21);
        assert_eq!(struct_desc[0].name, "FfiStruct");
        assert_eq!(struct_desc[0].fields[0].name, "arg1");
        assert_eq!(struct_desc[0].fields[0].ty.origin(), "i32");
        assert_eq!(struct_desc[0].fields[1].name, "arg2");
        assert_eq!(struct_desc[0].fields[1].ty.origin(), "u32");
        assert_eq!(struct_desc[0].fields[2].name, "arg3");
        assert_eq!(struct_desc[0].fields[2].ty.origin(), "i16");
        assert_eq!(struct_desc[0].fields[3].name, "arg4");
        assert_eq!(struct_desc[0].fields[3].ty.origin(), "u16");
        assert_eq!(struct_desc[0].fields[4].name, "arg5");
        assert_eq!(struct_desc[0].fields[4].ty.origin(), "i8");
        assert_eq!(struct_desc[0].fields[5].name, "arg6");
        assert_eq!(struct_desc[0].fields[5].ty.origin(), "u8");
        assert_eq!(struct_desc[0].fields[6].name, "arg7_str");
        assert_eq!(struct_desc[0].fields[6].ty.origin(), "String");
        assert_eq!(struct_desc[0].fields[7].name, "arg8_false");
        assert_eq!(struct_desc[0].fields[7].ty.origin(), "bool");
        assert_eq!(struct_desc[0].fields[8].name, "arg9");
        assert_eq!(struct_desc[0].fields[8].ty.origin(), "f32");
        assert_eq!(struct_desc[0].fields[9].name, "arg10");
        assert_eq!(struct_desc[0].fields[9].ty.origin(), "f64");
        assert_eq!(struct_desc[0].fields[10].name, "arg11");
        assert_eq!(struct_desc[0].fields[10].ty.origin(), "Vec<u8>");
        assert_eq!(struct_desc[0].fields[11].name, "arg12");
        assert_eq!(struct_desc[0].fields[11].ty.origin(), "Vec<u16>");
        assert_eq!(struct_desc[0].fields[12].name, "arg13");
        assert_eq!(struct_desc[0].fields[12].ty.origin(), "Vec<u32>");
        assert_eq!(struct_desc[0].fields[13].name, "arg14");
        assert_eq!(struct_desc[0].fields[13].ty.origin(), "Vec<u64>");
        assert_eq!(struct_desc[0].fields[14].name, "arg15");
        assert_eq!(struct_desc[0].fields[14].ty.origin(), "Vec<f32>");
        assert_eq!(struct_desc[0].fields[15].name, "arg16");
        assert_eq!(struct_desc[0].fields[15].ty.origin(), "Vec<f64>");
        assert_eq!(struct_desc[0].fields[16].name, "arg17");
        assert_eq!(struct_desc[0].fields[16].ty.origin(), "Vec<i8>");
        assert_eq!(struct_desc[0].fields[17].name, "arg18");
        assert_eq!(struct_desc[0].fields[17].ty.origin(), "Vec<i16>");
        assert_eq!(struct_desc[0].fields[18].name, "arg19");
        assert_eq!(struct_desc[0].fields[18].ty.origin(), "Vec<i32>");
        assert_eq!(struct_desc[0].fields[19].name, "arg20");
        assert_eq!(struct_desc[0].fields[19].ty.origin(), "Vec<i64>");
        assert_eq!(struct_desc[0].fields[20].name, "arg21_true");
        assert_eq!(struct_desc[0].fields[20].ty.origin(), "Vec<bool>");

        assert_eq!(trait_desc[0].name, "FfiContract");
        assert_eq!(trait_desc[0].crate_name, "demo_crate");
        assert_eq!(trait_desc[0].mod_name, "demo_mod");
        assert!(!trait_desc[0].is_callback);

        assert_eq!(trait_desc[0].methods[0].name, "return_vec");
        assert_eq!(trait_desc[0].methods[0].args.len(), 0);
        assert_eq!(trait_desc[0].methods[0].return_type.origin(), "Vec<u8>");
        assert_eq!(
            trait_desc[0].methods[0].return_type,
            AstType::Vec(AstBaseType::Byte("u8".to_string()))
        );

        assert_eq!(trait_desc[0].methods[1].name, "arg_vec");
        assert_eq!(trait_desc[0].methods[1].args[0].name, "command");
        assert_eq!(
            trait_desc[0].methods[1].args[0].ty,
            AstType::Int("i32".to_string())
        );
        assert_eq!(trait_desc[0].methods[1].args[1].name, "data");
        assert_eq!(
            trait_desc[0].methods[1].args[1].ty,
            AstType::Vec(AstBaseType::Byte("u8".to_string()))
        );
        assert_eq!(
            trait_desc[0].methods[1].return_type,
            AstType::Int("i32".to_string())
        );

        assert_eq!(trait_desc[0].methods[2].name, "arg_callback");
        assert_eq!(trait_desc[0].methods[2].args[0].name, "command");
        assert_eq!(
            trait_desc[0].methods[2].args[0].ty,
            AstType::Int("i32".to_string())
        );
        assert_eq!(trait_desc[0].methods[2].args[1].name, "callback");
        assert_eq!(
            trait_desc[0].methods[2].args[1].ty,
            AstType::Callback("FfiCallback".to_string())
        );
        assert_eq!(
            trait_desc[0].methods[2].return_type,
            AstType::Int("i32".to_string())
        );
        assert_eq!(trait_desc[0].methods[3].name, "return_callback");
        assert!(trait_desc[0].methods[3].args.is_empty());
        assert_eq!(
            trait_desc[0].methods[3].return_type,
            AstType::Callback("FfiCallback".to_string())
        );

        assert_eq!(trait_desc[1].name, "FfiCallback");
        assert_eq!(trait_desc[1].mod_name, "demo_mod");
        assert_eq!(trait_desc[1].crate_name, "demo_crate");
        assert!(trait_desc[1].is_callback);
        assert_eq!(trait_desc[1].methods[0].name, "callback_vec");
        assert_eq!(
            trait_desc[1].methods[0].return_type,
            AstType::Int("i32".to_string())
        );
        assert_eq!(trait_desc[1].methods[0].args[0].name, "command");
        assert_eq!(
            trait_desc[1].methods[0].args[0].ty,
            AstType::Int("i32".to_string())
        );
        assert_eq!(trait_desc[1].methods[0].args[1].name, "data");
        assert_eq!(
            trait_desc[1].methods[0].args[1].ty,
            AstType::Vec(AstBaseType::Byte("u8".to_string()))
        );
        assert_eq!(trait_desc[1].methods[1].name, "test_self")
    }
}
