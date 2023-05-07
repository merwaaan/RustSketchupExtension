use super::{Object, Value};

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    fn rb_int2inum(x: libc::intptr_t) -> Value;
    fn rb_num2int(x: Value) -> libc::c_long;
    fn rb_int_mul(x: Value, y: Value) -> Value;

    fn rb_float_new(num: f64) -> Value;
    fn rb_num2dbl(num: Value) -> libc::c_double;
}

// Integer

pub struct RubyInt {
    internal: Value,
}

impl RubyInt {
    pub fn new(value: i64) -> Self {
        Self {
            internal: unsafe { rb_int2inum(value as isize) },
        }
    }

    pub fn from_ruby(rb_value: Value) -> Self {
        Self { internal: rb_value }
    }

    pub fn multiply(&self, other: RubyInt) -> RubyInt {
        RubyInt {
            internal: unsafe { rb_int_mul(self.internal, other.internal) },
        }
    }
}

impl Object for RubyInt {
    fn value(&self) -> Value {
        self.internal
    }
}

impl From<i64> for RubyInt {
    fn from(value: i64) -> RubyInt {
        RubyInt {
            internal: unsafe { rb_int2inum(value as isize) },
        }
    }
}

impl Into<i64> for RubyInt {
    fn into(self) -> i64 {
        unsafe { rb_num2int(self.internal) as i64 }
    }
}

impl Into<Value> for RubyInt {
    fn into(self) -> Value {
        self.internal
    }
}

// Float

pub struct RubyFloat {
    internal: Value,
}

impl RubyFloat {
    pub fn new(x: f64) -> Self {
        Self {
            internal: unsafe { rb_float_new(x) },
        }
    }

    pub fn from_ruby(rb_value: Value) -> Self {
        Self { internal: rb_value }
    }
}

impl Into<f64> for RubyFloat {
    fn into(self) -> f64 {
        unsafe { rb_num2dbl(self.internal) as f64 }
    }
}

impl Object for RubyFloat {
    fn value(&self) -> Value {
        self.internal
    }
}
