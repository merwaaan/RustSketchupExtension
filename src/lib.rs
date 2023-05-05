pub fn say_hello() {}

// #[no_mangle]
// pub extern "C" fn Init_RustSketchupTest() {
//     say_hello();
// }

#[no_mangle]
pub extern "C" fn rust_test1() {}

#[no_mangle]
pub extern "C" fn rust_test2(value: i32) -> i32 {
    value * 3
}
