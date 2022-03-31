use contract::test_contract1::DemoStruct;
use contract::test_contract1::DemoTrait;
use contract::test_contract1::{DemoCallback, DemoCallback2, DemoTrait2};

use log::{Level, LevelFilter};

pub struct TestContract1Imp {}

impl DemoTrait for TestContract1Imp {
    fn setup() {
        error!("We call setup");
    }

    fn test_u8_1(arg: u8, arg2: u8) -> u8 {
        error!("We call test_u8_1");
        assert(arg == 100 && arg2 == 101, "test_u8_1");
        1
    }

    fn test_i8_2(arg: i8, arg2: i8) -> i8 {
        error!("We call test_i8_2");
        assert(arg == 100 && arg2 == 101, "test_u8_1");
        2
    }

    fn test_i16_3(arg: i16, arg2: i16) -> i16 {
        assert(arg == 100 && arg2 == 101, "test_u8_1");
        3
    }

    fn test_u16_4(arg: u16, arg2: u16) -> u16 {
        assert(arg == 100 && arg2 == 101, "test_u16_4");
        4
    }

    fn test_i32_5(arg: i32, arg2: i32) -> i32 {
        assert(arg == 100 && arg2 == 101, "test_i32_5");
        5
    }

    fn test_u32_6(arg: u32, arg2: u32) -> u32 {
        assert(arg == 100 && arg2 == 101, "test_u32_6");
        6
    }

    fn test_i64_7(arg: i64, arg2: i64) -> i64 {
        assert(arg == 100 && arg2 == 101, "test_i64_7");
        7
    }

    fn test_u64_7(arg: u64, arg2: u64) -> u64 {
        assert(arg == 100 && arg2 == 101, "test_u64_7");
        7
    }

    fn test_bool_false(arg_true: bool, arg2_false: bool) -> bool {
        assert(arg_true && !arg2_false, "test_bool_false");
        false
    }

    fn test_f32_30(arg: f32, arg2: f32) -> f32 {
        assert(arg > 99.0 && arg2 > 100.0, "test_float_30");
        30.0
    }

    fn test_f64_31(arg: f64, arg2: f64) -> f64 {
        assert(arg > 99.0 && arg2 > 100.0, "test_float_31");
        31.0
    }

    fn test_str(arg: String) -> String {
        assert_eq(arg.as_str(), "Hello world", "test_str");
        "Hello world".to_string()
    }

    fn test_arg_vec_str_7(arg: Vec<String>) -> i32 {
        assert_eq(
            arg.get(0).unwrap().as_str(),
            "Hello world",
            "test_arg_vec_str_7",
        );
        7
    }

    fn test_arg_vec_u8_true(arg: Vec<u8>) -> bool {
        assert_eq(arg.get(0).unwrap(), &100u8, "test_arg_vec_u8_true");
        true
    }

    fn test_arg_vec_i8_6(arg: Vec<i8>) -> i32 {
        assert_eq(arg.get(0).unwrap(), &100i8, "test_arg_vec_i8_6");
        6
    }

    fn test_arg_vec_i16_9(arg: Vec<i16>) -> i32 {
        assert_eq(arg.get(0).unwrap(), &100i16, "test_arg_vec_i16_9");
        9
    }

    fn test_arg_vec_u16_10(arg: Vec<u16>) -> i32 {
        assert_eq(arg.get(0).unwrap(), &100u16, "test_arg_vec_u16_10");
        10
    }

    fn test_arg_vec_i32_11(arg: Vec<i32>) -> i32 {
        assert_eq(arg.get(0).unwrap(), &100i32, "test_arg_vec_i32_11");
        11
    }

    fn test_arg_vec_u32_12(arg: Vec<u32>) -> i32 {
        assert_eq(arg.get(0).unwrap(), &100u32, "test_arg_vec_u32_12");
        12
    }

    fn test_arg_vec_i64_11(arg: Vec<i64>) -> i64 {
        assert_eq(arg.get(0).unwrap(), &100, "test_arg_vec_i64_11");
        11
    }

    fn test_arg_vec_u64_12(arg: Vec<u64>) -> u64 {
        assert_eq(arg.get(0).unwrap(), &100, "test_arg_vec_u64_12");
        12
    }

    fn test_arg_vec_bool_13(arg_true: Vec<bool>) -> i32 {
        assert_eq(arg_true.get(0).unwrap(), &true, "test_arg_vec_bool_13");
        13
    }

    fn test_arg_vec_struct_14(arg: Vec<DemoStruct>) -> i32 {
        assert_struct(arg.get(0).unwrap(), "test_arg_vec_struct_14");
        14
    }

