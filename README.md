
[![Test](https://github.com/sidneywang/rsbind/actions/workflows/build.yml/badge.svg)](https://github.com/sidneywang/rsbind/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/rsbind.svg)](https://crates.io/crates/rsbind)
## Rsbind
Rsbind provide tools to bind rust trait with other language and export library artifact directly. It also help you invoke rust services just like you write it in native language.     
It generate bindings from a Rust package and packaged to android aar or iOS lib or other library artifact. You don't need to write jni or other ffi code with this tool.   
  
The tool may be useful for the people who want to use Rust as a cross-platform language and exporting multiple artifact for each platform.

## Quick Start
Suppose you are writing two services in Rust and invoke from iOS and Android.  
First you need write several Rust traits stands for your services.   
```rust
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
```
Then you can implement your trait to achive your services.

```rust
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
```

After that, run rsbind command to generate iOS and Android library artifact.
```shell
cargo run . android all
cargo run . ios all
```

Then with iOS library, you can invoke service from swift directly.
```java
let future = loginService.login(user_name: "sidney.wang", pwd: "88888888")
let result = future.get();
print("login result = \(result)")

class Listener : UploadProgress {
    func onProgress(id: Int64, process: Int64, total: Int64) {
        print("Progress is \(process)/\(total)")
    }
}
let uploadService = RustLib.newServices().getUploadService();
uploadService.upload(path: "to/your/path", listener: Listener())

```

In iOS, it is very similar, just run swift code.

## Step by step.
1. [Setup rust environment](/docs/env.md).
2. Install 'rsbind'. ```cargo install --git https://github.com/sidneywang/rsbind.git --force -- rsbind```
3. Create a Rust library, which contains two mod, contract and imp. There are two structures you can arrange.
- First structure:  
![alt First structure picture](https://raw.githubusercontent.com/sidneywang/rsbind/main/docs/first_structure.jpg)
- Second structure:  
![alt Second structure picture](https://raw.githubusercontent.com/sidneywang/rsbind/main/docs/second_structure.jpg)

 You can put your interface to contract module and implemation to imp module. Expose these two modules in lib.rs.
```rust
// such as your code in contract dir as below:
pub trait YourContract : Send + Sync {
    fn test_simple(arg1: i32, arg2: String) -> String;
    fn test_callback(arg: Box<dyn Callback>);
    fn test_struct() -> StructSimple;
    fn test_return_callback() -> Box<dyn Callback>;
}

pub trait Callback : Send + Sync {
    fn on_callback(&self, arg1: i64, arg2: String);
}

pub struct StructSimple {
    pub arg3: String,
    pub arg4: bool,
}
```

```rust
// Your implementation is as below
pub struct YourImplemetation {}

impl YourContract for YourImplemetation {
    fn test_simple(arg1: i32, arg2: String) -> String {
        format!("Your test_simple result is {}_{}", arg1, arg2)
    }

    fn test_callback(arg: Box<dyn Callback>) {
        arg.on_callback(123i64, "hello callback".to_owned());
    }

    fn test_struct() -> StructSimple {
        StructSimple {
            arg1: "struct".to_owned(),
            arg2: true
        }
    }

    fn test_return_callback() -> Box<dyn Callback> {
        struct Instance{}
        impl Callback for Instance {
            fn on_callback(&self, arg1: i64, arg2: String) {

            }
        }
        Box::new(Instance{})
    }
}
```

4. Run rsbind command as below. Then the generated code will be in _gen directory and aar/framework will be in target directory.

Rsbind usage:
```sh
rsbind path-of-project android/ios/mac/jar/all  ast/bridge/artifact/header/build/all
```
- ast: generate simplified ast files with json format to _gen/ast.
- bridge: generate c methods to expose our interface to _gen/[ios/android/mac/jar]_bridge.
- artifact: generate java/swift wrapper and c header, and then put then into a project(_gen/[ios/android/mac/jar]_artifact).
- build: build bridge modules and copy output to artifact project and then build artifact project.
- all: run all the steps for binding.

5. It will generate java files packaged in aar or cocoapods lib, then you can integrated them to your android/iOS project and call the functions.
For android, you can call like as below:
```java
YourContract instance = RustLib.newYourContract();
instance.testCallback(new Callback(){
       void onCallback(long arg1, String arg2) {
           // do your things.
       }
})
```
Swift is very similar.

# Configuration
You can create a file named Rsbind.toml to add some configuration.
```toml
[android]
rustc_param = ""
arch = ["armv7-linux-androideabi", "aarch64-linux-android", "i686-linux-android"]
release = true
namespace = "com.afoxer.xxx.ffi"
so_name = "demo"
ext_lib = []
features_def = ["xxxx=[]"]

[ios]
rustc_param = ""
arch = ["armv7-apple-ios", "i386-apple-ios", "x86_64-apple-ios"]
release = true
features_def = []

[mac]
rustc_param = ""
release = true
features_def = []

[jar]
rustc_param = ""
release = true
namespace = "com.afoxer.xxx.ffi"
so_name = "demo"
#ext_lib = []
#features_def = ["xxxx=[]"]

```

# Supported Types
- Parameters: Basic types, Callback, Vec
- Return: Basic types, Struct, Callback, Vec

supported types in Callback:
- Parameters: Basic types, Vec, Struct
- Return: Basic types, Vec.

Will support it in near future.

It is different to define a callback and a normal trait.
It should contains &self in every callback but not in normal trait.

Callback:
```rust
pub trait Callback : Send + Sync {
    fn on_callback(&self, arg1: i32, arg2: String, arg3: bool, arg4: f32, arg5: f64) -> i32;
    fn on_callback2(&self, arg1: bool) -> bool;
    fn on_callback_complex(&self, arg1: StructSimple) -> bool;
    fn on_callback_arg_vec(&self, arg1: Vec<StructSimple>) -> bool;
    fn on_callback_arg_vec_simple(&self, arg1: Vec<String>) -> bool;
}
```

Normal trait:
```rust
pub trait TestContract1 : Send + Sync {
    fn test_arg_vec(arg: Vec<String>) -> i32;
    fn test_return_vec(arg: u8) -> Vec<i32>;
    fn test_arg_callback(arg: Box<dyn Callback>) -> u8;
    fn test_bool(arg1: bool) -> bool;
    fn test_struct() -> StructSimple;
    fn test_struct_vec() -> Vec<StructSimple>;
}

```
