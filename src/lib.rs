use crate::{
    basic::callback_test,
    gameboy::{load_rom, press_button, run_frame},
    physics::{set_dynamic_objects, set_static_objects, simulate},
    polyhedron::generate_polyhedron,
    ruby::{
        rb_cObject, rb_const_get, rb_define_module_function, rb_define_module_under, rb_intern,
    },
};

pub mod basic;
pub mod gameboy;
pub mod physics;
pub mod polyhedron;
pub mod ruby;
pub mod terrain;

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

    ruby_function!(rb_module_rust, generate_polyhedron, 0);

    ruby_function!(rb_module_rust, set_static_objects, 1);
    ruby_function!(rb_module_rust, set_dynamic_objects, 1);
    ruby_function!(rb_module_rust, simulate, 1);

    ruby_function!(rb_module_rust, load_rom, 1);
    ruby_function!(rb_module_rust, press_button, 1);
    ruby_function!(rb_module_rust, run_frame, 1);
}
