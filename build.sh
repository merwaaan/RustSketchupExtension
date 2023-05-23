#!/bin/bash

cargo build --release
cp target/release/rust_sketchup_test.dll extension/RustSketchupTest/RustSketchupTest.so
