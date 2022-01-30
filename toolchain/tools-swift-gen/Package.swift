// swift-tools-version:4.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "swift_gen",
    dependencies: [
//        .package(url: "https://github.com/gilt/SwiftPoet.git", from: "1.9.0"),
    ],
    targets: [
        .target(
            name: "SwiftGen",
            dependencies: ["SwiftGenCore"]),
        .target(
            name: "SwiftGenCore",
            dependencies: ["SwiftPoet"]),
        .target(
            name: "SwiftPoet"
        ),
    ]
)
