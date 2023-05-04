//use magnus::{ define_global_function};
pub fn say_hello() {}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}

//VALUE mSUEX_HelloWorld = rb_define_module("SUEX_HelloWorld");
//rb_define_const(mSUEX_HelloWorld, "CEXT_VERSION", GetRubyInterface("1.0.0"));
//rb_define_module_function(mSUEX_HelloWorld, "hello_world", VALUEFUNC(hello_world), 0);

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Init_RustSketchupTest() {
    say_hello();


    let name = CString::new("RustReverse").unwrap();
    let function_name = CString::new("reverse").unwrap();

    unsafe {
        let klass = rb_define_module(name.as_ptr());
        let callback = std::mem::transmute::<
            unsafe extern "C" fn(RubyValue, RubyValue) -> RubyValue,
            unsafe extern "C" fn() -> RubyValue,
        >(pub_reverse);
        rb_define_module_function(klass, function_name.as_ptr(), Some(callback), 1)
    }
}

fn hello(subject: String) -> String {
    format!("hello, {}", subject)
}

use rb_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_long;

unsafe extern "C" fn pub_reverse(_klass: RubyValue, mut input: RubyValue) -> RubyValue {
    if rb_sys::NIL_P(input) {
        // Just here to test out linking globals on msvc
        rb_raise(rb_eTypeError, "cannot be nil\0".as_ptr() as *const i8);
    }

    let ruby_string = CStr::from_ptr(rb_string_value_cstr(&mut input))
        .to_str()
        .unwrap();
    let reversed = ruby_string.to_string().chars().rev().collect::<String>();
    let reversed_cstring = CString::new(reversed).unwrap();
    let size = ruby_string.len() as c_long;

    rb_utf8_str_new(reversed_cstring.as_ptr(), size)
}

//#[magnus::init]
fn init(){
    //define_global_function("test", function!(distance, 2));
    //define_global_function("hello", magnus::function!(hello, 1));
}
