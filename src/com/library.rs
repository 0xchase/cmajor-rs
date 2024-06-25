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

static SYMBOL_NAME: &[u8] = b"cmajor_getEntryPointsV10";

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
            unsafe {
                CStr::from_ptr(((**entries).get_version)(entries))
            }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn create_program() -> Object<ProgramInterfaceVtable> {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            let entries = library.get_entry_points();

            unsafe {
                let ptr = ((**entries).create_program)(entries);

                if ptr as usize == 0 {
                    panic!("Failed to create program");
                }

                Object::from(ptr)
            }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn get_engine_types() -> *const i8 {
        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            let entries = library.get_entry_points();
            unsafe {
                let ptr = ((**entries).get_engine_types)(entries);
                /*CStr::from_ptr(ptr)
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .split(" ")
                    .map(|f| f.to_owned())
                    .collect::<Vec<_>>()*/

                return ptr;
            }
        } else {
            panic!("Library not loaded");
        }
    }

    pub fn create_engine_factory(
        option: &str,
    ) -> Result<Object<EngineFactoryInterfaceVtable>, String> {
        let library = &*LIBRARY.read().unwrap();
        // let option = CString::new(option).unwrap();

        if let Some(library) = library {
            let entries = library.get_entry_points();
            unsafe {
                println!("INTENTIONALLY EXTRA ARGUMENT");
                let ptr = ((**entries).create_engine_factory)(entries, std::ptr::null());
                if ptr.is_null() {
                    Err("Failed to create engine factory".to_owned())
                } else {
                    Ok(Object::from(ptr))
                }
            }
        } else {
            panic!("Library not loaded");
        }
    }

    fn get_entry_points(&self) -> *mut *const EntryPoints {
        type Entry<'a> = libloading::Symbol<'a, unsafe extern "C" fn() -> *mut *const EntryPoints>;

        let library = &*LIBRARY.read().unwrap();
        if let Some(library) = library {
            unsafe {
                let symbol: Entry = library.library.get(SYMBOL_NAME).unwrap();
                return (symbol)();
            }
        } else {
            panic!("Library not loaded");
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct EntryPoints {
    get_version: unsafe extern "C" fn(*mut *const Self) -> *mut i8,
    create_program: unsafe extern "C" fn(*mut *const Self) -> *mut *const ObjectVtable<ProgramInterfaceVtable>,
    get_engine_types: unsafe extern "C" fn(*mut *const Self) -> *const i8,
    create_engine_factory: unsafe extern "C" fn(*mut *const Self, *const i8) -> *mut *const ObjectVtable<EngineFactoryInterfaceVtable>,
}
