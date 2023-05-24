#!/bin/bash

if [ $1 = "debug" ]
then
   BUILD_FLAG=""
else
   BUILD_FLAG="--release"
fi

cargo build $BUILD_FLAG

cp "target/$1/rust_sketchup_test.dll" "extension/RustSketchupTest/RustSketchupTest.so"
