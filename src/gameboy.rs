use mizu_core::GameBoy;

use crate::ruby::{array::RubyArray, numeric::RubyInt, Value, NIL};

static mut GAMEBOY: Option<GameBoy> = None;

pub fn load_rom(_rb_module: Value, rb_path: Value) -> Value {
    let gameboy = GameBoy::builder("C:/Users/Utilisateur/dev/rust-sketchup-test/tetris.gb")
        .build()
        .unwrap();

    unsafe { GAMEBOY = Some(gameboy) };

    return NIL;
}

pub fn press_button(_rb_module: Value, rb_button_name: Value) -> Value {
    // TODO

    return NIL;
}

pub fn run_frame(_rb_module: Value, rb_frame_count: Value) -> Value {
    let frame_count: i64 = rb_frame_count.into();

    let rb_screen_buffer = RubyArray::new();

    if let Some(gb) = unsafe { &mut GAMEBOY } {
        // Run

        for _ in 0..frame_count {
            gb.clock_for_frame();
        }

        // Retrieve screen data

        let screen = gb.screen_buffer();

        for pixel in screen {
            rb_screen_buffer.push(RubyInt::new(*pixel as i64));
        }
    }

    return rb_screen_buffer.into();
}
