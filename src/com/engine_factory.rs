use std::ffi::{CStr, CString};

use super::*;

#[repr(C)]
pub struct EngineFactoryInterfaceVtable {
    create_engine: unsafe fn (
        *const EngineFactoryInterface,
        engine_creation_options: *const i8
    ) -> *mut EngineInterface,
    get_name: unsafe fn (
        *const EngineFactoryInterface
    ) -> *const i8
}

#[repr(C)]
pub struct EngineFactoryInterface {
    vtable: *const EngineFactoryInterfaceVtable
}

impl EngineFactoryInterface {
    pub fn create_engine(&self, engine_creation_options: *const i8) -> Box<EngineInterface> {
        unsafe {
            let ptr = ((*self.vtable).create_engine)(
                self as *const EngineFactoryInterface,
                engine_creation_options
            );

            Box::from_raw(ptr)
        }
    }

    pub fn get_name(&self) -> *const i8 {
        // println!("Creating engine {:p}", self.get_name);
        unsafe {
            ((*self.vtable).get_name)(self)
        }
    }
}

type EngineFactoryPtr = *mut EngineFactoryInterface;
