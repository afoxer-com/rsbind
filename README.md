# What's this?
- This Repository provide tools to build mobile applications in Rust.
- It generate bindings from a Rust package and packaged to android aar or iOS framework. You don't need to write jni or other ffi code with this tool.

# Step by step.
1. [Setup rust environment](/docs/env.md).
2. Install 'rsbind'. ```cargo install --git https://github.com/sidneywang/rsbind.git --force -- rsbind```
3. Create a Rust library, which contains two directory, contract and imp. You can put your interface to contract module and implemation to imp module. Expose these two modules in lib.rs.
4. Run rsbind command as below. Then the generated code will be in _gen directory and aar/framework will be in target directory.

Rsbind usage:
```sh
rsbind path-of-project android/ios/all ast/bridge/dest/header/build/all
```
- ast: generate simplified ast files with json format to _gen/ast.
- bridge: generate c methods to expose our interface to _gen/[ios/android]_bridge.
- dest: generate java/swift wrapper and c header, and then put then into a project(_gen/[ios/android]_dest).
- build: build bridge modules and copy output to dest project and then build dest project.
- all: run all the steps for binding.

# Configuration
You can create a file named Rsbind.toml to add some configuration.
```toml
[android]
rustc_param = ""
arch = ["armv7-linux-androideabi"]
arch_64 = ["aarch64-linux-android"]
arch_x86 = ["i686-linux-android"]
release = true
namespace = "com.bytedance.ee.xxx.ffi"
so_name = "demo"
ext_lib = []
features_def = ["xxxx=[]"]

[ios]
rustc_param = ""
arch_phone = ["armv7-apple-ios"]
arch_simu = ["i386-apple-ios", "x86_64-apple-ios"]
release = true
features_def = []
```

# Supported Types
- Parameters: Basic types, Callback, Vec
- Return: Basic types, Struct, Vec

supported types in Callback:
- Parameters: Basic types, Vec, Struct
- Return: Basic types.

TODO: add callback support for return types.

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
    fn test_arg_callback(arg: Box<Callback>) -> u8;
    fn test_bool(arg1: bool) -> bool;
    fn test_struct() -> StructSimple;
    fn test_struct_vec() -> Vec<StructSimple>;
}

```

----------------------

# 简单介绍
- 该库帮助开发者使用Rust语言来开发Android和iOS应用程序。
- 方式是通过简单的命令，直接生成iOS的framework以及android的aar, 其中自动生成了Rust接口对应的java绑定和swift绑定代码。省去了开发者自己动手写ffi及其转换代码的繁琐。

# 使用方式
1. [Rust环境搭建](/docs/env.md)
2. 安装rsbind。```cargo install --git https://github.com/sidneywang/rsbind.git --force -- rsbind```
3. 创建rust项目，并在项目的src目录下建立两个module，分别是contract和imp，contract用于存放Android/iOS调用的接口，而imp则是接口的实现。并需要在根目录lib.rs下将两个module开放出来。具体可以参考demo。
4. 执行rsbind命令(具体如下)，那么在A项目的target目录下就有生成的framework了。如果想要看接口，可以在A项目下_gen/[swift/java]_gen下查看文件。

错误解决：
执行rsbind，如果有Library not load的错误，在启动项中加入如下该配置即可： export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH

rsbind的使用方式：
```sh
rsbind path-of-project android/ios/all ast/bridge/dest/header/build/all
```

- ast：生成简化的ast，并以json保存在_gen/ast中
- bridge：生成暴露的c接口，并建立一个module放到_gen/[ios/android]_bridge中
- dest: 生成java、swift的wrapper代码以及c的头文件，并将工程放到_gen/[ios/android]_dest中
- header：单独诚生c header，并放到_gen/header中
- build: 编译bridge模块生成.a或者.so并拷贝到dest工程，然后编译dest工程生成最终产物。
- all: 执行所有的步骤，并生成产物。

# 编译参数配置
在module的根目录，新建Rsbind.toml。

```toml
[android]
rustc_param = ""
arch = ["armv7-linux-androideabi"]
arch_64 = ["aarch64-linux-android"]
arch_x86 = ["i686-linux-android"]
release = true
namespace = "com.bytedance.ee.xxx.ffi"
so_name = "demo"
ext_lib = []
features_def = ["xxxx=[]"]

[ios]
rustc_param = ""
arch_phone = ["armv7-apple-ios"]
arch_simu = ["i386-apple-ios", "x86_64-apple-ios"]
release = true
features_def = []
```

# 已经支持类型
- 参数： 基本类型，Callback，Vec
- 返回值：基本类型，一层的struct，Vec

Callback支持的类型
- 参数：基本类型，Vec，一层的struct
- 返回值：基本类型

待补充：返回值支持callback。

可以使用callback的getter setter来达到复杂类型的作用。

Callback和普通的类定义不同，callback的每个函数需要有&self
比如：
```rust
pub trait Callback : Sync {
    fn on_callback(&self, arg1: i32, arg2: String, arg3: bool, arg4: f32, arg5: f64) -> i32;
    fn on_callback2(&self, arg1: bool) -> bool;
    fn on_callback_complex(&self, arg1: StructSimple) -> bool;
    fn on_callback_arg_vec(&self, arg1: Vec<StructSimple>) -> bool;
    fn on_callback_arg_vec_simple(&self, arg1: Vec<String>) -> bool;
}
```

而普通的类不需要，比如：
```rust
pub trait TestContract1 {
    fn test_arg_vec(arg: Vec<String>) -> i32;
    fn test_return_vec(arg: u8) -> Vec<i32>;
    fn test_arg_callback(arg: Box<Callback>) -> u8;
    fn test_bool(arg1: bool) -> bool;
    fn test_struct() -> StructSimple;
    fn test_struct_vec() -> Vec<StructSimple>;
}

```
