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

#[repr(i32)]
pub enum CmajResult {
    Ok = 0,
    InvalidEndpointHandle = -1,
    InvalidBlockSize = -2,
    TypeIndexOutOfRange = -3
}

#[repr(transparent)]
pub struct Object<T> {
    pub ptr: *mut *const ObjectVtable<T>,
}

impl<T> Object<T> {
    pub fn from(ptr: *mut *const ObjectVtable<T>) -> Self {
        Self { ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T: Clone> Clone for Object<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

impl<T> Drop for Object<T> {
    fn drop(&mut self) {
        unsafe {
            ((**self.ptr).release)(self.ptr);
        }
    }
}

impl<T> Deref for Object<T> {
    type Target = *const ObjectVtable<T>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.ptr
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
    pub add_ref: unsafe extern "C" fn(*mut *const Self) -> i32,
    pub release: unsafe extern "C" fn(*mut *const Self) -> i32,
    pub get_reference_count: unsafe extern "C" fn(*mut *const Self) -> i32,
    pub table: T,
}
