use std::ffi::CStr;

use super::Value;

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    pub fn rb_str_new_cstr(str: *const libc::c_char) -> Value;
    pub fn rb_string_value_cstr(str: *const Value) -> *const libc::c_char;
}

pub struct RubyString {
    internal: Value,
}

impl RubyString {
    pub fn new(value: &str) -> Self {
        let cstr = null_terminate(value);

        Self {
            internal: unsafe { rb_str_new_cstr(cstr.as_ptr() as *const libc::c_char) },
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let cstr = rb_string_value_cstr(&self.internal);
            CStr::from_ptr(cstr).to_string_lossy().into_owned()
        }
    }
}

impl From<Value> for RubyString {
    fn from(value: Value) -> Self {
        RubyString { internal: value }
    }
}

pub fn null_terminate(string: &str) -> String {
    format!("{}\0", string)
}
