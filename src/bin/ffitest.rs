#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value {
    pub value: libc::uintptr_t,
}

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    pub fn rb_num2int(num: Value) -> libc::c_long;
    pub fn rb_int2inum(num: libc::intptr_t) -> Value;
}

fn main() {
    println!("Hello, world!");

    let x: libc::c_long = 123;
    let x_rb = unsafe { rb_int2inum(x as isize) };
    let y = unsafe { rb_num2int(x_rb) };

    println!("{}", y);
}
