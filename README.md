# 简单介绍
该库提供了rust库和Android以及iOS的绑定的工具，在开发的Rust工程中，通过简单的命令，直接生成iOS的framework以及android的aar。
详细介绍文档：https://docs.bytedance.net/doc/RQLk82pg6bjJFWc0NSBLFe

# 目录简介
| 目录            | 作用                                                                                 |
| --------------- | ------------------------------------------------------------------------------------ |
| tools-rsbind    | 这个是项目中最主要的工程，也是工具的主工程。最终通过这个目录将工具打包为二进制文件。 |
| tools-swift-gen | 这个是项目中的主要用于swift的代码生成。                                              |
| tools-java-gen | 这个是项目中的主要用于Java的代码生成。                                              |
| demo            | rust项目的示例，直接执行命令可以在demo/target下                                      |
| demo-ios        | 主要用于实验demo生成的framwork                                                       |
| template        | 一些模板项目，在生成时将模板项目解压并更改或添加其中的文件达到编译成目标文件的作用      |

# 使用方式
1. 安装rsbind。可以在tools-rsbind目录下执行cargo install --force安装。执行rsbind，看是否有错误。如果有Library not load的错误，在启动项中加入该配置即可。export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
2. 创建rust项目（假设为A），并在项目的src目录下建立两个module，分别是contract和imp，contract用于存放Android/iOS调用的接口，而imp则是接口的实现。并需要在根目录lib.rs下将两个module开放出来。具体可以参考demo。
3. 执行rsbind命令(具体如下)，那么在A项目的target目录下就有生成的framework了。如果想要看接口，可以在A项目下_gen/swift_gen下查看ffi.swift文件。

```sh
rsbind "path of project" android/ios/all ast/bridge/dest/header/build/all
```

- ast：生成简化的ast，并以json保存在_gen/ast中
- bridge：生成暴露的c接口，并建立一个module放到_gen/[ios/android]_bridge中
- dest: 生成java、swift的wrapper代码已经c的头文件，并将工程放到_gen/[ios/android]_dest中
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

# TODO
现在Vec的实现不是很高效，需要修改下。
现在函数如果不带返回值会有问题
