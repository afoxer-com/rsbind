## 安装Rustup
- https://rustup.rs/

```curl https://sh.rustup.rs -sSf | sh```

## Android:
### targes
```sh
rustup target add aarch64-linux-android arm-linux-androideabi armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### NDK and SDK
```sh
brew install coreutils
brew install openjdk@11
brew install android-ndk
brew install android-studio
```

For Non-Macos system:
https://developer.android.google.cn/ndk/downloads

### Environment
```sh
export ANDROID_NDK_ROOT=/usr/local/share/android-ndk
export ANDROID_SDK_ROOT=/Users/{username}/Library/Android/sdk
export JAVA_HOME=/usr/local/opt/openjdk@11
export PATH=$ANDROID_SDK_ROOT/tools:$ANDROID_SDK_ROOT/platform-tools:$PATH
```
 
### Test if env is ok.
Create a new project： cargo new my_project --lib
Try build：cargo rustc  --target arm-linux-androideabi

## iOS:
### targets
```sh
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios x86_64-apple-ios
```

### cargo-lipo
```sh
cargo install cargo-lipo
```

### XCode
```sh
xcode-select --install
xcode-select -s /Applications/Xcode.app/Contents/Developer
xcrun --show-sdk-path -sdk iphoneos
```
