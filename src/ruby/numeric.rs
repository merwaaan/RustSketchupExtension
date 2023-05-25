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
        value.into()
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

impl Into<usize> for RubyInt {
    fn into(self) -> usize {
        unsafe { rb_num2int(self.internal) as usize }
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
    pub fn new(value: f64) -> Self {
        value.into()
    }

    pub fn from_ruby(rb_value: Value) -> Self {
        Self { internal: rb_value }
    }
}

impl Into<f32> for RubyFloat {
    fn into(self) -> f32 {
        unsafe { rb_num2dbl(self.internal) as f32 }
    }
}

impl From<f64> for RubyFloat {
    fn from(value: f64) -> Self {
        RubyFloat {
            internal: unsafe { rb_float_new(value) },
        }
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
