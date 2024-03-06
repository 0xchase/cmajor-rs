mod database;
mod engine_factory;
mod engine;
mod library;
mod performer;
mod program;

use core::slice;
use std::ffi::CString;

pub use database::*;
pub use engine_factory::*;
pub use engine::*;
pub use library::*;
pub use performer::*;
pub use program::*;

#[repr(C)]
pub struct ChocString {
    begin_: unsafe fn () -> *mut u8,
    end_: unsafe fn () -> *mut u8
}

impl Drop for ChocString {
    fn drop(&mut self) {
        todo!()
    }
}

impl ToString for ChocString {
    fn to_string(&self) -> String {
        unsafe {
            let begin = (self.begin_)();
            let end = (self.end_)();
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