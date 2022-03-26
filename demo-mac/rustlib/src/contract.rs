pub trait DemoTrait : Send + Sync {
    fn setup();

    // Base types
    fn test_u8_1(arg: u8, arg2: u8) -> u8;
    fn test_i8_2(arg: i8, arg2: i8) -> i8;
    fn test_i16_3(arg: i16, arg2: i16) -> i16;
    fn test_u16_4(arg: u16, arg2: u16) -> u16;
    fn test_i32_5(arg: i32, arg2: i32) -> i32;
    fn test_u32_6(arg: u32, arg2: u32) -> u32;
    fn test_i64_7(arg: i64, arg2: i64) -> i64;
    fn test_u64_7(arg: u64, arg2: u64) -> u64;
    fn test_bool_false(arg_true: bool, arg2_false: bool) -> bool;
    fn test_f32_30(arg: f32, arg2: f32) -> f32;
    fn test_f64_31(arg: f64, arg2: f64) -> f64;

    // String
    fn test_str(arg: String) -> String;

    // Vec argument
    fn test_arg_vec_str_7(arg: Vec<String>) -> i32;
    fn test_arg_vec_u8_true(arg: Vec<u8>) -> bool;
    fn test_arg_vec_i8_6(arg: Vec<i8>) -> i32;
    fn test_arg_vec_i16_9(arg: Vec<i16>) -> i32;
    fn test_arg_vec_u16_10(arg: Vec<u16>) -> i32;
    fn test_arg_vec_i32_11(arg: Vec<i32>) -> i32;
    fn test_arg_vec_u32_12(arg: Vec<u32>) -> i32;
    fn test_arg_vec_i64_11(arg: Vec<i64>) -> i64;
    fn test_arg_vec_u64_12(arg: Vec<u64>) -> u64;
    fn test_arg_vec_bool_13(arg_true: Vec<bool>) -> i32;
    fn test_arg_vec_struct_14(arg: Vec<DemoStruct>) -> i32;
    fn test_two_vec_arg_15(arg: Vec<i32>, arg1: Vec<u32>) -> u32;

    // Vec return
    fn test_return_vec_str() -> Vec<String>;
    fn test_return_vec_u8() -> Vec<u8>;
    fn test_return_vec_i8() -> Vec<i8>;
    fn test_return_vec_i16() -> Vec<i16>;
    fn test_return_vec_u16() -> Vec<u16>;
    fn test_return_vec_i32() -> Vec<i32>;
    fn test_return_vec_u32() -> Vec<u32>;
    fn test_return_vec_i64() -> Vec<i64>;
    fn test_return_vec_u64() -> Vec<u64>;
    fn test_return_vec_bool_true() -> Vec<bool>;
    fn test_two_vec_u8(input: Vec<u8>) -> Vec<u8>;
    fn test_return_vec_struct() -> Vec<DemoStruct>;

    // Callback
    fn test_arg_callback_16(arg: Box<dyn DemoCallback>) -> u8;
    fn test_two_arg_callback_20(arg: Box<dyn DemoCallback>, arg1: Box<dyn DemoCallback>) -> u8;
    fn test_return_callback() -> Box<dyn DemoCallback>;

    // Struct
    fn test_return_struct() -> DemoStruct;
    fn test_arg_struct(arg: DemoStruct);
    fn test_no_return();
}

pub trait DemoTrait2 : Send + Sync {
    fn test_u8_2(arg: u8) -> u8;
    fn test_arg_callback1(callback: Box<dyn DemoCallback2>) -> i8;
    fn test_return_calllback2() -> Box<dyn DemoCallback2>;
}

pub trait DemoCallback: Sync + Send {
    // Base types
    fn test_u8_1(&self, arg: u8, arg2: u8) -> u8;
    fn test_i8_2(&self, arg: i8, arg2: i8) -> i8;
    fn test_i16_3(&self, arg: i16, arg2: i16) -> i16;
    fn test_u16_4(&self, arg: u16, arg2: u16) -> u16;
    fn test_i32_5(&self, arg: i32, arg2: i32) -> i32;
    fn test_u32_6(&self, arg: u32, arg2: u32) -> u32;
    fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool;
    fn test_f32_30(&self, arg: f32, arg2: f32) -> f32;
    fn test_f64_31(&self, arg: f64, arg2: f64) -> f64;
    fn test_i64_7(&self, arg: i64, arg2: i64) -> i64;
    fn test_u64_7(&self, arg: u64, arg2: u64) -> u64;

    // String
    fn test_str(&self, arg: String) -> String;

    // Vec argument
    fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32;
    fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32;
    fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32;
    fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32;
    fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32;
    fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32;
    fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32;
    fn test_arg_vec_i64_11(&self, arg: Vec<i64>) -> i64;
    fn test_arg_vec_u64_12(&self, arg: Vec<u64>) -> u64;
    fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool;
    fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32;
    fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32;

    // Vec return
    // fn test_return_vec_str(&self) -> Vec<String>;
    fn test_return_vec_u8(&self) -> Vec<u8>;
    fn test_return_vec_i8(&self) -> Vec<i8>;
    fn test_return_vec_i16(&self) -> Vec<i16>;
    fn test_return_vec_u16(&self) -> Vec<u16>;
    fn test_return_vec_i32(&self) -> Vec<i32>;
    fn test_return_vec_u32(&self) -> Vec<u32>;
    fn test_return_vec_i64(&self) -> Vec<i64>;
    fn test_return_vec_u64(&self) -> Vec<u64>;
    // fn test_return_vec_bool_true(&self) -> Vec<bool>;
    fn test_two_vec_u8(&self, input: Vec<u8>) -> Vec<u8>;
    // fn test_return_vec_struct(&self) -> Vec<DemoStruct>;

    // Struct
    fn test_arg_struct_14(&self, arg: DemoStruct) -> i32;
    fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32;
    fn test_no_return(&self);
}

pub trait DemoCallback2: Sync + Send {
    fn test_arg_callback_16(&self, arg: Box<dyn DemoCallback>) -> u8;
    fn test_return_callback(&self) -> Box<dyn DemoCallback>;
}

pub struct DemoStruct {
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
}
