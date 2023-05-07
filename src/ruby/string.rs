use super::Value;

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    pub fn rb_str_new_cstr(str: *const libc::c_char) -> Value;
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
}

pub fn null_terminate(string: &str) -> String {
    format!("{}\0", string)
}
