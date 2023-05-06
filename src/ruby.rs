#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Value {
    pub value: libc::uintptr_t,
}

pub type Id = libc::uintptr_t;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AnyObject {
    value: Value,
}

pub type Callback<I, O> = extern "C" fn(libc::c_int, *const AnyObject, I) -> O;

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

    // Numbers

    pub fn rb_int2inum(x: libc::intptr_t) -> Value;
    pub fn rb_num2int(x: Value) -> libc::c_long;
    pub fn rb_int_mul(x: Value, y: Value) -> Value;

    pub fn rb_float_new(num: f64) -> Value;
    pub fn rb_num2dbl(num: Value) -> libc::c_double;

    // Arrays

    pub fn rb_ary_new() -> Value;
    pub fn rb_ary_new_capa(capacity: libc::c_long) -> Value;
    pub fn rb_ary_push(array: Value, item: Value) -> Value;
}

#[macro_export]
macro_rules! ruby_function {
    ( $rb_module:ident, $function:expr, $argc:expr ) => {
        unsafe {
            rb_define_module_function(
                $rb_module,
                concat!(stringify!($function), "\0").as_ptr() as *const libc::c_char,
                $function as *const libc::c_void,
                $argc as libc::c_int,
            );
        };
    };
}
