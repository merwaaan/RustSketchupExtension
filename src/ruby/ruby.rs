use super::{Id, Value};

// TODO move out
#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    pub static rb_cObject: Value;

    // Misc.

    pub fn rb_intern(name: *const libc::c_char) -> Id;
    //pub fn rb_intern2(name: *const libc::c_char, length: libc::c_long) -> Id;
    pub fn rb_const_get(class: Value, id: Id) -> Value;
    pub fn rb_define_module_under(module: Value, name: *const libc::c_char) -> Value;
    pub fn rb_define_module_function(
        module: Value,
        name: *const libc::c_char,
        callback: *const libc::c_void,
        argc: libc::c_int,
    );

    // Strings

    pub fn rb_str_new_cstr(str: *const libc::c_char) -> Value;
}
