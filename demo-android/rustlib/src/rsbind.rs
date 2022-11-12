use std::thread;

pub trait Services: Send + Sync {
    fn get_login_service() -> Box<dyn LoginService>;
    fn get_upload_service() -> Box<dyn UploadService>;
}

pub trait LoginService: Send + Sync {
    fn login(&self, user_name: String, pwd: String) -> Box<dyn Future>;
}

pub trait Future: Send + Sync {
    fn get(&self) -> bool;
}

pub trait UploadService: Send + Sync {
    fn upload(&self, path: String, listener: Box<dyn UploadProgress>) -> i64;
}

pub trait UploadProgress : Send + Sync {
    fn on_progress(&self, id: i64, process: i64, total: i64);
}

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