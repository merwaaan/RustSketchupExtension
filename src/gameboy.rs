use mizu_core::GameBoy;

use crate::ruby::{rb_ary_new, rb_ary_push, rb_int2inum, rb_num2int, Value};

static mut GAMEBOY: Option<GameBoy> = None;

pub fn load_rom(_rb_module: Value, rb_path: Value) -> Value {
    let gameboy = GameBoy::builder("C:/Users/Utilisateur/dev/rust-sketchup-test/tetris.gb")
        .build()
        .unwrap();

    unsafe { GAMEBOY = Some(gameboy) };

    return unsafe { rb_ary_new() };
}

pub fn press_button(_rb_module: Value, rb_button_name: Value) -> Value {
    // TODO

    return unsafe { rb_ary_new() };
}

pub fn run_frame(_rb_module: Value, rb_frame_count: Value) -> Value {
    let frame_count = unsafe { rb_num2int(rb_frame_count) };

    let rb_screen_buffer = unsafe { rb_ary_new() };

    unsafe {
        if let Some(gb) = &mut GAMEBOY {
            // Run

            for _ in 0..frame_count {
                gb.clock_for_frame();
            }

            // Retrieve screen data

            let screen = gb.screen_buffer();

            for pixel in screen {
                rb_ary_push(rb_screen_buffer, rb_int2inum(*pixel as libc::intptr_t));
            }
        }
    }

    return rb_screen_buffer;
}
