#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Value {
    pub value: libc::uintptr_t,
}

pub type Id = libc::uintptr_t;

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    pub static rb_cObject: Value;

    // Misc.
    pub fn rb_intern(name: *const libc::c_char) -> Id;
    //pub fn rb_intern2(name: *const libc::c_char, length: libc::c_long) -> Id;
    pub fn rb_const_get(class: Value, id: Id) -> Value;
    pub fn rb_define_module_under(outer: Value, name: *const libc::c_char);

    // Number stuff
    pub fn rb_int2inum(x: libc::intptr_t) -> Value;
    pub fn rb_num2int(x: Value) -> libc::c_long;
    pub fn rb_int_mul(x: Value, y: Value) -> Value;
}

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

#[no_mangle]
pub extern "C" fn Init_RustSketchupTest() {
    say_hello();
    multiply(12, 10);

    // std::fs::write("C:/Users/Utilisateur/rust_debug.txt", x.to_string())
    //     .expect("Unable to write file");

    let rb_module_lindale = unsafe {
        rb_const_get(
            rb_cObject,
            rb_intern("Lindale\0".as_ptr() as *const libc::c_char),
        )
    };

    let rb_module_rustsketchuptest = unsafe {
        rb_const_get(
            rb_module_lindale,
            rb_intern("RustSketchupTest\0".as_ptr() as *const libc::c_char),
        )
    };

    let rb_module_rust = unsafe {
        rb_define_module_under(
            rb_module_rustsketchuptest,
            "Rust\0".as_ptr() as *const libc::c_char,
        )
    };
}
