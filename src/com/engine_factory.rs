use std::ffi::CString;

use super::*;

#[repr(C)]
pub struct EngineFactoryInterface {
    create_engine_: unsafe fn (engine_creation_options: *const i8) -> *mut EngineInterface,
    get_name_: unsafe fn () -> *mut i8
}

impl EngineFactoryInterface {
    pub fn create_engine(&self, engine_creation_options: &str) -> Box<EngineInterface> {
        let options = CString::new(engine_creation_options).unwrap();

        unsafe {
            Box::from_raw((self.create_engine_)(options.as_ptr()))

        }
    }

    pub fn get_name(&self) -> String {
        unsafe {
            CString::from_raw((self.get_name_)()).into_string().unwrap()
        }
    }
}

type EngineFactoryPtr = *mut EngineFactoryInterface;
