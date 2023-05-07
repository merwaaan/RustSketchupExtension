pub mod array;
pub mod numeric;
pub mod ruby;

pub use self::array::RubyArray;
pub use self::numeric::{RubyFloat, RubyInt};

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
        unsafe {
            rb_define_module_function(
                $rb_module,
                concat!($binding_name, "\0").as_ptr() as *const libc::c_char,
                $function as *const libc::c_void,
                $argc as libc::c_int,
            );
        };
    };
}
