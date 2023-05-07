pub mod array;
pub mod numeric;
pub mod string;

pub use self::array::RubyArray;
pub use self::numeric::{RubyFloat, RubyInt};
use self::string::null_terminate;
pub use self::string::RubyString;

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    static rb_cObject: Value;

    fn rb_intern(name: *const libc::c_char) -> Id;
    fn rb_const_get(class: Value, id: Id) -> Value;
    fn rb_define_module_function(
        module: Value,
        name: *const libc::c_char,
        callback: *const libc::c_void,
        argc: libc::c_int,
    );
}

pub fn c_object() -> Value {
    unsafe { rb_cObject }
}

pub fn intern(name: &str) -> Id {
    unsafe { rb_intern(null_terminate(name).as_ptr() as *const libc::c_char) }
}

pub fn const_get(outer: Value, name: &str) -> Value {
    let id = intern(name);
    unsafe { rb_const_get(outer, id) }
}

pub fn define_module_function(
    module: Value,
    name: &str,
    callback: *const libc::c_void,
    argc: usize,
) {
    unsafe {
        rb_define_module_function(
            module,
            null_terminate(name).as_ptr() as *const libc::c_char,
            callback,
            argc as i32,
        )
    }
}

// Id

pub type Id = libc::uintptr_t;

// Value

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: libc::uintptr_t,
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        RubyInt::new(value).value()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        RubyInt::from_ruby(self).into()
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        RubyFloat::new(value).value()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        RubyFloat::from_ruby(self).into()
    }
}

// Object

pub trait Object {
    fn value(&self) -> Value;
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AnyObject {
    value: Value,
}

pub type Callback<I, O> = extern "C" fn(libc::c_int, *const AnyObject, I) -> O;

// Nil

pub const NIL: Value = Value { value: 0x08 };

#[macro_export]
macro_rules! ruby_function {
    ( $rb_module:ident, $function:expr, $binding_name:expr, $argc:expr ) => {
        define_module_function(
            $rb_module,
            $binding_name,
            $function as *const libc::c_void,
            $argc,
        );
    };
}
