[![Build](https://github.com/sidneywang/rsbind/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/sidneywang/rsbind/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/rsbind.svg)](https://crates.io/crates/rsbind)
# What's this?
- This Repository provide tools to bind rust interface with other language and export library artifact directly.
- It generate bindings from a Rust package and packaged to android aar or iOS lib. You don't need to write jni or other ffi code with this tool.
  
The tool may be useful for the people who want to use Rust as a cross-platform language and exporting multiple artifact for each platform.

# Step by step.
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
pub trait YourContract {
    fn test_simple(arg1: i32, arg2: String) -> String;
    fn test_callback(arg: Box<dyn Callback>);
    fn test_struct() -> StructSimple;
}

pub trait Callback {
    fn on_callback(arg1: i64, arg2: String);
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
YourContract.test_callback(new Callback(){
       void on_callback(long arg1, String arg2) {
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
arch = ["armv7-linux-androideabi"]
arch_64 = ["aarch64-linux-android"]
arch_x86 = ["i686-linux-android"]
release = true
namespace = "com.afoxer.xxx.ffi"
so_name = "demo"
ext_lib = []
features_def = ["xxxx=[]"]

[ios]
rustc_param = ""
arch_phone = ["armv7-apple-ios"]
arch_simu = ["i386-apple-ios", "x86_64-apple-ios"]
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
- Return: Basic types, Struct, Vec

supported types in Callback:
- Parameters: Basic types, Vec, Struct
- Return: Basic types.

## Note:
These is not supported yet:
- Callback in return type.

In Callback:
- Return String.

Will support it in near future.

It is different to define a callback and a normal trait.
It should contains &self in every callback but not in normal trait.

Callback:
```rust
pub trait Callback : Sync {
    fn on_callback(&self, arg1: i32, arg2: String, arg3: bool, arg4: f32, arg5: f64) -> i32;
    fn on_callback2(&self, arg1: bool) -> bool;
    fn on_callback_complex(&self, arg1: StructSimple) -> bool;
    fn on_callback_arg_vec(&self, arg1: Vec<StructSimple>) -> bool;
    fn on_callback_arg_vec_simple(&self, arg1: Vec<String>) -> bool;
}
```

Normal trait:
```rust
pub trait TestContract1 {
    fn test_arg_vec(arg: Vec<String>) -> i32;
    fn test_return_vec(arg: u8) -> Vec<i32>;
    fn test_arg_callback(arg: Box<dyn Callback>) -> u8;
    fn test_bool(arg1: bool) -> bool;
    fn test_struct() -> StructSimple;
    fn test_struct_vec() -> Vec<StructSimple>;
}

```
