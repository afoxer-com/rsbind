pub trait TestContract1 {
    fn test_arg_vec(arg: Vec<String>) -> i32;
    fn test_return_vec(arg: u8) -> Vec<i32>;
    fn test_arg_callback(arg: Box<Callback>) -> u8;
    fn test_bool(arg1: bool) -> bool;
    fn test_struct() -> StructSimple;
    fn test_struct_vec() -> Vec<StructSimple>;
    fn test_two_string(arg1: String, arg2: String) -> String;
    fn test_return_vec_u8(input: Vec<u8>) -> Vec<u8>;
    //    fn test_return_callback(arg: bool) -> Box<Callback>;
}

pub trait Callback: Sync {
    fn on_callback(&self, arg1: i32, arg2: String, arg3: bool, arg4: f32, arg5: f64) -> i32;
    fn on_callback2(&self, arg1: bool) -> bool;
    fn on_callback_complex(&self, arg1: StructSimple) -> bool;
    fn on_callback_arg_vec(&self, arg1: Vec<StructSimple>) -> bool;
    fn on_callback_arg_vec_simple(&self, arg1: Vec<String>) -> bool;
}

pub struct StructSimple {
    pub arg1: i32,
    pub arg2: i8,
    pub arg3: String,
    pub arg4: bool,
    pub arg5: f32,
    pub art6: f64,
}
