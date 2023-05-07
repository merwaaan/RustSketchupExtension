use crate::ruby::{RubyInt, Value};

pub fn say_hello() {
    std::fs::write("C:/Users/Utilisateur/rust_extension_loaded.txt", "Hello!")
        .expect("Unable to write file");
}

pub fn multiply(x: i64, y: i64) {
    let rb_x: RubyInt = x.into();
    let rb_y: RubyInt = y.into();
    let rb_xy = rb_x.multiply(rb_y);
    let xy: i64 = rb_xy.into();

    std::fs::write(
        "C:/Users/Utilisateur/rust_multiplies_numbers.txt",
        xy.to_string(),
    )
    .expect("Unable to write file");
}

pub fn callback_test(_rb_module: Value) -> Value {
    RubyInt::new(666).into()
}
