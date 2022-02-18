ORIGIN_PATH=`pwd`
cd demo-android/rustlib 
pwd
cargo run --manifest-path ../../toolchain/rsbind/Cargo.toml . android all
cd ..
./gradlew assembleDebug
cd $ORIGIN_PATH