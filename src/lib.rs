pub fn say_hello() {}

#[no_mangle]
pub extern "C" fn Init_RustSketchupTest() {
    say_hello();
}
