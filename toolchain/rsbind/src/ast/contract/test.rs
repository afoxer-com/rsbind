#[cfg(test)]
mod tests {
    use crate::ast::contract::parser::parse_from_str;
    use crate::ast::types::{AstBaseType, AstType};

    #[test]
    fn parse_contract_works() {
        let contract_str = "
            pub struct FfiStruct {
                pub arg_bool: bool
            }

            pub trait FfiContract {
                fn return_vec() -> Vec<u8>;
                fn arg_vec(command: i32, data: Vec<u8>) -> i32;
                fn arg_callback(command: i32, callback: Box<dyn FfiCallback>) -> i32;
            }

            pub trait FfiCallback {
                fn callback_vec(&self, command: i32, data: Vec<u8>) -> i32;
            }
        ";

        let (trait_desc, struct_desc) =
            parse_from_str("demo_crate", "demo_mod", contract_str).unwrap();
        assert_eq!(trait_desc.len(), 2);
        assert_eq!(struct_desc.len(), 1);
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
    }
}
