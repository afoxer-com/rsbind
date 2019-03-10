## 安装Rustup
参考 https://rustup.rs/
~~curl https://sh.rustup.rs -sSf | sh~~
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

- 如果要切换版本
```sh
rustup default stable 
rustup default nightly
rustup default nightly-gnu
rustup default stable-gnu 
rustc —version
```

- 安装格式化代码工具
```sh
rustup component add rustfmt --toolchain nightly-x86_64-apple-darwin
```

## Android:

### 安装targes
```sh
rustup target add aarch64-linux-android arm-linux-androideabi armv7-linux-androideabi i686-linux-android
```

~~windows & linux~~
~~rustup target add x86_64-pc-windows-gnu~~
~~rustup target add x86_64-unknown-linux-gnu~~

### 安装NDK和SDK 
```sh
brew install coreutils
brew tap caskroom/cask;\
brew cask install android-sdk;\
brew cask install android-ndk
```
### 增加环境变量
```sh
export ANDROID_SDK=/usr/local/share/android-sdk
export ANDROID_NDK=/usr/local/share/android-ndk
export PATH=$ANDROID_SDK/tools:$ANDROID_SDK/platform-tools:$PATH
export NDK_STANDALONE_DIR=/usr/local/ndk
export PATH=$PATH:$NDK_STANDALONE_DIR/ndk-standalone-21-armeabi/bin
export PATH=$PATH:$NDK_STANDALONE_DIR/ndk-standalone-21-arm64-v8a/bin
export PATH=$PATH:$NDK_STANDALONE_DIR/ndk-standalone-21-x86/bin
```

### 创建NDK standalone
```sh
sudo mkdir -p ${NDK_STANDALONE_DIR}; \
sudo ${ANDROID_NDK}/build/tools/make_standalone_toolchain.py \
      --api 21 --force \
      --install-dir ${NDK_STANDALONE_DIR}/ndk-standalone-21-x86 --arch x86; \
sudo ${ANDROID_NDK}/build/tools/make_standalone_toolchain.py \
      --api 21 --force \
      --install-dir ${NDK_STANDALONE_DIR}/ndk-standalone-21-armeabi --arch arm; \
sudo ${ANDROID_NDK}/build/tools/make_standalone_toolchain.py \
      --api 21 --force \
      --install-dir ${NDK_STANDALONE_DIR}/ndk-standalone-21-arm64-v8a --arch arm64
```

### 增加下面的配置到文件 ~/.cargo/config
Note: change ndk standalone version to your version
```toml
[target.arm-linux-androideabi]
linker = "/usr/local/ndk/ndk-standalone-21-armeabi/bin/arm-linux-androideabi-gcc"
ar = "/usr/local/ndk/ndk-standalone-21-armeabi/bin/arm-linux-androideabi-ar"
[target.i686-linux-android]
linker = "/usr/local/ndk/ndk-standalone-21-x86/bin/i686-linux-android-gcc"
ar = "/usr/local/ndk/ndk-standalone-21-x86/bin/i686-linux-android-ar"
[target.aarch64-linux-android]
linker = "/usr/local/ndk/ndk-standalone-21-arm64-v8a/bin/aarch64-linux-android-gcc"
ar = "/usr/local/ndk/ndk-standalone-21-arm64-v8a/bin/aarch64-linux-android-ar"
[target.armv7-linux-androideabi]
linker = "/usr/local/ndk/ndk-standalone-21-armeabi/bin/arm-linux-androideabi-gcc"
ar = "/usr/local/ndk/ndk-standalone-21-armeabi/bin/arm-linux-androideabi-ar"
```
 
### 测试环境是否搭建ok
自己在一个路径下新建一个工程： cargo new my_project —lib
编译试一下：cargo rustc  —target arm-linux-androideabi

## iOS:
### 安装targets
```sh
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios x86_64-apple-ios
```

### 安装cargo-lipo
```sh
cargo install cargo-lipo
```

### xcode配置
```sh
xcode-select —install
xcode-select -s /Applications/Xcode.app/Contents/Developer
xcrun —show-sdk-path -sdk iphoneos
```
