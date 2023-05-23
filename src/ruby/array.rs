use super::{Object, Value};

#[link(name = "x64-msvcrt-ruby270")]
extern "C" {
    fn rb_ary_new() -> Value;
    fn rb_ary_new_capa(capacity: libc::c_long) -> Value;
    fn rb_ary_push(array: Value, item: Value) -> Value;
    fn rb_ary_entry(array: Value, index: libc::c_long) -> Value;
}

pub struct RubyArray {
    internal: Value,
}

impl RubyArray {
    pub fn new() -> Self {
        Self {
            internal: unsafe { rb_ary_new() },
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal: unsafe { rb_ary_new_capa(capacity as libc::c_long) },
        }
    }

    pub fn length(&self) -> usize {
        unsafe {
            let rarray: *const RArray = std::mem::transmute(self.internal);
            let flags = (*rarray).basic.flags;

            let length = if flags & (RArrayEmbed::Flag as libc::size_t) == 0 {
                (*rarray).as_.heap.len
            } else {
                ((flags as i64 >> RArrayEmbed::LenShift as i64)
                    & (RArrayEmbed::LenMask as i64 >> RArrayEmbed::LenShift as i64))
                    as libc::c_long
            };

            length as usize
        }
    }

    pub fn at(&self, index: usize) -> Value {
        unsafe { rb_ary_entry(self.internal, index as libc::c_long) }
    }

    pub fn push<T: Object>(&self, item: T) {
        unsafe { rb_ary_push(self.internal, item.value()) };
    }
}

impl Object for RubyArray {
    fn value(&self) -> Value {
        self.internal
    }
}

impl Into<Value> for RubyArray {
    fn into(self) -> Value {
        self.internal
    }
}

impl From<Value> for RubyArray {
    fn from(value: Value) -> Self {
        RubyArray { internal: value }
    }
}

// Array-related structures from https://github.com/danielpclark/rutie

#[derive(Debug, PartialEq)]
#[repr(C)]
enum RArrayEmbed {
    Flag = (1 << 13) as isize,
    LenMask = ((1 << 16) | (1 << 15)) as isize,
    LenShift = (12 + 3) as isize,
}

#[repr(C)]
struct RArrayAs {
    heap: RArrayHeap,
}

#[repr(C)]
struct RArrayHeap {
    len: libc::c_long,
    // Really, this is a union but value is the largest item.
    value: libc::uintptr_t,
    ptr: libc::uintptr_t,
}

#[repr(C)]
pub struct RBasic {
    pub flags: libc::uintptr_t,
    pub klass: libc::uintptr_t,
}

#[repr(C)]
struct RArray {
    basic: RBasic,
    as_: RArrayAs,
}
