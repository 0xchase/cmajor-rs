mod database;
mod engine_factory;
mod engine;
mod library;
mod performer;
mod program;

use core::slice;
use std::ops::Deref;
use std::ffi::c_void;

pub use database::*;
pub use engine_factory::*;
pub use engine::*;
pub use library::*;
pub use performer::*;
pub use program::*;

#[repr(transparent)]
pub struct Object<T> {
    ptr: *mut *const ObjectVtable<T>
}

impl<T> Object<T> {
    pub fn from(ptr: *mut *const ObjectVtable<T>) -> Self {
        unsafe {
            let count = ((**ptr).get_reference_count)(ptr);
            println!("Starting ref {}", count);

            let count = ((**ptr).add_ref)(ptr);
            println!("Ending ref {}", count);
        }

        Self { ptr }

    }
}

impl<T> Clone for Object<T> {
    fn clone(&self) -> Self {
        unsafe {
            ((**self.ptr).add_ref)(self.ptr);
        }

        Self {
            ptr: self.ptr
        }
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
    add_ref: fn (*mut *const ObjectVtable<T>) -> i32,
    release: fn (*mut *const ObjectVtable<T>) -> i32,
    get_reference_count: fn(*mut *const ObjectVtable<T>) -> i32,
    table: T
}

#[repr(C)]
pub struct ChocStringVtable {
    begin: unsafe fn (*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8,
    end: unsafe fn (*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8
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

            let slice = slice::from_raw_parts(
                begin,
                len
            );

            let vec = slice.to_owned();

            String::from_utf8(vec).unwrap()
        }
    }
}