    fn test_two_vec_arg_15(arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
        let v1 = arg.get(0).unwrap();
        let v2 = arg1.get(0).unwrap();

        assert_eq(v1, &100i32, "test_two_vec_arg_15");
        assert_eq(v2, &101u32, "test_two_vec_arg_15");
        15
    }

    fn test_return_vec_str() -> Vec<String> {
        vec!["Hello world".to_string()]
    }

    fn test_return_vec_u8() -> Vec<u8> {
        vec![100u8]
    }

    fn test_return_vec_i8() -> Vec<i8> {
        vec![100i8]
    }

    fn test_return_vec_i16() -> Vec<i16> {
        vec![100i16]
    }

    fn test_return_vec_u16() -> Vec<u16> {
        vec![100u16]
    }

    fn test_return_vec_i32() -> Vec<i32> {
        vec![100i32]
    }

    fn test_return_vec_u32() -> Vec<u32> {
        vec![100u32]
    }

    fn test_return_vec_i64() -> Vec<i64> {
        vec![100]
    }

    fn test_return_vec_u64() -> Vec<u64> {
        vec![100]
    }

    fn test_return_vec_bool_true() -> Vec<bool> {
        vec![true]
    }

    fn test_two_vec_u8(input: Vec<u8>) -> Vec<u8> {
        let v = input.get(0).unwrap();
        assert_eq(v, &100u8, "test_two_vec_u8");
        vec![100u8]
    }

    fn test_return_vec_struct() -> Vec<DemoStruct> {
        vec![new_struct()]
    }

    fn test_arg_callback_16(arg: Box<dyn DemoCallback>) -> u8 {
        handle_callback(arg)
    }

    fn test_two_arg_callback_20(arg: Box<dyn DemoCallback>, arg1: Box<dyn DemoCallback>) -> u8 {
        handle_callback(arg);
        handle_callback(arg1);
        20
    }

    fn test_return_callback() -> Box<dyn DemoCallback> {
        struct CallbackStruct {};
        impl DemoCallback for CallbackStruct {
            fn test_u8_1(&self, arg: u8, arg2: u8) -> u8 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                1
            }

            fn test_i8_2(&self, arg: i8, arg2: i8) -> i8 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                2
            }

            fn test_i16_3(&self, arg: i16, arg2: i16) -> i16 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                3
            }

            fn test_u16_4(&self, arg: u16, arg2: u16) -> u16 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                4
            }

            fn test_i32_5(&self, arg: i32, arg2: i32) -> i32 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                5
            }

            fn test_u32_6(&self, arg: u32, arg2: u32) -> u32 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                6
            }

            fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool {
                assert_eq!(arg_true, true);
                assert_eq!(arg_false, false);
                false
            }

            fn test_f32_30(&self, arg: f32, arg2: f32) -> f32 {
                assert!(arg > 0.0 && arg2 > 0.0);
                30.0
            }

            fn test_f64_31(&self, arg: f64, arg2: f64) -> f64 {
                assert!(arg > 0.0 && arg2 > 0.0);
                31.0
            }

            fn test_i64_7(&self, arg: i64, arg2: i64) -> i64 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                7
            }

            fn test_u64_7(&self, arg: u64, arg2: u64) -> u64 {
                assert_eq!(arg, 100);
                assert_eq!(arg2, 101);
                7
            }

            fn test_str(&self, arg: String) -> String {
                assert_eq!(arg, "Hello world");
                "Hello world".to_string()
            }

            fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), "Hello world");
                18
            }

            fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100u8);
                7
            }

            fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100i8);
                8
            }

            fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100i16);
                9
            }

            fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100u16);
                10
            }

            fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100i32);
                11
            }

            fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32 {
                assert_eq!(arg.get(0).unwrap(), &100u32);
                12
            }

            fn test_arg_vec_i64_11(&self, arg: Vec<i64>) -> i64 {
                assert_eq!(arg.get(0).unwrap(), &100i64);
                11
            }

            fn test_arg_vec_u64_12(&self, arg: Vec<u64>) -> u64 {
                assert_eq!(arg.get(0).unwrap(), &100u64);
                12
            }

            fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool {
                assert_eq!(arg_true.get(0).unwrap(), &true);
                true
            }

            fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32 {
                assert_struct(&arg.get(0).unwrap(), "test_arg_vec_struct_17");
                17
            }

            fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
                assert_eq!(arg.get(0).unwrap(), &100i32);
                assert_eq!(arg1.get(0).unwrap(), &101u32);
                13
            }

            fn test_return_vec_str(&self) -> Vec<String> {
                vec!["Hello world".to_string()]
            }

            fn test_return_vec_u8(&self) -> Vec<u8> {
                vec![100]
            }

            fn test_return_vec_i8(&self) -> Vec<i8> {
                vec![100]
            }

            fn test_return_vec_i16(&self) -> Vec<i16> {
                vec![100]
            }

            fn test_return_vec_u16(&self) -> Vec<u16> {
                vec![100]
            }

            fn test_return_vec_i32(&self) -> Vec<i32> {
                vec![100]
            }

            fn test_return_vec_u32(&self) -> Vec<u32> {
                vec![100]
            }

            fn test_return_vec_i64(&self) -> Vec<i64> {
                vec![100]
            }

            fn test_return_vec_u64(&self) -> Vec<u64> {
                vec![100]
            }

            fn test_return_vec_bool_true(&self) -> Vec<bool> {
                vec![true]
            }

            fn test_two_vec_u8(&self, input: Vec<u8>) -> Vec<u8> {
                vec![100]
            }

            fn test_return_vec_struct(&self) -> Vec<DemoStruct> {
                vec![new_struct()]
            }

            fn test_arg_struct_14(&self, arg: DemoStruct) -> i32 {
                assert_struct(&arg, "test_arg_struct_14");
                14
            }

            fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32 {
                assert_struct(&arg, "test_arg_struct_14");
                assert_struct(&arg1, "test_arg_struct_14");
                15
            }

            fn test_return_struct(&self) -> DemoStruct {
                new_struct()
            }

            fn test_no_return(&self) {}
        }

        Box::new(CallbackStruct {})
    }

    fn test_return_struct() -> DemoStruct {
        new_struct()
    }

    fn test_arg_struct(arg: DemoStruct) {
        assert_struct(&arg, "test_arg_struct");
    }

    fn test_no_return() {}
}

