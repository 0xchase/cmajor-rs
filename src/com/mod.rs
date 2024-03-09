mod database;
mod engine;
mod engine_factory;
mod library;
mod performer;
mod program;

use core::slice;
use std::ffi::c_void;
use std::ops::Deref;

pub use database::*;
pub use engine::*;
pub use engine_factory::*;
pub use library::*;
pub use performer::*;
pub use program::*;

#[repr(transparent)]
pub struct Object<T> {
    ptr: *mut *const ObjectVtable<T>,
}

impl<T> Object<T> {
    pub fn from(ptr: *mut *const ObjectVtable<T>) -> Self {
        Self { ptr }
    }
}

impl<T> Clone for Object<T> {
    fn clone(&self) -> Self {
        unsafe {
            ((**self.ptr).add_ref)(self.ptr);
        }

        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Object<T> {
    fn drop(&mut self) {
        unsafe {
            ((**self.ptr).release)(self.ptr);
        }
    }
}

/*impl<T> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &(**self.ptr).table
        }
    }
}*/

#[repr(C)]
pub struct ObjectVtable<T> {
    add_ref: unsafe extern "C" fn(*mut *const Self) -> i32,
    release: unsafe extern "C" fn(*mut *const Self) -> i32,
    get_reference_count: unsafe extern "C" fn(*mut *const Self) -> i32,
    table: T,
}

#[repr(C)]
pub struct ChocStringVtable {
    begin: unsafe fn(*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8,
    end: unsafe fn(*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8,
}

impl ToString for Object<ChocStringVtable> {
    fn to_string(&self) -> String {
        unsafe {
            /*println!("ChocString at {:p}", self);
            println!("ChocString vtable at {:p}", self.ptr);
            println!("ChocStringVtable begin at {:p}", (*self.ptr).table.begin);
            println!("ChocStringVtable end at {:p}", (*self.ptr).table.end);*/

            let begin = ((**self.ptr).table.begin)(self.ptr);
            let end = ((**self.ptr).table.end)(self.ptr);
            let len = end as usize - begin as usize;

            let slice = slice::from_raw_parts(begin, len);

            let vec = slice.to_owned();

            String::from_utf8(vec).unwrap()
        }
    }
}
