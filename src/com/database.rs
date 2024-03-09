use std::ffi::{c_void, CStr, CString};

use super::*;

#[repr(C)]
pub struct CacheDatabaseInterfaceVtable {
    store_: unsafe fn(key: *const i8, data_to_save: *const c_void, data_size: u64),
    reload_: unsafe fn(key: *const i8, dest_address: *const c_void, dest_size: u64) -> u64,
}

impl CacheDatabaseInterfaceVtable {
    pub fn store(&self, key: &str, data: Vec<u8>) {
        let key = CString::new(key).unwrap();

        unsafe {
            (self.store_)(
                key.as_ptr(),
                data.as_ptr() as *const c_void,
                data.len() as u64,
            )
        }
    }

    pub fn reload(&self, key: &str, data: Vec<u8>) {}
}
