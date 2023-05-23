#!/bin/bash

cargo build --release
cp target/debug/rust_sketchup_test.dll extension/RustSketchupTest/RustSketchupTest.so
