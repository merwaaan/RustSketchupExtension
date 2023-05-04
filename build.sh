#!/bin/bash

cargo build && cp target/debug/rust_sketchup_test.dll extension/RustSketchupTest/RustSketchupTest.so
