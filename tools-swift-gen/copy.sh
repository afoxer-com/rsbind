#!/usr/bin/env bash
swift build --configuration release
cp ./.build/release/SwiftGen ../tools-rsbind/src/ios/res/swift_gen
chmod a+x ../tools-rsbind/src/ios/res/swift_gen