impl DemoTrait2 for TestContract1Imp {
    fn test_return_callback2(arg: i8) -> Box<dyn DemoCallback2> {
        struct Instance {}
        impl DemoCallback2 for Instance {
            fn test_return_callback2(&self, arg: i8) -> Box<dyn DemoCallback> {
                create_callback()
            }

            fn test_arg_callback3(&self, arg: Box<dyn DemoCallback>) -> u8 {
                handle_callback(arg);
                3
            }
        }

        return Box::new(Instance {});
    }

    fn test_arg_callback3(arg: Box<dyn DemoCallback2>) -> u8 {
        arg.test_arg_callback3(create_callback());
        let callback = arg.test_return_callback2(100);
        handle_callback(callback);
        3
    }
}

fn handle_callback(arg: Box<dyn DemoCallback>) -> u8 {
    error!("We call handle_callback test_u8_1");
    assert_eq(&arg.test_u8_1(100, 101), &1, "handle_callback");
    error!("We call handle_callback test_i8_2");
    assert_eq(&arg.test_i8_2(100, 101), &2, "handle_callback");
    error!("We call handle_callback test_i16_3");
    assert_eq(&arg.test_i16_3(100, 101), &3, "handle_callback");
    error!("We call handle_callback test_u16_4");
    assert_eq(&arg.test_u16_4(100, 101), &4, "handle_callback");
    error!("We call handle_callback test_i32_5");
    assert_eq(&arg.test_i32_5(100, 101), &5, "handle_callback");
    error!("We call handle_callback test_u32_6");
    assert_eq(&arg.test_u32_6(100, 101), &6, "handle_callback");
    error!("We call handle_callback test_bool_false");
    assert_eq(&arg.test_bool_false(true, false), &false, "handle_callback");
    error!("We call handle_callback test_f32_30");
    assert(arg.test_f32_30(100.0, 101.0) > 29.0, "test_f32_30");
    error!("We call handle_callback test_f64_31");
    assert(arg.test_f64_31(100.0, 101.0) > 30.0, "test_f64_31");
    error!("We call handle_callback test_arg_vec_str_18");
    assert_eq(
        &arg.test_str("Hello world".to_string()),
        &"Hello world".to_string(),
        "test_str",
    );
    assert_eq(
        &arg.test_arg_vec_str_18(vec!["Hello world".to_string()]),
        &18i32,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_vec_u8_7");
    assert_eq(&arg.test_arg_vec_u8_7(vec![100u8]), &7, "handle_callback");
    error!("We call handle_callback test_arg_vec_i8_8");
    assert_eq(&arg.test_arg_vec_i8_8(vec![100i8]), &8, "handle_callback");
    error!("We call handle_callback test_arg_vec_i16_9");
    assert_eq(&arg.test_arg_vec_i16_9(vec![100i16]), &9, "handle_callback");
    error!("We call handle_callback test_arg_vec_u16_10");
    assert_eq(
        &arg.test_arg_vec_u16_10(vec![100u16]),
        &10,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_vec_i32_11");
    assert_eq(
        &arg.test_arg_vec_i32_11(vec![100i32]),
        &11,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_vec_u32_12");
    assert_eq(
        &arg.test_arg_vec_u32_12(vec![100u32]),
        &12,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_vec_bool_true");
    assert_eq(
        &arg.test_arg_vec_bool_true(vec![true]),
        &true,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_vec_struct_17");
    assert_eq(
        &arg.test_arg_vec_struct_17(vec![new_struct()]),
        &17,
        "handle_callback",
    );
    error!("We call handle_callback test_two_vec_arg_13");
    assert_eq(
        &arg.test_two_vec_arg_13(vec![100i32], vec![101u32]),
        &13,
        "handle_callback",
    );
    error!("We call handle_callback test_arg_struct_14");
    let r = arg.test_arg_struct_14(new_struct());
    assert_eq(&r, &14, "handle_callback");
    error!("We call handle_callback test_two_arg_struct_15");
    let r = arg.test_two_arg_struct_15(new_struct(), new_struct());
    assert_eq(&r, &15, "handle_callback");
    error!("We call handle_callback test_no_return");
    arg.test_no_return();
    assert_eq(&arg.test_i64_7(100, 101), &7, "assert_eq");
    assert_eq(&arg.test_u64_7(100, 101), &7, "test_u64_7");
    assert_eq(
        &arg.test_arg_vec_i64_11(vec![100]),
        &11,
        "test_arg_vec_i64_11",
    );
    assert_eq(
        &arg.test_arg_vec_u64_12(vec![100]),
        &12,
        "test_arg_vec_u64_12",
    );

    assert_eq(&arg.test_return_vec_str(), &vec!["Hello world".to_string()], "test_return_vec_str");
    assert_eq(&arg.test_return_vec_u8(), &vec![100], "test_return_vec_u8");
    assert_eq(&arg.test_return_vec_i8(), &vec![100], "test_return_vec_i8");
    assert_eq(
        &arg.test_return_vec_i16(),
        &vec![100],
        "test_return_vec_i16",
    );
    assert_eq(
        &arg.test_return_vec_u16(),
        &vec![100],
        "test_return_vec_u16",
    );
    assert_eq(
        &arg.test_return_vec_i32(),
        &vec![100],
        "test_return_vec_i32",
    );
    assert_eq(
        &arg.test_return_vec_u32(),
        &vec![100],
        "test_return_vec_u32",
    );
    assert_eq(
        &arg.test_return_vec_i64(),
        &vec![100],
        "test_return_vec_i64",
    );
    assert_eq(
        &arg.test_return_vec_u64(),
        &vec![100],
        "test_return_vec_u64",
    );
    // assert_eq(&arg.test_return_vec_bool_true(), &vec![true], "test_return_vec_bool_true");
    assert_eq(
        &arg.test_two_vec_u8(vec![100]),
        &vec![100],
        "test_two_vec_u8",
    );
    // assert_struct(&arg.test_return_vec_struct()[0], "test_return_vec_struct");

    16
}

fn create_callback() -> Box<dyn DemoCallback> {
    struct CallbackStruct{};
    impl DemoCallback for CallbackStruct {
        fn test_u8_1(&self, arg: u8, arg2: u8) -> u8 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            1
        }

        fn test_i8_2(&self, arg: i8, arg2: i8) -> i8 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            2
        }

        fn test_i16_3(&self, arg: i16, arg2: i16) -> i16 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            3
        }

        fn test_u16_4(&self, arg: u16, arg2: u16) -> u16 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            4
        }

        fn test_i32_5(&self, arg: i32, arg2: i32) -> i32 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            5
        }

        fn test_u32_6(&self, arg: u32, arg2: u32) -> u32 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            6
        }

        fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool {
            assert_eq!(arg_true, true);
            assert_eq!(arg_false, false);
            false
        }

        fn test_f32_30(&self, arg: f32, arg2: f32) -> f32 {
            assert!(arg > 0.0 && arg2 > 0.0);
            30.0
        }

        fn test_f64_31(&self, arg: f64, arg2: f64) -> f64 {
            assert!(arg > 0.0 && arg2 > 0.0);
            31.0
        }

        fn test_i64_7(&self, arg: i64, arg2: i64) -> i64 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            7
        }

        fn test_u64_7(&self, arg: u64, arg2: u64) -> u64 {
            assert_eq!(arg, 100);
            assert_eq!(arg2, 101);
            7
        }

        fn test_str(&self, arg: String) -> String {
            assert_eq!(arg, "Hello world");
            "Hello world".to_string()
        }

        fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), "Hello world");
            18
        }

        fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100u8);
            7
        }

        fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100i8);
            8
        }

        fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100i16);
            9
        }

        fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100u16);
            10
        }

        fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100i32);
            11
        }

        fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32 {
            assert_eq!(arg.get(0).unwrap(), &100u32);
            12
        }

        fn test_arg_vec_i64_11(&self, arg: Vec<i64>) -> i64 {
            assert_eq!(arg.get(0).unwrap(), &100i64);
            11
        }

        fn test_arg_vec_u64_12(&self, arg: Vec<u64>) -> u64 {
            assert_eq!(arg.get(0).unwrap(), &100u64);
            12
        }

        fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool {
            assert_eq!(arg_true.get(0).unwrap(), &true);
            true
        }

        fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32 {
            assert_struct(&arg.get(0).unwrap(), "test_arg_vec_struct_17");
            17
        }

        fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
            assert_eq!(arg.get(0).unwrap(), &100i32);
            assert_eq!(arg1.get(0).unwrap(), &101u32);
            13
        }

        fn test_return_vec_str(&self) -> Vec<String> {
            vec!["Hello world".to_string()]
        }

        fn test_return_vec_u8(&self) -> Vec<u8> {
            vec![100]
        }

        fn test_return_vec_i8(&self) -> Vec<i8> {
            vec![100]
        }

        fn test_return_vec_i16(&self) -> Vec<i16> {
            vec![100]
        }

        fn test_return_vec_u16(&self) -> Vec<u16> {
            vec![100]
        }

        fn test_return_vec_i32(&self) -> Vec<i32> {
            vec![100]
        }

        fn test_return_vec_u32(&self) -> Vec<u32> {
            vec![100]
        }

        fn test_return_vec_i64(&self) -> Vec<i64> {
            vec![100]
        }

        fn test_return_vec_u64(&self) -> Vec<u64> {
            vec![100]
        }

        fn test_return_vec_bool_true(&self) -> Vec<bool> {
            vec![true]
        }

        fn test_two_vec_u8(&self, input: Vec<u8>) -> Vec<u8> {
            vec![100]
        }

        fn test_return_vec_struct(&self) -> Vec<DemoStruct> {
            vec![new_struct()]
        }

        fn test_arg_struct_14(&self, arg: DemoStruct) -> i32 {
            assert_struct(&arg, "test_arg_struct_14");
            14
        }

        fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32 {
            assert_struct(&arg, "test_arg_struct_14");
            assert_struct(&arg1, "test_arg_struct_14");
            15
        }

        fn test_return_struct(&self) -> DemoStruct {
            new_struct()
        }

        fn test_no_return(&self) {

        }
    }

    Box::new(CallbackStruct{})
}

