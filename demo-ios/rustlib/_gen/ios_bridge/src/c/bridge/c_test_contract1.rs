use c::bridge::common::*;
use contract::test_contract1::*;
use imp::test_contract1_imp::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Struct_DemoStruct {
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
impl From<DemoStruct> for Struct_DemoStruct {
    fn from(origin: DemoStruct) -> Self {
        Struct_DemoStruct {
            arg1: origin.arg1,
            arg2: origin.arg2,
            arg3: origin.arg3,
            arg4: origin.arg4,
            arg5: origin.arg5,
            arg6: origin.arg6,
            arg7_str: origin.arg7_str,
            arg8_false: origin.arg8_false,
            arg9: origin.arg9,
            arg10: origin.arg10,
        }
    }
}
impl From<Struct_DemoStruct> for DemoStruct {
    fn from(origin: Struct_DemoStruct) -> Self {
        DemoStruct {
            arg1: origin.arg1,
            arg2: origin.arg2,
            arg3: origin.arg3,
            arg4: origin.arg4,
            arg5: origin.arg5,
            arg6: origin.arg6,
            arg7_str: origin.arg7_str,
            arg8_false: origin.arg8_false,
            arg9: origin.arg9,
            arg10: origin.arg10,
        }
    }
}
#[no_mangle]
pub extern "C" fn test_contract1_setup() {
    TestContract1Imp::setup();
}
#[no_mangle]
pub extern "C" fn test_contract1_test_u8_1(arg: i8, arg2: i8) -> i8 {
    let r_arg = arg as u8;
    let r_arg2 = arg2 as u8;
    let ret_value = TestContract1Imp::test_u8_1(r_arg, r_arg2);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_i8_2(arg: i8, arg2: i8) -> i8 {
    let r_arg = arg as i8;
    let r_arg2 = arg2 as i8;
    let ret_value = TestContract1Imp::test_i8_2(r_arg, r_arg2);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_i16_3(arg: i32, arg2: i32) -> i32 {
    let r_arg = arg as i16;
    let r_arg2 = arg2 as i16;
    let ret_value = TestContract1Imp::test_i16_3(r_arg, r_arg2);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_u16_4(arg: i32, arg2: i32) -> i32 {
    let r_arg = arg as u16;
    let r_arg2 = arg2 as u16;
    let ret_value = TestContract1Imp::test_u16_4(r_arg, r_arg2);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_i32_5(arg: i32, arg2: i32) -> i32 {
    let r_arg = arg as i32;
    let r_arg2 = arg2 as i32;
    let ret_value = TestContract1Imp::test_i32_5(r_arg, r_arg2);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_u32_6(arg: i32, arg2: i32) -> i32 {
    let r_arg = arg as u32;
    let r_arg2 = arg2 as u32;
    let ret_value = TestContract1Imp::test_u32_6(r_arg, r_arg2);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_bool_false(arg_true: i32, arg2_false: i32) -> i32 {
    let r_arg_true = if arg_true > 0 { true } else { false };
    let r_arg2_false = if arg2_false > 0 { true } else { false };
    let ret_value = TestContract1Imp::test_bool_false(r_arg_true, r_arg2_false);
    if ret_value {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn test_contract1_test_f32_30(arg: f32, arg2: f32) -> f32 {
    let r_arg = arg as f32;
    let r_arg2 = arg2 as f32;
    let ret_value = TestContract1Imp::test_f32_30(r_arg, r_arg2);
    ret_value as f32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_f64_31(arg: f64, arg2: f64) -> f64 {
    let r_arg = arg as f64;
    let r_arg2 = arg2 as f64;
    let ret_value = TestContract1Imp::test_f64_31(r_arg, r_arg2);
    ret_value as f64
}
#[no_mangle]
pub extern "C" fn test_contract1_test_str(arg: *const c_char) -> *mut c_char {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_str_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg: String = c_str_arg.to_owned();
    let ret_value = TestContract1Imp::test_str(r_arg);
    CString::new(ret_value).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_str_7(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_str_7(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_u8_true(arg: CInt8Array) -> i32 {
    let r_arg =
        unsafe { std::slice::from_raw_parts(arg.ptr as (*const u8), arg.len as usize).to_vec() };
    let ret_value = TestContract1Imp::test_arg_vec_u8_true(r_arg);
    if ret_value {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_i8_6(arg: CInt8Array) -> i32 {
    let r_arg =
        unsafe { std::slice::from_raw_parts(arg.ptr as (*const i8), arg.len as usize).to_vec() };
    let ret_value = TestContract1Imp::test_arg_vec_i8_6(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_i16_9(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_i16_9(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_u16_10(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_u16_10(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_i32_11(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_i32_11(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_u32_12(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_u32_12(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec_bool_13(arg_true: *const c_char) -> i32 {
    let c_str_arg_true: &CStr = unsafe { CStr::from_ptr(arg_true) };
    let c_slice_arg_true: &str = c_str_arg_true.to_str().unwrap();
    let r_arg_true = serde_json::from_str(&c_slice_arg_true.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec_bool_13(r_arg_true);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_two_vec_arg_15(
    arg: *const c_char,
    arg1: *const c_char,
) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let c_str_arg1: &CStr = unsafe { CStr::from_ptr(arg1) };
    let c_slice_arg1: &str = c_str_arg1.to_str().unwrap();
    let r_arg1 = serde_json::from_str(&c_slice_arg1.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_two_vec_arg_15(r_arg, r_arg1);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_str() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_str();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_u8() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_u8();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_i8() -> *mut c_char {
    let mut ret_value = TestContract1Imp::test_return_vec_i8();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_i16() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_i16();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_u16() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_u16();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_i32() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_i32();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_u32() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_u32();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_bool_true() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_bool_true();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_two_vec_u8(input: CInt8Array) -> *mut c_char {
    let r_input = unsafe {
        std::slice::from_raw_parts(input.ptr as (*const u8), input.len as usize).to_vec()
    };
    let ret_value = TestContract1Imp::test_two_vec_u8(r_input);
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_struct() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_vec_struct();
    let ret_value = ret_value
        .into_iter()
        .map(|each| Struct_DemoStruct::from(each))
        .collect::<Vec<Struct_DemoStruct>>();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_callback_16(
    arg: test_contract1_DemoCallback_Model,
) -> i8 {
    pub struct arg_struct {
        pub test_u8_1: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i8_2: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i16_3: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u16_4: extern "C" fn(i64, i32, i32) -> i32,
        pub test_i32_5: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u32_6: extern "C" fn(i64, i32, i32) -> i32,
        pub test_bool_false: extern "C" fn(i64, i32, i32) -> i32,
        pub test_f32_30: extern "C" fn(i64, f32, f32) -> f32,
        pub test_f64_31: extern "C" fn(i64, f64, f64) -> f64,
        pub test_arg_vec_str_18: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u8_7: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i8_8: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i16_9: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u16_10: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_i32_11: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u32_12: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_bool_true: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_struct_17: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_vec_arg_13: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_arg_struct_14: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_arg_struct_15: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_no_return: extern "C" fn(i64) -> (),
        pub free_callback: extern "C" fn(i64),
        pub index: i64,
    }
    impl DemoCallback for arg_struct {
        fn test_u8_1(&self, arg: u8, arg2: u8) -> u8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_u8_1 = self.test_u8_1;
            let result = fn_test_u8_1(self.index, c_arg, c_arg2);
            let s_result = result as u8;
            s_result
        }
        fn test_i8_2(&self, arg: i8, arg2: i8) -> i8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_i8_2 = self.test_i8_2;
            let result = fn_test_i8_2(self.index, c_arg, c_arg2);
            let s_result = result as i8;
            s_result
        }
        fn test_i16_3(&self, arg: i16, arg2: i16) -> i16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i16_3 = self.test_i16_3;
            let result = fn_test_i16_3(self.index, c_arg, c_arg2);
            let s_result = result as i16;
            s_result
        }
        fn test_u16_4(&self, arg: u16, arg2: u16) -> u16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u16_4 = self.test_u16_4;
            let result = fn_test_u16_4(self.index, c_arg, c_arg2);
            let s_result = result as u16;
            s_result
        }
        fn test_i32_5(&self, arg: i32, arg2: i32) -> i32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i32_5 = self.test_i32_5;
            let result = fn_test_i32_5(self.index, c_arg, c_arg2);
            let s_result = result as i32;
            s_result
        }
        fn test_u32_6(&self, arg: u32, arg2: u32) -> u32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u32_6 = self.test_u32_6;
            let result = fn_test_u32_6(self.index, c_arg, c_arg2);
            let s_result = result as u32;
            s_result
        }
        fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool {
            let c_arg_true = if arg_true { 1 } else { 0 };
            let c_arg_false = if arg_false { 1 } else { 0 };
            let fn_test_bool_false = self.test_bool_false;
            let result = fn_test_bool_false(self.index, c_arg_true, c_arg_false);
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_f32_30(&self, arg: f32, arg2: f32) -> f32 {
            let c_arg = arg as f32;
            let c_arg2 = arg2 as f32;
            let fn_test_f32_30 = self.test_f32_30;
            let result = fn_test_f32_30(self.index, c_arg, c_arg2);
            let s_result = result as f32;
            s_result
        }
        fn test_f64_31(&self, arg: f64, arg2: f64) -> f64 {
            let c_arg = arg as f64;
            let c_arg2 = arg2 as f64;
            let fn_test_f64_31 = self.test_f64_31;
            let result = fn_test_f64_31(self.index, c_arg, c_arg2);
            let s_result = result as f64;
            s_result
        }
        fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_str_18 = self.test_arg_vec_str_18;
            let result = fn_test_arg_vec_str_18(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_u8_7 = self.test_arg_vec_u8_7;
            let result = fn_test_arg_vec_u8_7(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_i8_8 = self.test_arg_vec_i8_8;
            let result = fn_test_arg_vec_i8_8(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i16_9 = self.test_arg_vec_i16_9;
            let result = fn_test_arg_vec_i16_9(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u16_10 = self.test_arg_vec_u16_10;
            let result = fn_test_arg_vec_u16_10(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i32_11 = self.test_arg_vec_i32_11;
            let result = fn_test_arg_vec_i32_11(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u32_12 = self.test_arg_vec_u32_12;
            let result = fn_test_arg_vec_u32_12(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool {
            let c_tmp_arg_true = serde_json::to_string(&arg_true);
            let c_arg_true = CString::new(c_tmp_arg_true.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_bool_true = self.test_arg_vec_bool_true;
            let result = fn_test_arg_vec_bool_true(self.index, c_arg_true);
            unsafe { CString::from_raw(c_arg_true) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32 {
            let c_tmp_vec_arg = arg
                .into_iter()
                .map(|each| Struct_DemoStruct::from(each))
                .collect::<Vec<Struct_DemoStruct>>();
            let c_tmp_arg = serde_json::to_string(&c_tmp_vec_arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_struct_17 = self.test_arg_vec_struct_17;
            let result = fn_test_arg_vec_struct_17(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&arg1);
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_vec_arg_13 = self.test_two_vec_arg_13;
            let result = fn_test_two_vec_arg_13(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_arg_struct_14(&self, arg: DemoStruct) -> i32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_struct_14 = self.test_arg_struct_14;
            let result = fn_test_arg_struct_14(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&Struct_DemoStruct::from(arg1));
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_arg_struct_15 = self.test_two_arg_struct_15;
            let result = fn_test_two_arg_struct_15(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_no_return(&self) -> () {
            let fn_test_no_return = self.test_no_return;
            let result = fn_test_no_return(self.index);
        }
    }
    impl Drop for arg_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    let r_arg = Box::new(arg_struct {
        test_u8_1: arg.test_u8_1,
        test_i8_2: arg.test_i8_2,
        test_i16_3: arg.test_i16_3,
        test_u16_4: arg.test_u16_4,
        test_i32_5: arg.test_i32_5,
        test_u32_6: arg.test_u32_6,
        test_bool_false: arg.test_bool_false,
        test_f32_30: arg.test_f32_30,
        test_f64_31: arg.test_f64_31,
        test_arg_vec_str_18: arg.test_arg_vec_str_18,
        test_arg_vec_u8_7: arg.test_arg_vec_u8_7,
        test_arg_vec_i8_8: arg.test_arg_vec_i8_8,
        test_arg_vec_i16_9: arg.test_arg_vec_i16_9,
        test_arg_vec_u16_10: arg.test_arg_vec_u16_10,
        test_arg_vec_i32_11: arg.test_arg_vec_i32_11,
        test_arg_vec_u32_12: arg.test_arg_vec_u32_12,
        test_arg_vec_bool_true: arg.test_arg_vec_bool_true,
        test_arg_vec_struct_17: arg.test_arg_vec_struct_17,
        test_two_vec_arg_13: arg.test_two_vec_arg_13,
        test_arg_struct_14: arg.test_arg_struct_14,
        test_two_arg_struct_15: arg.test_two_arg_struct_15,
        test_no_return: arg.test_no_return,
        free_callback: arg.free_callback,
        index: arg.index,
    });
    let ret_value = TestContract1Imp::test_arg_callback_16(r_arg);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_two_arg_callback_20(
    arg: test_contract1_DemoCallback_Model,
    arg1: test_contract1_DemoCallback_Model,
) -> i8 {
    pub struct arg_struct {
        pub test_u8_1: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i8_2: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i16_3: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u16_4: extern "C" fn(i64, i32, i32) -> i32,
        pub test_i32_5: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u32_6: extern "C" fn(i64, i32, i32) -> i32,
        pub test_bool_false: extern "C" fn(i64, i32, i32) -> i32,
        pub test_f32_30: extern "C" fn(i64, f32, f32) -> f32,
        pub test_f64_31: extern "C" fn(i64, f64, f64) -> f64,
        pub test_arg_vec_str_18: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u8_7: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i8_8: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i16_9: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u16_10: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_i32_11: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u32_12: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_bool_true: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_struct_17: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_vec_arg_13: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_arg_struct_14: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_arg_struct_15: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_no_return: extern "C" fn(i64) -> (),
        pub free_callback: extern "C" fn(i64),
        pub index: i64,
    }
    impl DemoCallback for arg_struct {
        fn test_u8_1(&self, arg: u8, arg2: u8) -> u8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_u8_1 = self.test_u8_1;
            let result = fn_test_u8_1(self.index, c_arg, c_arg2);
            let s_result = result as u8;
            s_result
        }
        fn test_i8_2(&self, arg: i8, arg2: i8) -> i8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_i8_2 = self.test_i8_2;
            let result = fn_test_i8_2(self.index, c_arg, c_arg2);
            let s_result = result as i8;
            s_result
        }
        fn test_i16_3(&self, arg: i16, arg2: i16) -> i16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i16_3 = self.test_i16_3;
            let result = fn_test_i16_3(self.index, c_arg, c_arg2);
            let s_result = result as i16;
            s_result
        }
        fn test_u16_4(&self, arg: u16, arg2: u16) -> u16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u16_4 = self.test_u16_4;
            let result = fn_test_u16_4(self.index, c_arg, c_arg2);
            let s_result = result as u16;
            s_result
        }
        fn test_i32_5(&self, arg: i32, arg2: i32) -> i32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i32_5 = self.test_i32_5;
            let result = fn_test_i32_5(self.index, c_arg, c_arg2);
            let s_result = result as i32;
            s_result
        }
        fn test_u32_6(&self, arg: u32, arg2: u32) -> u32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u32_6 = self.test_u32_6;
            let result = fn_test_u32_6(self.index, c_arg, c_arg2);
            let s_result = result as u32;
            s_result
        }
        fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool {
            let c_arg_true = if arg_true { 1 } else { 0 };
            let c_arg_false = if arg_false { 1 } else { 0 };
            let fn_test_bool_false = self.test_bool_false;
            let result = fn_test_bool_false(self.index, c_arg_true, c_arg_false);
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_f32_30(&self, arg: f32, arg2: f32) -> f32 {
            let c_arg = arg as f32;
            let c_arg2 = arg2 as f32;
            let fn_test_f32_30 = self.test_f32_30;
            let result = fn_test_f32_30(self.index, c_arg, c_arg2);
            let s_result = result as f32;
            s_result
        }
        fn test_f64_31(&self, arg: f64, arg2: f64) -> f64 {
            let c_arg = arg as f64;
            let c_arg2 = arg2 as f64;
            let fn_test_f64_31 = self.test_f64_31;
            let result = fn_test_f64_31(self.index, c_arg, c_arg2);
            let s_result = result as f64;
            s_result
        }
        fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_str_18 = self.test_arg_vec_str_18;
            let result = fn_test_arg_vec_str_18(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_u8_7 = self.test_arg_vec_u8_7;
            let result = fn_test_arg_vec_u8_7(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_i8_8 = self.test_arg_vec_i8_8;
            let result = fn_test_arg_vec_i8_8(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i16_9 = self.test_arg_vec_i16_9;
            let result = fn_test_arg_vec_i16_9(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u16_10 = self.test_arg_vec_u16_10;
            let result = fn_test_arg_vec_u16_10(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i32_11 = self.test_arg_vec_i32_11;
            let result = fn_test_arg_vec_i32_11(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u32_12 = self.test_arg_vec_u32_12;
            let result = fn_test_arg_vec_u32_12(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool {
            let c_tmp_arg_true = serde_json::to_string(&arg_true);
            let c_arg_true = CString::new(c_tmp_arg_true.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_bool_true = self.test_arg_vec_bool_true;
            let result = fn_test_arg_vec_bool_true(self.index, c_arg_true);
            unsafe { CString::from_raw(c_arg_true) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32 {
            let c_tmp_vec_arg = arg
                .into_iter()
                .map(|each| Struct_DemoStruct::from(each))
                .collect::<Vec<Struct_DemoStruct>>();
            let c_tmp_arg = serde_json::to_string(&c_tmp_vec_arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_struct_17 = self.test_arg_vec_struct_17;
            let result = fn_test_arg_vec_struct_17(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&arg1);
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_vec_arg_13 = self.test_two_vec_arg_13;
            let result = fn_test_two_vec_arg_13(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_arg_struct_14(&self, arg: DemoStruct) -> i32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_struct_14 = self.test_arg_struct_14;
            let result = fn_test_arg_struct_14(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&Struct_DemoStruct::from(arg1));
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_arg_struct_15 = self.test_two_arg_struct_15;
            let result = fn_test_two_arg_struct_15(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_no_return(&self) -> () {
            let fn_test_no_return = self.test_no_return;
            let result = fn_test_no_return(self.index);
        }
    }
    impl Drop for arg_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    let r_arg = Box::new(arg_struct {
        test_u8_1: arg.test_u8_1,
        test_i8_2: arg.test_i8_2,
        test_i16_3: arg.test_i16_3,
        test_u16_4: arg.test_u16_4,
        test_i32_5: arg.test_i32_5,
        test_u32_6: arg.test_u32_6,
        test_bool_false: arg.test_bool_false,
        test_f32_30: arg.test_f32_30,
        test_f64_31: arg.test_f64_31,
        test_arg_vec_str_18: arg.test_arg_vec_str_18,
        test_arg_vec_u8_7: arg.test_arg_vec_u8_7,
        test_arg_vec_i8_8: arg.test_arg_vec_i8_8,
        test_arg_vec_i16_9: arg.test_arg_vec_i16_9,
        test_arg_vec_u16_10: arg.test_arg_vec_u16_10,
        test_arg_vec_i32_11: arg.test_arg_vec_i32_11,
        test_arg_vec_u32_12: arg.test_arg_vec_u32_12,
        test_arg_vec_bool_true: arg.test_arg_vec_bool_true,
        test_arg_vec_struct_17: arg.test_arg_vec_struct_17,
        test_two_vec_arg_13: arg.test_two_vec_arg_13,
        test_arg_struct_14: arg.test_arg_struct_14,
        test_two_arg_struct_15: arg.test_two_arg_struct_15,
        test_no_return: arg.test_no_return,
        free_callback: arg.free_callback,
        index: arg.index,
    });
    pub struct arg1_struct {
        pub test_u8_1: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i8_2: extern "C" fn(i64, i8, i8) -> i8,
        pub test_i16_3: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u16_4: extern "C" fn(i64, i32, i32) -> i32,
        pub test_i32_5: extern "C" fn(i64, i32, i32) -> i32,
        pub test_u32_6: extern "C" fn(i64, i32, i32) -> i32,
        pub test_bool_false: extern "C" fn(i64, i32, i32) -> i32,
        pub test_f32_30: extern "C" fn(i64, f32, f32) -> f32,
        pub test_f64_31: extern "C" fn(i64, f64, f64) -> f64,
        pub test_arg_vec_str_18: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u8_7: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i8_8: extern "C" fn(i64, CInt8Array) -> i32,
        pub test_arg_vec_i16_9: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u16_10: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_i32_11: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_u32_12: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_bool_true: extern "C" fn(i64, *const c_char) -> i32,
        pub test_arg_vec_struct_17: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_vec_arg_13: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_arg_struct_14: extern "C" fn(i64, *const c_char) -> i32,
        pub test_two_arg_struct_15: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
        pub test_no_return: extern "C" fn(i64) -> (),
        pub free_callback: extern "C" fn(i64),
        pub index: i64,
    }
    impl DemoCallback for arg1_struct {
        fn test_u8_1(&self, arg: u8, arg2: u8) -> u8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_u8_1 = self.test_u8_1;
            let result = fn_test_u8_1(self.index, c_arg, c_arg2);
            let s_result = result as u8;
            s_result
        }
        fn test_i8_2(&self, arg: i8, arg2: i8) -> i8 {
            let c_arg = arg as i8;
            let c_arg2 = arg2 as i8;
            let fn_test_i8_2 = self.test_i8_2;
            let result = fn_test_i8_2(self.index, c_arg, c_arg2);
            let s_result = result as i8;
            s_result
        }
        fn test_i16_3(&self, arg: i16, arg2: i16) -> i16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i16_3 = self.test_i16_3;
            let result = fn_test_i16_3(self.index, c_arg, c_arg2);
            let s_result = result as i16;
            s_result
        }
        fn test_u16_4(&self, arg: u16, arg2: u16) -> u16 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u16_4 = self.test_u16_4;
            let result = fn_test_u16_4(self.index, c_arg, c_arg2);
            let s_result = result as u16;
            s_result
        }
        fn test_i32_5(&self, arg: i32, arg2: i32) -> i32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_i32_5 = self.test_i32_5;
            let result = fn_test_i32_5(self.index, c_arg, c_arg2);
            let s_result = result as i32;
            s_result
        }
        fn test_u32_6(&self, arg: u32, arg2: u32) -> u32 {
            let c_arg = arg as i32;
            let c_arg2 = arg2 as i32;
            let fn_test_u32_6 = self.test_u32_6;
            let result = fn_test_u32_6(self.index, c_arg, c_arg2);
            let s_result = result as u32;
            s_result
        }
        fn test_bool_false(&self, arg_true: bool, arg_false: bool) -> bool {
            let c_arg_true = if arg_true { 1 } else { 0 };
            let c_arg_false = if arg_false { 1 } else { 0 };
            let fn_test_bool_false = self.test_bool_false;
            let result = fn_test_bool_false(self.index, c_arg_true, c_arg_false);
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_f32_30(&self, arg: f32, arg2: f32) -> f32 {
            let c_arg = arg as f32;
            let c_arg2 = arg2 as f32;
            let fn_test_f32_30 = self.test_f32_30;
            let result = fn_test_f32_30(self.index, c_arg, c_arg2);
            let s_result = result as f32;
            s_result
        }
        fn test_f64_31(&self, arg: f64, arg2: f64) -> f64 {
            let c_arg = arg as f64;
            let c_arg2 = arg2 as f64;
            let fn_test_f64_31 = self.test_f64_31;
            let result = fn_test_f64_31(self.index, c_arg, c_arg2);
            let s_result = result as f64;
            s_result
        }
        fn test_arg_vec_str_18(&self, arg: Vec<String>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_str_18 = self.test_arg_vec_str_18;
            let result = fn_test_arg_vec_str_18(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u8_7(&self, arg: Vec<u8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_u8_7 = self.test_arg_vec_u8_7;
            let result = fn_test_arg_vec_u8_7(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i8_8(&self, arg: Vec<i8>) -> i32 {
            let c_arg = unsafe {
                CInt8Array {
                    ptr: arg.as_ptr() as (*const i8),
                    len: arg.len() as i32,
                }
            };
            let fn_test_arg_vec_i8_8 = self.test_arg_vec_i8_8;
            let result = fn_test_arg_vec_i8_8(self.index, c_arg);
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i16_9(&self, arg: Vec<i16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i16_9 = self.test_arg_vec_i16_9;
            let result = fn_test_arg_vec_i16_9(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u16_10(&self, arg: Vec<u16>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u16_10 = self.test_arg_vec_u16_10;
            let result = fn_test_arg_vec_u16_10(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_i32_11(&self, arg: Vec<i32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_i32_11 = self.test_arg_vec_i32_11;
            let result = fn_test_arg_vec_i32_11(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_u32_12(&self, arg: Vec<u32>) -> i32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_u32_12 = self.test_arg_vec_u32_12;
            let result = fn_test_arg_vec_u32_12(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_arg_vec_bool_true(&self, arg_true: Vec<bool>) -> bool {
            let c_tmp_arg_true = serde_json::to_string(&arg_true);
            let c_arg_true = CString::new(c_tmp_arg_true.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_bool_true = self.test_arg_vec_bool_true;
            let result = fn_test_arg_vec_bool_true(self.index, c_arg_true);
            unsafe { CString::from_raw(c_arg_true) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn test_arg_vec_struct_17(&self, arg: Vec<DemoStruct>) -> i32 {
            let c_tmp_vec_arg = arg
                .into_iter()
                .map(|each| Struct_DemoStruct::from(each))
                .collect::<Vec<Struct_DemoStruct>>();
            let c_tmp_arg = serde_json::to_string(&c_tmp_vec_arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_vec_struct_17 = self.test_arg_vec_struct_17;
            let result = fn_test_arg_vec_struct_17(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_vec_arg_13(&self, arg: Vec<i32>, arg1: Vec<u32>) -> u32 {
            let c_tmp_arg = serde_json::to_string(&arg);
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&arg1);
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_vec_arg_13 = self.test_two_vec_arg_13;
            let result = fn_test_two_vec_arg_13(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_arg_struct_14(&self, arg: DemoStruct) -> i32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let fn_test_arg_struct_14 = self.test_arg_struct_14;
            let result = fn_test_arg_struct_14(self.index, c_arg);
            unsafe { CString::from_raw(c_arg) };
            let s_result = result as i32;
            s_result
        }
        fn test_two_arg_struct_15(&self, arg: DemoStruct, arg1: DemoStruct) -> u32 {
            let c_tmp_arg = serde_json::to_string(&Struct_DemoStruct::from(arg));
            let c_arg = CString::new(c_tmp_arg.unwrap()).unwrap().into_raw();
            let c_tmp_arg1 = serde_json::to_string(&Struct_DemoStruct::from(arg1));
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_test_two_arg_struct_15 = self.test_two_arg_struct_15;
            let result = fn_test_two_arg_struct_15(self.index, c_arg, c_arg1);
            unsafe { CString::from_raw(c_arg) };
            unsafe { CString::from_raw(c_arg1) };
            let s_result = result as u32;
            s_result
        }
        fn test_no_return(&self) -> () {
            let fn_test_no_return = self.test_no_return;
            let result = fn_test_no_return(self.index);
        }
    }
    impl Drop for arg1_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    let r_arg1 = Box::new(arg1_struct {
        test_u8_1: arg1.test_u8_1,
        test_i8_2: arg1.test_i8_2,
        test_i16_3: arg1.test_i16_3,
        test_u16_4: arg1.test_u16_4,
        test_i32_5: arg1.test_i32_5,
        test_u32_6: arg1.test_u32_6,
        test_bool_false: arg1.test_bool_false,
        test_f32_30: arg1.test_f32_30,
        test_f64_31: arg1.test_f64_31,
        test_arg_vec_str_18: arg1.test_arg_vec_str_18,
        test_arg_vec_u8_7: arg1.test_arg_vec_u8_7,
        test_arg_vec_i8_8: arg1.test_arg_vec_i8_8,
        test_arg_vec_i16_9: arg1.test_arg_vec_i16_9,
        test_arg_vec_u16_10: arg1.test_arg_vec_u16_10,
        test_arg_vec_i32_11: arg1.test_arg_vec_i32_11,
        test_arg_vec_u32_12: arg1.test_arg_vec_u32_12,
        test_arg_vec_bool_true: arg1.test_arg_vec_bool_true,
        test_arg_vec_struct_17: arg1.test_arg_vec_struct_17,
        test_two_vec_arg_13: arg1.test_two_vec_arg_13,
        test_arg_struct_14: arg1.test_arg_struct_14,
        test_two_arg_struct_15: arg1.test_two_arg_struct_15,
        test_no_return: arg1.test_no_return,
        free_callback: arg1.free_callback,
        index: arg1.index,
    });
    let ret_value = TestContract1Imp::test_two_arg_callback_20(r_arg, r_arg1);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_struct() -> *mut c_char {
    let ret_value = TestContract1Imp::test_return_struct();
    let json_ret = serde_json::to_string(&Struct_DemoStruct::from(ret_value));
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_struct(arg: *const c_char) {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_slice_arg: &str = c_str_arg.to_str().unwrap();
    let c_tmp_arg: Struct_DemoStruct = serde_json::from_str(&c_slice_arg.to_owned()).unwrap();
    let r_arg = c_tmp_arg.into();
    TestContract1Imp::test_arg_struct(r_arg);
}
#[no_mangle]
pub extern "C" fn test_contract1_test_no_return() {
    TestContract1Imp::test_no_return();
}
#[repr(C)]
pub struct test_contract1_DemoCallback_Model {
    pub test_u8_1: extern "C" fn(i64, i8, i8) -> i8,
    pub test_i8_2: extern "C" fn(i64, i8, i8) -> i8,
    pub test_i16_3: extern "C" fn(i64, i32, i32) -> i32,
    pub test_u16_4: extern "C" fn(i64, i32, i32) -> i32,
    pub test_i32_5: extern "C" fn(i64, i32, i32) -> i32,
    pub test_u32_6: extern "C" fn(i64, i32, i32) -> i32,
    pub test_bool_false: extern "C" fn(i64, i32, i32) -> i32,
    pub test_f32_30: extern "C" fn(i64, f32, f32) -> f32,
    pub test_f64_31: extern "C" fn(i64, f64, f64) -> f64,
    pub test_arg_vec_str_18: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_u8_7: extern "C" fn(i64, CInt8Array) -> i32,
    pub test_arg_vec_i8_8: extern "C" fn(i64, CInt8Array) -> i32,
    pub test_arg_vec_i16_9: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_u16_10: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_i32_11: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_u32_12: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_bool_true: extern "C" fn(i64, *const c_char) -> i32,
    pub test_arg_vec_struct_17: extern "C" fn(i64, *const c_char) -> i32,
    pub test_two_vec_arg_13: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
    pub test_arg_struct_14: extern "C" fn(i64, *const c_char) -> i32,
    pub test_two_arg_struct_15: extern "C" fn(i64, *const c_char, *const c_char) -> i32,
    pub test_no_return: extern "C" fn(i64) -> (),
    pub free_callback: extern "C" fn(i64),
    pub index: i64,
}
