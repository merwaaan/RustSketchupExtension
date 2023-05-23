use crate::ruby::{c_object, const_get, define_module_function};

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

    let rb_module = const_get(c_object(), "RustExtension");

    // Basic

    ruby_function!(rb_module, basic::callback_test, "binding_test", 1);

    // Polyhedron

    ruby_function!(
        rb_module,
        polyhedron::generate_polyhedron,
        "generate_polyhedron",
        0
    );

    // Physics

    ruby_function!(
        rb_module,
        physics::set_static_objects,
        "physics_set_static_objects",
        1
    );
    ruby_function!(
        rb_module,
        physics::set_dynamic_objects,
        "physics_set_dynamic_objects",
        1
    );
    ruby_function!(rb_module, physics::simulate, "physics_simulate", 1);

    // GameBoy

    ruby_function!(rb_module, gameboy::load_rom, "gameboy_load_rom", 1);
    ruby_function!(rb_module, gameboy::press_button, "gameboy_press_button", 1);
    ruby_function!(
        rb_module,
        gameboy::release_button,
        "gameboy_release_button",
        1
    );
    ruby_function!(rb_module, gameboy::run_frame, "gameboy_run_frame", 1);
}
