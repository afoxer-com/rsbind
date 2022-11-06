pub trait Demo {
    fn test_i8_100(arg100: i8, arg101: i8) -> i8;
    fn test_u8_100(arg100: u8, arg101: u8) -> u8;
    fn test_i16_100(arg100: i16, arg101: i16) -> i16;
    fn test_u16_100(arg100: u16, arg101: u16) -> u16;
    fn test_i32_100(arg100: i32, arg101: i32) -> i32;
    fn test_u32_100(arg100: u32, arg101: u32) -> u32;
    fn test_u64_100(arg100: u64, arg101: u64) -> u64;
    fn test_i64_100(arg100: i64, arg101: i64) -> i64;
    fn test_f32_100(arg100: f32, arg101: f32) -> f32;
    fn test_f64_100(arg100: f64, arg101: f64) -> f64;
    fn test_boolean_true(arg_true: bool, arg_false: bool) -> bool;
    fn test_string_hello(arg_hello: String) -> String;
    fn test_i8_array(arg_2_array: Vec<i8>) -> Vec<i8>;
    fn test_i16_array(arg_2_array: Vec<i16>) -> Vec<i16>;
    fn test_i32_array(arg_2_array: Vec<i32>) -> Vec<i32>;
    fn test_i64_array(arg_2_array: Vec<i64>) -> Vec<i64>;
    fn test_u8_array(arg_2_array: Vec<u8>) -> Vec<u8>;
    fn test_u16_array(arg_2_array: Vec<u16>) -> Vec<u16>;
    fn test_u32_array(arg_2_array: Vec<u32>) -> Vec<u32>;
    fn test_u64_array(arg_2_array: Vec<u64>) -> Vec<u64>;
    fn test_bool_true_array(arg_true_array: Vec<bool>) -> Vec<bool>;
    fn test_bool_false_array(arg_false_array: Vec<bool>) -> Vec<bool>;
    fn test_struct(demo_struct: DemoTestStruct) -> DemoTestStruct;
    fn test_callback(callback: Box<dyn DemoCallback>) -> Box<dyn DemoCallback>;
}

pub trait DemoCallback: Send + Sync {
    fn test_i8_100(&self, arg100: i8, arg101: i8) -> i8;
    // fn test_u8_100(arg100: u8, arg101: u8) -> u8;
    // fn test_i16_100(arg100: i16, arg101: i16) -> i16;
    // fn test_u16_100(arg100: u16, arg101: u16) -> u16;
    // fn test_i32_100(arg100: i32, arg101: i32) -> i32;
    // fn test_u32_100(arg100: u32, arg101: u32) -> u32;
    // fn test_u64_100(arg100: u64, arg101: u64) -> u64;
    // fn test_i64_100(arg100: i64, arg101: i64) -> i64;
    // fn test_f32_100(arg100: f32, arg101: f32) -> f32;
    // fn test_f64_100(arg100: f64, arg101: f64) -> f64;
    // fn test_boolean_true(arg_true: bool, arg_false: bool) -> bool;
    // fn test_string_hello(arg_hello: String) -> String;
    // fn test_i8_array(arg_2_array: Vec<i8>) -> Vec<i8>;
    // fn test_i16_array(arg_2_array: Vec<i16>) -> Vec<i16>;
    // fn test_i32_array(arg_2_array: Vec<i32>) -> Vec<i32>;
    // fn test_i64_array(arg_2_array: Vec<i64>) -> Vec<i64>;
    // fn test_u8_array(arg_2_array: Vec<u8>) -> Vec<u8>;
    // fn test_u16_array(arg_2_array: Vec<u16>) -> Vec<u16>;
    // fn test_u32_array(arg_2_array: Vec<u32>) -> Vec<u32>;
    // fn test_u64_array(arg_2_array: Vec<u64>) -> Vec<u64>;
    // fn test_bool_true_array(arg_true_array: Vec<bool>) -> Vec<bool>;
    // fn test_bool_false_array(arg_false_array: Vec<bool>) -> Vec<bool>;
    // fn test_struct(demo_struct: DemoTestStruct) -> DemoTestStruct;
}

struct DemoCallbackImp {}

impl DemoCallback for DemoCallbackImp {
    fn test_i8_100(&self, arg100: i8, arg101: i8) -> i8 {
        assert_eq!(arg100, 100);
        assert_eq!(arg101, 101);
        100
    }
}

pub struct DemoTestStruct {
    pub i8_1: i8,
    pub u8_2: u8,
    pub i16_3: i16,
    pub u16_4: u16,
    pub i32_5: i32,
    pub u32_6: u32,
    pub i64_7: i64,
    pub u64_8: u64,
    pub f32_9: f32,
    pub f64_10: f64,
    pub bool_true: bool,
    pub str_hello: String,
}

pub struct DemoStruct {}

impl Demo for DemoStruct {
    fn test_i8_100(arg100: i8, arg101: i8) -> i8 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_u8_100(arg100: u8, arg101: u8) -> u8 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_i16_100(arg100: i16, arg101: i16) -> i16 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_u16_100(arg100: u16, arg101: u16) -> u16 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_i32_100(arg100: i32, arg101: i32) -> i32 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_u32_100(arg100: u32, arg101: u32) -> u32 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_u64_100(arg100: u64, arg101: u64) -> u64 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_i64_100(arg100: i64, arg101: i64) -> i64 {
        assert_eq!(arg100, 100, "");
        assert_eq!(arg101, 101, "");
        100
    }

