use std::borrow::Borrow;
use std::thread;
use std::thread::JoinHandle;
use contract::test_contract1::{Future, LoginService, Services, UploadProgress, UploadService};

pub struct ServiceHolder {}

impl Services for ServiceHolder {
    fn get_login_service() -> Box<dyn LoginService> {
        Box::new(LoginServiceImp {})
    }

    fn get_upload_service() -> Box<dyn UploadService> {
        Box::new(UploadServiceImp {})
    }
}

pub struct LoginServiceImp {}

impl LoginService for LoginServiceImp {
    fn login(&self, user_name: String, pwd: String) -> Box<dyn Future> {
        struct FutureImp {
            pub user_name: String,
            pub pwd: String,
        }
        impl Future for FutureImp {
            fn get(&self) -> bool {
                let handle = thread::spawn(move || {
                    // do your login
                    true
                });
                handle.join().unwrap()
            }
        }
        Box::new(FutureImp { user_name, pwd })
    }
}

pub struct UploadServiceImp {}

impl UploadService for UploadServiceImp {
    fn upload(&self, path: String, listener: Box<dyn UploadProgress>) -> i64 {
        thread::spawn(move || {
            // doing uploading
            listener.on_progress(99999, 10, 12345);
        });

        99999
    }
}