fn new_struct() -> DemoStruct {
    DemoStruct {
        arg1: 1,
        arg2: 2,
        arg3: 3,
        arg4: 4,
        arg5: 5,
        arg6: 6,
        arg7_str: "Hello world".to_string(),
        arg8_false: false,
        arg9: 9.0,
        arg10: 10.0,
    }
}

fn assert_struct(arg: &DemoStruct, fn_name: &str) {
    let v = arg;
    assert(
        v.arg1 == 1
            && v.arg2 == 2
            && v.arg3 == 3
            && v.arg4 == 4
            && v.arg5 == 5
            && v.arg6 == 6
            && v.arg7_str == "Hello world"
            && !v.arg8_false
            && v.arg9 > 0.0
            && v.arg10 > 0.0,
        fn_name,
    );
}

fn assert(condition: bool, fn_name: &str) {
    if !condition {
        error!("{} failed!", fn_name);
        panic!("{} failed!", fn_name);
    }
}

fn assert_eq<T: PartialEq + std::fmt::Debug + ?Sized>(expected: &T, actual: &T, fn_name: &str) {
    if expected != actual {
        error!(
            "Need {:?}, actual is {:?} in {:?}",
            expected, actual, fn_name
        );
        // panic!("Need {}, actual is {} in {}", expected, actual, fn_name);
    }
}
