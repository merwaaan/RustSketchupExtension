use crate::{
    basic::callback_test,
    ruby::{
        rb_cObject, rb_const_get, rb_define_module_function, rb_define_module_under, rb_intern,
    },
};

pub mod basic;
pub mod ruby;

#[no_mangle]
pub extern "C" fn Init_RustSketchupTest() {
    // Extension loading tests

    basic::say_hello();

    basic::multiply(12, 10);

    // Rust functions exposed to Ruby

    let rb_module_rusttest = unsafe {
        rb_const_get(
            rb_cObject,
            rb_intern("RustTest\0".as_ptr() as *const libc::c_char),
        )
    };

    let rb_module_rust = unsafe {
        rb_define_module_under(rb_module_rusttest, "Rust\0".as_ptr() as *const libc::c_char)
    };

    ruby_function!(rb_module_rust, callback_test, 1);
}
