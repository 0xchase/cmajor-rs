mod database;
mod engine_factory;
mod engine;
mod library;
mod performer;
mod program;

use core::slice;

pub use database::*;
pub use engine_factory::*;
pub use engine::*;
pub use library::*;
pub use performer::*;
pub use program::*;

#[repr(C)]
pub struct ObjectVtable {
    add_ref: fn (*const Object) -> i32,
    release: fn (*const Object) -> i32,
    get_reference_count: fn(*const Object) -> i32,
}

#[repr(C)]
pub struct Object;

#[repr(C)]
pub struct ChocStringVtable {
    begin: unsafe fn (*const ChocString) -> *mut u8,
    end: unsafe fn (*const ChocString) -> *mut u8
}

#[repr(C)]
pub struct ChocString {
    // object_vtable: *const ObjectVtable,
    vtable: *const ChocStringVtable
}

impl Drop for ChocString {
    fn drop(&mut self) {
    }
}

impl ToString for ChocString {
    fn to_string(&self) -> String {
        unsafe {
            println!("ChocString at {:p}", self);
            println!("ChocString vtable at {:p}", self.vtable);
            println!("ChocStringVtable begin at {:p}", (*self.vtable).begin);
            println!("ChocStringVtable end at {:p}", (*self.vtable).end);

            let begin = ((*self.vtable).begin)(self);
            let end = ((*self.vtable).end)(self);
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