use std::ffi::{c_void, CString};

use super::*;

// Engine interface

type RequestExternalVariableFn = fn (
    context: *const c_void,
    external_variable: *const i8
);

type RequestExternalFunctionFn = fn (
    context: *const c_void,
    function_name: *const i8,
    function_signature: *const i8
);

type HandleCodeGenOutput = fn (
    context: *const c_void,
    generated_code: *const i8,
    generated_code_size: usize,
    main_class_name: *const i8,
    message_list_json: *const i8
);

#[repr(C)]
pub struct EngineInterfaceVtable {
    get_build_settings: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> Object<ChocStringVtable>,
    set_build_settings: unsafe fn (
        *mut *const ObjectVtable<Self>,
        settings: *const i8
    ),
    load: unsafe fn (
        *mut *const ObjectVtable<Self>,
        program: *mut *const ObjectVtable<ProgramInterfaceVtable>,
        request_variable_context: *const c_void,
        v: RequestExternalVariableFn,
        request_function_context: *const c_void,
        f: RequestExternalFunctionFn
    ) -> *mut *const ObjectVtable<ChocStringVtable>,
    set_external_variable: unsafe fn (
        *mut *const ObjectVtable<Self>,
        name: *const u8,
        serialised_value_data: *const c_void,
        serialized_value_size: usize
    ),
    unload: unsafe fn (
        *mut *const ObjectVtable<Self>
    ),
    get_program_details: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> *mut *const ObjectVtable<ChocStringVtable>,
    get_endpoint_handle: unsafe fn (
        *mut *const ObjectVtable<Self>,
        endpoint_id: *const u8
    ),
    link: unsafe fn (
        *mut *const ObjectVtable<Self>,
        *mut CacheDatabaseInterface
    ),
    create_performer: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> *mut PerformerInterfaceVtable,
    get_last_build_log: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> *mut *const ChocStringVtable,
    is_loaded: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> bool,
    is_linked: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> bool,
    generate_code: unsafe fn (
        *mut *const ObjectVtable<Self>,
        target_type: *const u8,
        options: *const u8,
        callback_context: *mut c_void,
        HandleCodeGenOutput
    ),
    get_available_code_gen_target_types: unsafe fn (
        *mut *const ObjectVtable<Self>
    ) -> *const u8
}

impl Object<EngineInterfaceVtable> {
    pub fn get_build_settings(&self) -> String {
        unsafe {
            ((**self.ptr).table.get_build_settings)(self.ptr).to_string()
        }
    }

    pub fn set_build_settings(&self, settings: &str) {
        let settings = CString::new(settings).unwrap();
        unsafe {
            ((**self.ptr).table.set_build_settings)(self.ptr, settings.as_ptr());
        }
    }

    pub fn load(&self, program: &Object<ProgramInterfaceVtable>) -> Result<(), String> {
        unsafe {
            let request_variable_context = std::ptr::null();
            let request_function_context = std::ptr::null();
            let ptr = ((**self.ptr).table.load)(
                self.ptr,
                program.ptr,
                request_variable_context,
                request_external_variable_function,
                request_function_context,
                request_external_function_function
            );

            if ptr as usize != 0 {
                Err(Object::from(ptr).to_string())
            } else {
                Ok(())
            }

        }
    }

    // pub fn set_external_variable(&self, name: &str, serialized_value_data: *const c_void, size: usize) -> bool {}

    pub fn unload(&self) {
        unsafe {
            ((**self.ptr).table.unload)(self.ptr);
        }
    }

    pub fn get_program_details(&self) -> Option<String> {
        unsafe {
            let ptr = ((**self.ptr).table.get_program_details)(self.ptr);

            if ptr as usize == 0 {
                None
            } else {
                Some(Object::from(ptr).to_string())
            }
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe {
            ((**self.ptr).table.is_loaded)(self.ptr)
        }
    }

    pub fn is_linked(&self) -> bool {
        unsafe {
            ((**self.ptr).table.is_linked)(self.ptr)
        }
    }

    // set_build_settings
}

fn request_external_variable_function(context: *const c_void, external_variable: *const i8) {
    panic!("Requesting external variable");
}

fn request_external_function_function(context: *const c_void, function_name: *const i8, external_variable: *const i8) {
    panic!("Requesting external function");
}