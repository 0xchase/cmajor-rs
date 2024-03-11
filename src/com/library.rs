use lazy_static::lazy_static;
use std::{
    ffi::{CStr, CString},
    sync::{Arc, RwLock},
};

use super::*;

/*#[link(name = "CmajPerformer")]
extern "C" {
    fn cmajor_getEntryPointsV9() -> *const *const EntryPoints;
}*/

static SYMBOL_NAME: &[u8] = b"cmajor_getEntryPointsV9";

lazy_static! {
    static ref LIBRARY: Arc<RwLock<Option<Library>>> = Arc::from(RwLock::from(None));
}

pub struct Library {
    library: libloading::Library,
}

impl Library {
    pub fn load(path: &str) -> bool {
        unsafe {
            let library = libloading::Library::new(path).unwrap();
            *LIBRARY.write().unwrap() = Some(Self { library });
        }

        true
    }

    pub fn get_version() -> &'static CStr {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            let entries = library.get_entry_points();

            println!("Entries at {:p}", entries);
            println!("Get version at {:p}", entries.get_version);
            println!("Get program at {:p}", entries.create_program);
            println!("Get engine types at {:p}", entries.get_engine_types);
            println!(
                "Create engine factory at {:p}",
                entries.create_engine_factory
            );

            unsafe { CStr::from_ptr((entries.get_version)()) }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn create_program() -> Object<ProgramInterfaceVtable> {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            let entries = library.get_entry_points();

            unsafe {
                let ptr = (entries.create_program)();

                if ptr as usize == 0 {
                    panic!("Failed to create program");
                }

                Object::from(ptr)
            }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn get_engine_types() -> Vec<String> {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            let entries = library.get_entry_points();
            unsafe {
                let ptr = (entries.get_engine_types)();
                CStr::from_ptr(ptr)
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .split(" ")
                    .map(|f| f.to_owned())
                    .collect::<Vec<_>>()
            }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn create_engine_factory(
        option: &str,
    ) -> Result<Object<EngineFactoryInterfaceVtable>, String> {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            println!("Creating engine factory");

            let entries = library.get_entry_points();
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
                    Err(String::from("Failed to create engine factory"))
                } else {
                    Ok(Object::from(ptr))
                }
            }
        } else {
            panic!("Library not loaded");
        }
    }

    fn get_entry_points(&self) -> &'static EntryPoints {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            unsafe {
                let symbol: libloading::Symbol<
                    unsafe extern "C" fn() -> *const *const EntryPoints,
                > = library.library.get(SYMBOL_NAME).unwrap();

                (symbol)().as_ref().unwrap().as_ref().unwrap()
            }
        } else {
            panic!("Library not loaded");
        }
    }
}

#[repr(C)]
pub struct EntryPoints {
    get_version: unsafe extern "C" fn() -> *mut i8,
    create_program: unsafe extern "C" fn() -> *mut *const ObjectVtable<ProgramInterfaceVtable>,
    get_engine_types: unsafe extern "C" fn() -> *const i8,
    create_engine_factory:
        unsafe extern "C" fn(*const i8) -> *mut *const ObjectVtable<EngineFactoryInterfaceVtable>,
}
