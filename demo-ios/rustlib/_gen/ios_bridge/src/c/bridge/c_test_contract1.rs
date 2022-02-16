use contract::test_contract1::*;
use imp::test_contract1_imp::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Struct_StructSimple {
    pub arg1: i32,
    pub arg2: i8,
    pub arg3: String,
    pub arg4: bool,
    pub arg5: f32,
    pub art6: f64,
}
impl From<StructSimple> for Struct_StructSimple {
    fn from(origin: StructSimple) -> Self {
        Struct_StructSimple {
            arg1: origin.arg1,
            arg2: origin.arg2,
            arg3: origin.arg3,
            arg4: origin.arg4,
            arg5: origin.arg5,
            art6: origin.art6,
        }
    }
}
#[no_mangle]
pub extern "C" fn test_contract1_test_byte(arg: i8) -> i8 {
    let r_arg = arg as u8;
    let ret_value = TestContract1Imp::test_byte(r_arg);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_byte_i8(arg: i8) -> i8 {
    let r_arg = arg as i8;
    let ret_value = TestContract1Imp::test_byte_i8(r_arg);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_vec(arg: *const c_char) -> i32 {
    let c_str_arg: &CStr = unsafe { CStr::from_ptr(arg) };
    let c_str_arg: &str = c_str_arg.to_str().unwrap();
    let r_arg = serde_json::from_str(&c_str_arg.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_arg_vec(r_arg);
    ret_value as i32
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec(arg: i8) -> *mut c_char {
    let r_arg = arg as u8;
    let ret_value = TestContract1Imp::test_return_vec(r_arg);
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_arg_callback(arg: test_contract1_Callback_Model) -> i8 {
    pub struct arg_struct {
        pub on_callback_u8: extern "C" fn(i64, i8) -> i8,
        pub on_callback_i8: extern "C" fn(i64, i8) -> i8,
        pub on_callback: extern "C" fn(i64, i32, *const c_char, i32, f32, f64) -> i32,
        pub on_callback2: extern "C" fn(i64, i32) -> i32,
        pub on_callback_complex: extern "C" fn(i64, *const c_char) -> i32,
        pub on_callback_arg_vec: extern "C" fn(i64, *const c_char) -> i32,
        pub on_callback_arg_vec_simple: extern "C" fn(i64, *const c_char) -> i32,
        pub on_empty_callback: extern "C" fn(i64) -> (),
        pub free_callback: extern "C" fn(i64),
        pub index: i64,
    }
    impl Callback for arg_struct {
        fn on_callback_u8(&self, arg1: u8) -> u8 {
            let c_arg1 = arg1 as i8;
            let fn_on_callback_u8 = self.on_callback_u8;
            let result = fn_on_callback_u8(self.index, c_arg1);
            let s_result = result as u8;
            s_result
        }
        fn on_callback_i8(&self, arg1: i8) -> i8 {
            let c_arg1 = arg1 as i8;
            let fn_on_callback_i8 = self.on_callback_i8;
            let result = fn_on_callback_i8(self.index, c_arg1);
            let s_result = result as i8;
            s_result
        }
        fn on_callback(&self, arg1: i32, arg2: String, arg3: bool, arg4: f32, arg5: f64) -> i32 {
            let c_arg1 = arg1 as i32;
            let c_arg2 = CString::new(arg2).unwrap().into_raw();
            let c_arg3 = if arg3 { 1 } else { 0 };
            let c_arg4 = arg4 as f32;
            let c_arg5 = arg5 as f64;
            let fn_on_callback = self.on_callback;
            let result = fn_on_callback(self.index, c_arg1, c_arg2, c_arg3, c_arg4, c_arg5);
            unsafe { CString::from_raw(c_arg2) };
            let s_result = result as i32;
            s_result
        }
        fn on_callback2(&self, arg1: bool) -> bool {
            let c_arg1 = if arg1 { 1 } else { 0 };
            let fn_on_callback2 = self.on_callback2;
            let result = fn_on_callback2(self.index, c_arg1);
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn on_callback_complex(&self, arg1: StructSimple) -> bool {
            let c_tmp_arg1 = serde_json::to_string(&Struct_StructSimple::from(arg1));
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_on_callback_complex = self.on_callback_complex;
            let result = fn_on_callback_complex(self.index, c_arg1);
            unsafe { CString::from_raw(c_arg1) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn on_callback_arg_vec(&self, arg1: Vec<StructSimple>) -> bool {
            let c_tmp_vec_arg1 = arg1
                .into_iter()
                .map(|each| Struct_StructSimple::from(each))
                .collect::<Vec<Struct_StructSimple>>();
            let c_tmp_arg1 = serde_json::to_string(&c_tmp_vec_arg1);
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_on_callback_arg_vec = self.on_callback_arg_vec;
            let result = fn_on_callback_arg_vec(self.index, c_arg1);
            unsafe { CString::from_raw(c_arg1) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn on_callback_arg_vec_simple(&self, arg1: Vec<String>) -> bool {
            let c_tmp_arg1 = serde_json::to_string(&arg1);
            let c_arg1 = CString::new(c_tmp_arg1.unwrap()).unwrap().into_raw();
            let fn_on_callback_arg_vec_simple = self.on_callback_arg_vec_simple;
            let result = fn_on_callback_arg_vec_simple(self.index, c_arg1);
            unsafe { CString::from_raw(c_arg1) };
            let s_result = if result > 0 { true } else { false };
            s_result
        }
        fn on_empty_callback(&self) -> () {
            let fn_on_empty_callback = self.on_empty_callback;
            let result = fn_on_empty_callback(self.index);
        }
    }
    impl Drop for arg_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    let r_arg = Box::new(arg_struct {
        on_callback_u8: arg.on_callback_u8,
        on_callback_i8: arg.on_callback_i8,
        on_callback: arg.on_callback,
        on_callback2: arg.on_callback2,
        on_callback_complex: arg.on_callback_complex,
        on_callback_arg_vec: arg.on_callback_arg_vec,
        on_callback_arg_vec_simple: arg.on_callback_arg_vec_simple,
        on_empty_callback: arg.on_empty_callback,
        free_callback: arg.free_callback,
        index: arg.index,
    });
    let ret_value = TestContract1Imp::test_arg_callback(r_arg);
    ret_value as i8
}
#[no_mangle]
pub extern "C" fn test_contract1_test_bool(arg1: i32) -> i32 {
    let r_arg1 = if arg1 > 0 { true } else { false };
    let ret_value = TestContract1Imp::test_bool(r_arg1);
    if ret_value {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn test_contract1_test_struct() -> *mut c_char {
    let ret_value = TestContract1Imp::test_struct();
    let json_ret = serde_json::to_string(&Struct_StructSimple::from(ret_value));
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_struct_vec() -> *mut c_char {
    let ret_value = TestContract1Imp::test_struct_vec();
    let ret_value = ret_value
        .into_iter()
        .map(|each| Struct_StructSimple::from(each))
        .collect::<Vec<Struct_StructSimple>>();
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_two_string(
    arg1: *const c_char,
    arg2: *const c_char,
) -> *mut c_char {
    let c_str_arg1: &CStr = unsafe { CStr::from_ptr(arg1) };
    let c_str_arg1: &str = c_str_arg1.to_str().unwrap();
    let r_arg1: String = c_str_arg1.to_owned();
    let c_str_arg2: &CStr = unsafe { CStr::from_ptr(arg2) };
    let c_str_arg2: &str = c_str_arg2.to_str().unwrap();
    let r_arg2: String = c_str_arg2.to_owned();
    let ret_value = TestContract1Imp::test_two_string(r_arg1, r_arg2);
    CString::new(ret_value).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_u8(input: *const c_char) -> *mut c_char {
    let c_str_input: &CStr = unsafe { CStr::from_ptr(input) };
    let c_str_input: &str = c_str_input.to_str().unwrap();
    let r_input = serde_json::from_str(&c_str_input.to_owned()).unwrap();
    let ret_value = TestContract1Imp::test_return_vec_u8(r_input);
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_return_vec_i8(input: *const c_char) -> *mut c_char {
    let c_str_input: &CStr = unsafe { CStr::from_ptr(input) };
    let c_str_input: &str = c_str_input.to_str().unwrap();
    let r_input = serde_json::from_str(&c_str_input.to_owned()).unwrap();
    let mut ret_value = TestContract1Imp::test_return_vec_i8(r_input);
    let json_ret = serde_json::to_string(&ret_value);
    CString::new(json_ret.unwrap()).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn test_contract1_test_no_return() {
    TestContract1Imp::test_no_return();
}
#[repr(C)]
pub struct test_contract1_Callback_Model {
    pub on_callback_u8: extern "C" fn(i64, i8) -> i8,
    pub on_callback_i8: extern "C" fn(i64, i8) -> i8,
    pub on_callback: extern "C" fn(i64, i32, *const c_char, i32, f32, f64) -> i32,
    pub on_callback2: extern "C" fn(i64, i32) -> i32,
    pub on_callback_complex: extern "C" fn(i64, *const c_char) -> i32,
    pub on_callback_arg_vec: extern "C" fn(i64, *const c_char) -> i32,
    pub on_callback_arg_vec_simple: extern "C" fn(i64, *const c_char) -> i32,
    pub on_empty_callback: extern "C" fn(i64) -> (),
    pub free_callback: extern "C" fn(i64),
    pub index: i64,
}
