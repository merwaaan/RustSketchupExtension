use crate::ruby::{array::RubyArray, numeric::RubyInt, RubyString, Value, NIL};
use mizu_core::{GameBoy, JoypadButton};
use std::cell::RefCell;

thread_local!(
    static GAMEBOY: RefCell<Option<GameBoy>> = RefCell::new(Option::None);
);

pub fn load_rom(_rb_module: Value, rb_path: Value) -> Value {
    let rb_path: RubyString = rb_path.into();
    let path: String = rb_path.to_string();

    let gameboy = GameBoy::builder(&path).build().unwrap();

    GAMEBOY.with(|gb| gb.borrow_mut().replace(gameboy));

    return NIL;
}

fn get_joypad_button(rb_button_name: Value) -> JoypadButton {
    let rb_button_name: RubyString = rb_button_name.into();

    match &rb_button_name.to_string()[..] {
        "a" => JoypadButton::A,
        "b" => JoypadButton::B,
        "up" => JoypadButton::Up,
        "down" => JoypadButton::Down,
        "left" => JoypadButton::Left,
        "right" => JoypadButton::Right,
        "start" => JoypadButton::Start,
        "select" => JoypadButton::Select,
        _ => JoypadButton::A,
    }
}
pub fn press_button(_rb_module: Value, rb_button_name: Value) -> Value {
    GAMEBOY.with(|gb| {
        gb.borrow_mut().as_mut().map(|gb| {
            gb.press_joypad(get_joypad_button(rb_button_name));
            gb
        });
    });

    return NIL;
}

pub fn release_button(_rb_module: Value, rb_button_name: Value) -> Value {
    GAMEBOY.with(|gb| {
        gb.borrow_mut().as_mut().map(|gb| {
            gb.release_joypad(get_joypad_button(rb_button_name));
            gb
        });
    });

    return NIL;
}

pub fn run_frame(_rb_module: Value, rb_frame_count: Value) -> Value {
    let frame_count: i64 = rb_frame_count.into();

    let rb_screen_buffer = RubyArray::new();

    GAMEBOY.with(|gb| {
        gb.borrow_mut().as_mut().map(|gb| {
            // Run

            for _ in 0..frame_count {
                gb.clock_for_frame();
            }

            // Retrieve screen data

            let screen = gb.screen_buffer();

            for pixel in screen {
                rb_screen_buffer.push(RubyInt::new(*pixel as i64));
            }

            gb
        });
    });

    return rb_screen_buffer.into();
}
