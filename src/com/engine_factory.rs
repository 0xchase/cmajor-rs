use std::ffi::{CStr, CString};

use super::*;

#[repr(C)]
pub struct EngineFactoryInterfaceVtable {
    object: ObjectVtable,
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
    pub fn create_engine(&self, engine_creation_options: *const i8) -> *const EngineInterface {
        unsafe {
            let ptr = ((*self.vtable).create_engine)(
                self as *const EngineFactoryInterface,
                engine_creation_options
            );

            ptr
        }
    }

    pub fn get_name(&self) -> *const i8 {
        unsafe {
            ((*self.vtable).get_name)(self)
        }
    }
}

type EngineFactoryPtr = *mut EngineFactoryInterface;
