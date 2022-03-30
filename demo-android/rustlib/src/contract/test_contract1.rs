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
