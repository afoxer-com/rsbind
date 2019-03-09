#!/usr/bin/env bash

rm -rfv ../demo-iOS/demo-swift/demo-swift/rustlib.framework
cp -rv ./target/iphone_simulator/rustlib.framework  ../demo-iOS/demo-swift/demo-swift/rustlib.framework
rm -rfv ../demo-android/rustlib/rustlib-release.aar
cp -rv ./target/android/rustlib-release.aar ../demo-android/rustlib/