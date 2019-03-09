#[cfg(test)]
mod tests {
    use bridge::java;
    use std::path::PathBuf;
    use std::fs;
    use ast::contract;
    use ast::imp;

    #[test]
    fn gen_jni_works() {
        let test_path = PathBuf::from("./.test");
        let traits = contract::parser::parse(&PathBuf::from("./test_res/contract/sample_contract.rs")).unwrap();
        let imp = imp::parser::parse_dir(&PathBuf::from("./test_res/imp")).unwrap();
        let jni_gen = java::new_gen(&test_path,
                                    &traits,
                                    &imp,
                                    "com.bytedance.ee.bear");
        let result = jni_gen.gen_one_bridge_file("java_sample_contract.rs").unwrap();
        let content = fs::read_to_string(&test_path.join("jni_sample_contract.rs"));
        println!("result text = {:#?}", content.unwrap());
    }
}
