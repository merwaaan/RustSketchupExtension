#!/bin/bash

cargo build && cp target/debug/librust_sketchup_test.dylib extension/RustSketchupTest/RustSketchupTest.bundle
