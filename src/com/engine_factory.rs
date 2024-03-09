use std::ffi::{c_void, CStr, CString};

use super::*;

#[repr(C)]
pub struct EngineFactoryInterfaceVtable {
    create_engine: unsafe fn (
        &Object<Self>,
        engine_creation_options: *const i8
    ) -> *mut *const ObjectVtable<EngineInterfaceVtable>,
    get_name: unsafe fn (
        &Object<Self>
    ) -> *const i8
}

impl Object<EngineFactoryInterfaceVtable> {
    pub fn create_engine(&self, engine_creation_options: &str) -> Object<EngineInterfaceVtable> {
        let options = CString::new(engine_creation_options).unwrap();
        unsafe {
            let ptr = ((**self.ptr).table.create_engine)(
                self,
                std::ptr::null()
            );

            Object::from(ptr)
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let ptr = ((**self.ptr).table.get_name)(self);
            let string = CStr::from_ptr(ptr);

            string
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
