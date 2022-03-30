use c::bridge::common::*;
use contract::test_contract1::*;
use contract::test_contract1::*;
use contract::test_contract1::*;
use imp::test_contract1_imp::*;
use imp::test_contract1_imp::*;
use imp::test_contract1_imp::*;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::RwLock;
lazy_static! {
    static ref CALLBACK_HASHMAP: Arc<RwLock<HashMap<i64, CallbackEnum>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref CALLBACK_INDEX: Arc<RwLock<i64>> = Arc::new(RwLock::new(0));
}
#[no_mangle]
pub extern "C" fn test_contract1_Services_get_login_service() -> test_contract1_LoginService_Model {
    let result = ServiceHolder::get_login_service();
    let callback_index = {
        let mut global_index = CALLBACK_INDEX.write().unwrap();
        let mut index = *global_index;
        if index == i64::MAX {
            *global_index = 0;
            index = 0;
        } else {
            *global_index = index + 1;
            index = index + 1;
        }
        index
    };
    let r_result = box_to_model_LoginService(callback_index);
    (*CALLBACK_HASHMAP.write().unwrap()).insert(callback_index, CallbackEnum::LoginService(result));
    r_result
}
#[no_mangle]
pub extern "C" fn test_contract1_Services_get_upload_service() -> test_contract1_UploadService_Model
{
    let result = ServiceHolder::get_upload_service();
    let callback_index = {
        let mut global_index = CALLBACK_INDEX.write().unwrap();
        let mut index = *global_index;
        if index == i64::MAX {
            *global_index = 0;
            index = 0;
        } else {
            *global_index = index + 1;
            index = index + 1;
        }
        index
    };
    let r_result = box_to_model_UploadService(callback_index);
    (*CALLBACK_HASHMAP.write().unwrap())
        .insert(callback_index, CallbackEnum::UploadService(result));
    r_result
}
#[repr(C)]
pub struct test_contract1_LoginService_Model {
    pub login: extern "C" fn(i64, *const c_char, *const c_char) -> test_contract1_Future_Model,
    pub free_callback: extern "C" fn(i64),
    pub free_ptr: extern "C" fn(*mut i8, i32),
    pub index: i64,
}
#[repr(C)]
pub struct test_contract1_Future_Model {
    pub get: extern "C" fn(i64) -> i32,
    pub free_callback: extern "C" fn(i64),
    pub free_ptr: extern "C" fn(*mut i8, i32),
    pub index: i64,
}
#[repr(C)]
pub struct test_contract1_UploadService_Model {
    pub upload: extern "C" fn(i64, *const c_char, test_contract1_UploadProgress_Model) -> i64,
    pub free_callback: extern "C" fn(i64),
    pub free_ptr: extern "C" fn(*mut i8, i32),
    pub index: i64,
}
#[repr(C)]
pub struct test_contract1_UploadProgress_Model {
    pub on_progress: extern "C" fn(i64, i64, i64, i64) -> (),
    pub free_callback: extern "C" fn(i64),
    pub free_ptr: extern "C" fn(*mut i8, i32),
    pub index: i64,
}
enum CallbackEnum {
    LoginService(Box<dyn LoginService>),
    Future(Box<dyn Future>),
    UploadService(Box<dyn UploadService>),
    UploadProgress(Box<dyn UploadProgress>),
}
fn box_to_model_LoginService(callback_index: i64) -> test_contract1_LoginService_Model {
    impl test_contract1_LoginService_Model {
        pub extern "C" fn ret_login(
            index: i64,
            user_name: *const c_char,
            pwd: *const c_char,
        ) -> test_contract1_Future_Model {
            let c_str_user_name: &CStr = unsafe { CStr::from_ptr(user_name) };
            let c_str_user_name: &str = c_str_user_name.to_str().unwrap();
            let r_user_name: String = c_str_user_name.to_owned();
            let c_str_pwd: &CStr = unsafe { CStr::from_ptr(pwd) };
            let c_str_pwd: &str = c_str_pwd.to_str().unwrap();
            let r_pwd: String = c_str_pwd.to_owned();
            let mut callback_index = 0;
            let mut result: Option<Box<dyn Future>> = None;
            let final_result = {
                let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
                let ret_callback = callback_hashmap.get(&index);
                match ret_callback {
                    Some(ret_callback) => {
                        if let CallbackEnum::LoginService(ret_callback) = ret_callback {
                            result = Some(ret_callback.login(r_user_name, r_pwd));
                            callback_index = {
                                let mut global_index = CALLBACK_INDEX.write().unwrap();
                                let mut index = *global_index;
                                if index == i64::MAX {
                                    *global_index = 0;
                                    index = 0;
                                } else {
                                    *global_index = index + 1;
                                    index = index + 1;
                                }
                                index
                            };
                            let r_result = box_to_model_Future(callback_index);
                            r_result
                        } else {
                            panic!("Callback doesn't match for index: {}", index);
                        }
                    }
                    None => {
                        panic!("No callback found for index: {}", index);
                    }
                }
            };
            (*CALLBACK_HASHMAP.write().unwrap())
                .insert(callback_index, CallbackEnum::Future(result.unwrap()));
            final_result
        }
        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }
        pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
            demo_free_rust(buffer, size as u32)
        }
    }
    test_contract1_LoginService_Model {
        login: test_contract1_LoginService_Model::ret_login,
        free_callback: test_contract1_LoginService_Model::ret_free_callback,
        free_ptr: test_contract1_LoginService_Model::ret_free_ptr,
        index: callback_index,
    }
}
fn box_to_model_Future(callback_index: i64) -> test_contract1_Future_Model {
    impl test_contract1_Future_Model {
        pub extern "C" fn ret_get(index: i64) -> i32 {
            let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
            let ret_callback = callback_hashmap.get(&index);
            match ret_callback {
                Some(ret_callback) => {
                    if let CallbackEnum::Future(ret_callback) = ret_callback {
                        let mut result = ret_callback.get();
                        let r_result = if result { 1 } else { 0 };
                        r_result
                    } else {
                        panic!("Callback doesn't match for index: {}", index);
                    }
                }
                None => {
                    panic!("No callback found for index: {}", index);
                }
            }
        }
        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }
        pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
            demo_free_rust(buffer, size as u32)
        }
    }
    test_contract1_Future_Model {
        get: test_contract1_Future_Model::ret_get,
        free_callback: test_contract1_Future_Model::ret_free_callback,
        free_ptr: test_contract1_Future_Model::ret_free_ptr,
        index: callback_index,
    }
}
fn box_to_model_UploadService(callback_index: i64) -> test_contract1_UploadService_Model {
    impl test_contract1_UploadService_Model {
        pub extern "C" fn ret_upload(
            index: i64,
            path: *const c_char,
            listener: test_contract1_UploadProgress_Model,
        ) -> i64 {
            let c_str_path: &CStr = unsafe { CStr::from_ptr(path) };
            let c_str_path: &str = c_str_path.to_str().unwrap();
            let r_path: String = c_str_path.to_owned();
            let r_listener = model_to_box_UploadProgress(listener);
            let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
            let ret_callback = callback_hashmap.get(&index);
            match ret_callback {
                Some(ret_callback) => {
                    if let CallbackEnum::UploadService(ret_callback) = ret_callback {
                        let mut result = ret_callback.upload(r_path, r_listener);
                        let r_result = result as i64;
                        r_result
                    } else {
                        panic!("Callback doesn't match for index: {}", index);
                    }
                }
                None => {
                    panic!("No callback found for index: {}", index);
                }
            }
        }
        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }
        pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
            demo_free_rust(buffer, size as u32)
        }
    }
    test_contract1_UploadService_Model {
        upload: test_contract1_UploadService_Model::ret_upload,
        free_callback: test_contract1_UploadService_Model::ret_free_callback,
        free_ptr: test_contract1_UploadService_Model::ret_free_ptr,
        index: callback_index,
    }
}
fn box_to_model_UploadProgress(callback_index: i64) -> test_contract1_UploadProgress_Model {
    impl test_contract1_UploadProgress_Model {
        pub extern "C" fn ret_on_progress(index: i64, id: i64, process: i64, total: i64) -> () {
            let r_id = id as i64;
            let r_process = process as i64;
            let r_total = total as i64;
            let callback_hashmap = &*CALLBACK_HASHMAP.read().unwrap();
            let ret_callback = callback_hashmap.get(&index);
            match ret_callback {
                Some(ret_callback) => {
                    if let CallbackEnum::UploadProgress(ret_callback) = ret_callback {
                        let mut result = ret_callback.on_progress(r_id, r_process, r_total);
                        let r_result = result;
                        r_result
                    } else {
                        panic!("Callback doesn't match for index: {}", index);
                    }
                }
                None => {
                    panic!("No callback found for index: {}", index);
                }
            }
        }
        pub extern "C" fn ret_free_callback(index: i64) {
            (*CALLBACK_HASHMAP.write().unwrap()).remove(&index);
        }
        pub extern "C" fn ret_free_ptr(buffer: *mut i8, size: i32) {
            demo_free_rust(buffer, size as u32)
        }
    }
    test_contract1_UploadProgress_Model {
        on_progress: test_contract1_UploadProgress_Model::ret_on_progress,
        free_callback: test_contract1_UploadProgress_Model::ret_free_callback,
        free_ptr: test_contract1_UploadProgress_Model::ret_free_ptr,
        index: callback_index,
    }
}
fn model_to_box_LoginService(
    callback_model: test_contract1_LoginService_Model,
) -> Box<dyn LoginService> {
    pub struct LoginService_struct {
        pub login: extern "C" fn(i64, *const c_char, *const c_char) -> test_contract1_Future_Model,
        pub free_callback: extern "C" fn(i64),
        pub free_ptr: extern "C" fn(*mut i8, i32),
        pub index: i64,
    }
    impl LoginService for LoginService_struct {
        fn login(&self, user_name: String, pwd: String) -> Box<dyn Future> {
            let c_user_name = CString::new(user_name).unwrap().into_raw();
            let c_pwd = CString::new(pwd).unwrap().into_raw();
            let fn_login = self.login;
            let result = fn_login(self.index, c_user_name, c_pwd);
            unsafe { CString::from_raw(c_user_name) };
            unsafe { CString::from_raw(c_pwd) };
            let r_result = model_to_box_Future(result);
            r_result
        }
    }
    impl Drop for LoginService_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    Box::new(LoginService_struct {
        login: callback_model.login,
        free_callback: callback_model.free_callback,
        free_ptr: callback_model.free_ptr,
        index: callback_model.index,
    })
}
fn model_to_box_Future(callback_model: test_contract1_Future_Model) -> Box<dyn Future> {
    pub struct Future_struct {
        pub get: extern "C" fn(i64) -> i32,
        pub free_callback: extern "C" fn(i64),
        pub free_ptr: extern "C" fn(*mut i8, i32),
        pub index: i64,
    }
    impl Future for Future_struct {
        fn get(&self) -> bool {
            let fn_get = self.get;
            let result = fn_get(self.index);
            let r_result = if result > 0 { true } else { false };
            r_result
        }
    }
    impl Drop for Future_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    Box::new(Future_struct {
        get: callback_model.get,
        free_callback: callback_model.free_callback,
        free_ptr: callback_model.free_ptr,
        index: callback_model.index,
    })
}
fn model_to_box_UploadService(
    callback_model: test_contract1_UploadService_Model,
) -> Box<dyn UploadService> {
    pub struct UploadService_struct {
        pub upload: extern "C" fn(i64, *const c_char, test_contract1_UploadProgress_Model) -> i64,
        pub free_callback: extern "C" fn(i64),
        pub free_ptr: extern "C" fn(*mut i8, i32),
        pub index: i64,
    }
    impl UploadService for UploadService_struct {
        fn upload(&self, path: String, listener: Box<dyn UploadProgress>) -> i64 {
            let callback_index = {
                let mut global_index = CALLBACK_INDEX.write().unwrap();
                let mut index = *global_index;
                if index == i64::MAX {
                    *global_index = 0;
                    index = 0;
                } else {
                    *global_index = index + 1;
                    index = index + 1;
                }
                index
            };
            {
                (*CALLBACK_HASHMAP.write().unwrap())
                    .insert(callback_index, CallbackEnum::UploadProgress(listener));
            }
            let c_path = CString::new(path).unwrap().into_raw();
            let c_listener = box_to_model_UploadProgress(callback_index);
            let fn_upload = self.upload;
            let result = fn_upload(self.index, c_path, c_listener);
            unsafe { CString::from_raw(c_path) };
            let r_result = result as i64;
            r_result
        }
    }
    impl Drop for UploadService_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    Box::new(UploadService_struct {
        upload: callback_model.upload,
        free_callback: callback_model.free_callback,
        free_ptr: callback_model.free_ptr,
        index: callback_model.index,
    })
}
fn model_to_box_UploadProgress(
    callback_model: test_contract1_UploadProgress_Model,
) -> Box<dyn UploadProgress> {
    pub struct UploadProgress_struct {
        pub on_progress: extern "C" fn(i64, i64, i64, i64) -> (),
        pub free_callback: extern "C" fn(i64),
        pub free_ptr: extern "C" fn(*mut i8, i32),
        pub index: i64,
    }
    impl UploadProgress for UploadProgress_struct {
        fn on_progress(&self, id: i64, process: i64, total: i64) -> () {
            let c_id = id as i64;
            let c_process = process as i64;
            let c_total = total as i64;
            let fn_on_progress = self.on_progress;
            let result = fn_on_progress(self.index, c_id, c_process, c_total);
        }
    }
    impl Drop for UploadProgress_struct {
        fn drop(&mut self) {
            let free_callback = self.free_callback;
            free_callback(self.index)
        }
    }
    Box::new(UploadProgress_struct {
        on_progress: callback_model.on_progress,
        free_callback: callback_model.free_callback,
        free_ptr: callback_model.free_ptr,
        index: callback_model.index,
    })
}
