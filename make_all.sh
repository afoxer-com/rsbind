#!/usr/bin/env bash
./tools-java-gen/copy.sh
./tools-swift-gen/copy.sh
./template/copy_all.sh
cd tools-rsbind
cargo build