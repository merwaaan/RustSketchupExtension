use crate::ruby::{rb_int2inum, rb_int_mul, rb_num2int, Value};

pub fn say_hello() {
    std::fs::write("C:/Users/Utilisateur/rust_extension_loaded.txt", "Hello!")
        .expect("Unable to write file");
}

pub fn multiply(x: i32, y: i32) {
    println!("Hello, world!");

    let x_rb = unsafe { rb_int2inum(x as libc::intptr_t) };
    let y_rb = unsafe { rb_int2inum(y as libc::intptr_t) };
    let xy_rb = unsafe { rb_int_mul(x_rb, y_rb) };
    let xy = unsafe { rb_num2int(xy_rb) };

    println!("{}", xy);

    std::fs::write(
        "C:/Users/Utilisateur/rust_multiplies_numbers.txt",
        xy.to_string(),
    )
    .expect("Unable to write file");
}

pub fn callback_test(_rb_module: Value) -> Value {
    return unsafe { rb_int2inum(666) };
}
