use std::ffi::c_void;
use std::ffi::{CStr, CString};

use super::*;

/*#[link(name = "CmajPerformer")]
extern "C" {
    fn cmajor_getEntryPointsV9() -> *const *const EntryPoints;
}*/

static SYMBOL_NAME: &[u8] = b"cmajor_getEntryPointsV9";

pub struct Library {
    lib: libloading::Library
}

impl Library {
    pub fn load(path: &str) -> Self {
        unsafe {
            Self {
                lib: libloading::Library::new(path)
                    .unwrap()
            }
        }
    }

    pub fn get_version(&self) -> &'static CStr {
        let entries = self.get_entry_points();

        println!("Entries at {:p}", entries);
        println!("Get version at {:p}", entries.get_version);
        println!("Get program at {:p}", entries.create_program);
        println!("Get engine types at {:p}", entries.get_engine_types);
        println!("Create engine factory at {:p}", entries.create_engine_factory);

        unsafe {
            CStr::from_ptr(
                (entries.get_version)()
            )
        }
    }

    pub fn get_engine_types(&self) -> *const i8 {
        let entries = self.get_entry_points();

        unsafe {
            (entries.get_engine_types)()
        }
    }

    pub fn create_program(&self) -> Box<ProgramInterface> {
        let entries = self.get_entry_points();

        unsafe {
            let ptr = (entries.create_program)();

            if ptr as usize == 0 {
                panic!("Failed to create program");
            } else {
                println!("Program at {:p}", ptr);
                Box::from_raw(ptr)
            }
        }
    }

    pub fn create_engine_factory(&self, option: &str) -> *const EngineFactoryInterface {
        println!("Creating engine factory");

        let entries = self.get_entry_points();
        let option = CString::new(option).unwrap();

        unsafe {
            println!("Calling create_engine_factory");
            let ptr = (entries.create_engine_factory)(std::ptr::null());

            if ptr as usize == 0 {
                panic!("Failed to create engine factory");
            } else {
                ptr
            }
        }
    }

    fn get_entry_points(&self) -> &EntryPoints {
        unsafe {
            let symbol: libloading::Symbol<unsafe extern fn () -> *const *const EntryPoints> = self
                .lib
                .get(SYMBOL_NAME)
                .unwrap();

            (symbol)()
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap()
        }
    }
}

#[repr(C)]
pub struct EntryPoints {
    get_version: unsafe fn () -> *mut i8,
    create_program: unsafe fn () -> *mut ProgramInterface,
    get_engine_types: unsafe fn () -> *const i8,
    create_engine_factory: unsafe fn (*const i8) -> *mut EngineFactoryInterface,
}

/*impl Drop for Library {
    fn drop(&mut self) {
        todo!()
    }
}*/

// Entry Points
/*fn get_version() -> &'static str {
    unsafe {
        let entries: &EntryPoints = get_entry_points();

        println!("Entries at {:p}", entries);
        println!("Get version at {:p}", entries.get_version);
        println!("Get program at {:p}", entries.create_program);
        println!("Get engine types at {:p}", entries.get_engine_types);
        println!("Create engine factory at {:p}", entries.create_engine_factory);

        let ptr = (entries.get_version)();
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap()
    }
}*/

/*fn get_entry_points() -> &'static EntryPoints {
    unsafe {
        cmajor_getEntryPointsV9()
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
    }
}*/
