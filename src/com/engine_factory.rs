use std::ffi::{CStr, CString};

use super::*;

#[repr(C)]
pub struct EngineFactoryInterface {
    create_engine_: unsafe fn (*const Self, engine_creation_options: *const i8) -> *mut EngineInterface,
    get_name_: unsafe fn (*const Self) -> *const i8
}

impl EngineFactoryInterface {
    pub fn create_engine(&self, engine_creation_options: *const i8) -> Box<EngineInterface> {
        unsafe {
            println!("Creating engine {:p}", self.create_engine_);
            let ptr = (self.create_engine_)(
                self as *const EngineFactoryInterface,
                engine_creation_options
            );

            Box::from_raw(ptr)
        }
    }

    pub fn get_name(&self) -> *const i8 {
        println!("Creating engine {:p}", self.get_name_);
        unsafe {
            (self.get_name_)(self as *const EngineFactoryInterface)
        }
    }
}

type EngineFactoryPtr = *mut EngineFactoryInterface;
