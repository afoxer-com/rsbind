use ::imp::sample_contract_imp::*;
use ::contract::sample_contract::*;
use jni::JNIEnv;
use jni::JavaVM;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jlong, jstring, jbyteArray};
use std::os::raw::c_void;
use jni::sys::JNI_VERSION_1_6;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_bytedance_ee_bear_TestContract_test_normal(env: JNIENV, class: JClass, bool_arg: u8, u8_arg: i32, i16_arg: i32, int_arg: i32, long_arg: i64, float_arg: f32, double_arg: f64) -> i32 {
    let r_bool_arg = if bool_arg > 0 { true } else { false };
    let r_u8_arg = u8_arg as u8;
    let r_i16_arg = i16_arg as i16;
    let r_int_arg = int_arg as i32;
    let r_long_arg = long_arg as i64;
    let r_float_arg = float_arg as f32;
    let r_double_arg = double_arg as f64;
    let ret_value = TestContractImp::test_normal(r_bool_arg, r_u8_arg, r_i16_arg, r_int_arg, r_long_arg, r_float_arg, r_double_arg);
    return ret_value as i32;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_bytedance_ee_bear_TestContract_test_string(env: JNIENV, class: JClass, str_arg: jstring, int_arg: i32) -> jstring {
    let r_str_arg: String = env.get_string(str_arg).expect("Couldn't get java string!").into();
    let r_int_arg = int_arg as i32;
    let ret_value = TestContractImp::test_string(r_str_arg, r_int_arg);
    return CString::new(ret_value).unwrap().into_raw();
}