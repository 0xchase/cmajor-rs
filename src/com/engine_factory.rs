use std::ffi::{c_void, CStr, CString};

use super::*;

#[repr(C)]
pub struct EngineFactoryInterfaceVtable {
    create_engine: unsafe fn(
        *mut *const ObjectVtable<Self>,
        engine_creation_options: *const i8,
    ) -> *mut *const ObjectVtable<EngineInterfaceVtable>,
    get_name: unsafe fn(*mut *const ObjectVtable<Self>) -> *const i8,
}

impl Object<EngineFactoryInterfaceVtable> {
    pub fn create_engine(&self, engine_creation_options: &str) -> Result<Object<EngineInterfaceVtable>, String> {
        let options = CString::new(engine_creation_options).unwrap();
        unsafe {
            let ptr = ((**self.ptr).table.create_engine)(self.ptr, std::ptr::null());

            if ptr == std::ptr::null_mut() {
                Err(String::from("Failed to create engine"))
            } else {
                Ok(Object::from(ptr))
            }
        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let ptr = ((**self.ptr).table.get_name)(self.ptr);
            let string = CStr::from_ptr(ptr);

            string.to_str().unwrap().to_string()
        }
    }
}
