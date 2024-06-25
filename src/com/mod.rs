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