    fn test_f32_100(arg100: f32, arg101: f32) -> f32 {
        assert_eq!(arg100, 100.0, "");
        assert_eq!(arg101, 101.0, "");
        100.0
    }

    fn test_f64_100(arg100: f64, arg101: f64) -> f64 {
        assert_eq!(arg100, 100.0, "");
        assert_eq!(arg101, 101.0, "");
        100.0
    }

    fn test_boolean_true(arg_true: bool, arg_false: bool) -> bool {
        assert_eq!(arg_true, true, "");
        assert_eq!(arg_false, false, "");
        true
    }

    fn test_string_hello(arg_hello: String) -> String {
        assert_eq!(arg_hello, "Hello", "");
        "Hello".to_string()
    }

    fn test_i8_array(arg_2_array: Vec<i8>) -> Vec<i8> {
        assert_eq!(arg_2_array[0], 2i8, "");
        assert_eq!(arg_2_array[1], 2i8, "");
        vec![2i8, 2i8]
    }

    fn test_i16_array(arg_2_array: Vec<i16>) -> Vec<i16> {
        assert_eq!(arg_2_array[0], 2i16, "");
        assert_eq!(arg_2_array[1], 2i16, "");
        vec![2i16, 2i16]
    }

    fn test_i32_array(arg_2_array: Vec<i32>) -> Vec<i32> {
        assert_eq!(arg_2_array[0], 2i32, "");
        assert_eq!(arg_2_array[1], 2i32, "");
        vec![2i32, 2i32]
    }

    fn test_i64_array(arg_2_array: Vec<i64>) -> Vec<i64> {
        assert_eq!(arg_2_array[0], 2i64, "");
        assert_eq!(arg_2_array[1], 2i64, "");
        vec![2i64, 2i64]
    }

    fn test_u8_array(arg_2_array: Vec<u8>) -> Vec<u8> {
        assert_eq!(arg_2_array[0], 2u8, "");
        assert_eq!(arg_2_array[1], 2u8, "");
        vec![2u8, 2u8]
    }

    fn test_u16_array(arg_2_array: Vec<u16>) -> Vec<u16> {
        assert_eq!(arg_2_array[0], 2u16, "");
        assert_eq!(arg_2_array[1], 2u16, "");
        vec![2u16, 2u16]
    }

    fn test_u32_array(arg_2_array: Vec<u32>) -> Vec<u32> {
        assert_eq!(arg_2_array[0], 2u32, "");
        assert_eq!(arg_2_array[1], 2u32, "");
        vec![2u32, 2u32]
    }

    fn test_u64_array(arg_2_array: Vec<u64>) -> Vec<u64> {
        assert_eq!(arg_2_array[0], 2u64, "");
        assert_eq!(arg_2_array[1], 2u64, "");
        vec![2u64, 2u64]
    }

    fn test_bool_true_array(arg_true_array: Vec<bool>) -> Vec<bool> {
        assert_eq!(arg_true_array[0], true, "");
        assert_eq!(arg_true_array[1], true, "");
        vec![true, true]
    }

    fn test_bool_false_array(arg_false_array: Vec<bool>) -> Vec<bool> {
        assert_eq!(arg_false_array[0], false, "");
        assert_eq!(arg_false_array[1], false, "");
        vec![false, false]
    }

    fn test_struct(demo_struct: DemoTestStruct) -> DemoTestStruct {
        assert_struct(demo_struct);
        create_demo_struct()
    }

    fn test_callback(callback: Box<dyn DemoCallback>) -> Box<dyn DemoCallback> {
        let result = callback.test_i8_100(100, 101);
        assert_eq!(result, 100, "");
        Box::new()
    }
}

fn assert_struct(demo_struct: DemoTestStruct) {
    assert_eq!(demo_struct.i8_1, 1, "");
    assert_eq!(demo_struct.u8_2, 2, "");
    assert_eq!(demo_struct.i16_3, 3, "");
    assert_eq!(demo_struct.u16_4, 4, "");
    assert_eq!(demo_struct.i32_5, 5, "");
    assert_eq!(demo_struct.u32_6, 6, "");
    assert_eq!(demo_struct.i64_7, 7, "");
    assert_eq!(demo_struct.u64_8, 8, "");
    assert_eq!(demo_struct.f32_9, 9.0, "");
    assert_eq!(demo_struct.f64_10, 10.0, "");
    assert_eq!(demo_struct.bool_true, true, "");
    assert_eq!(demo_struct.str_hello, "Hello".to_string(), "");
}

fn create_demo_struct() -> DemoTestStruct {
    DemoTestStruct {
        i8_1: 1,
        u8_2: 2,
        i16_3: 3,
        u16_4: 4,
        i32_5: 5,
        u32_6: 6,
        i64_7: 7,
        u64_8: 8,
        f32_9: 9.0,
        f64_10: 10.0,
        bool_true: true,
        str_hello: "Hello".to_string(),
    }
}
