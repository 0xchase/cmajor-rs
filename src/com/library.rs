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

    pub fn create_program(&self) -> Object<ProgramInterfaceVtable> {
        let entries = self.get_entry_points();

        unsafe {
            let ptr = (entries.create_program)();

            if ptr as usize == 0 {
                panic!("Failed to create program");
            }

            Object::from(ptr)
        }
    }

    pub fn get_engine_types(&self) -> *const i8 {
        let entries = self.get_entry_points();

        unsafe {
            (entries.get_engine_types)()
        }
    }

    pub fn create_engine_factory(&self, option: &str) -> Object<EngineFactoryInterfaceVtable> {
        println!("Creating engine factory");

        let entries = self.get_entry_points();
        let option = CString::new(option).unwrap();
        let option = option.as_ptr();

        unsafe {
            // DELETING THESE CAUSES A SEGFAULT
            println!("> Function at {:p}", entries.get_version);
            println!("> Function at {:p}", entries.create_program);
            println!("> Function at {:p}", entries.get_engine_types);
            println!("> Function at {:p}", entries.create_engine_factory);

            let ptr = (entries.create_engine_factory)(option);
            if ptr as usize == 0 {
                panic!("Failed to create engine factory");
            }

            Object::from(ptr)
        }
    }

    fn get_entry_points(&self) -> &'static EntryPoints {
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
    get_version: unsafe extern "C" fn () -> *mut i8,
    create_program: unsafe extern "C" fn () -> *mut *const ObjectVtable<ProgramInterfaceVtable>,
    get_engine_types: unsafe extern "C" fn () -> *const i8,
    create_engine_factory: unsafe extern "C" fn (*const i8) -> *mut *const ObjectVtable<EngineFactoryInterfaceVtable>,
}