use std::ffi::{c_void, CStr, CString};

use super::*;
use crate::{choc::*, EndpointHandle};

// Engine interface

type RequestExternalVariableFn = extern "C" fn(context: *const c_void, external_variable: *const i8);
type RequestExternalFunctionFn = extern "C" fn(context: *const c_void, function_name: *const i8, function_signature: *const i8) -> *const c_void;

type HandleCodeGenOutput = fn(
    context: *const c_void,
    generated_code: *const i8,
    generated_code_size: usize,
    main_class_name: *const i8,
    message_list_json: *const i8,
);

#[repr(C)]
pub struct EngineInterfaceVtable {
    get_build_settings: unsafe fn(*mut *const ObjectVtable<Self>) -> Object<ChocStringVtable>,
    set_build_settings: unsafe fn(*mut *const ObjectVtable<Self>, settings: *const i8),
    load: unsafe fn(
        *mut *const ObjectVtable<Self>,
        program: Object<ProgramInterfaceVtable>,
        request_variable_context: *const c_void,
        v: RequestExternalVariableFn,
        request_function_context: *const c_void,
        f: RequestExternalFunctionFn,
    ) -> *mut *const ObjectVtable<ChocStringVtable>,
    set_external_variable: unsafe fn(
        *mut *const ObjectVtable<Self>,
        name: *const i8,
        serialised_value_data: *const c_void,
        serialized_value_size: usize,
    ) -> bool,
    unload: unsafe fn(*mut *const ObjectVtable<Self>),
    get_program_details:
        unsafe fn(*mut *const ObjectVtable<Self>) -> *mut *const ObjectVtable<ChocStringVtable>,
    get_endpoint_handle:
        unsafe fn(*mut *const ObjectVtable<Self>, endpoint_id: *const i8) -> EndpointHandle,
    link: unsafe fn(
        *mut *const ObjectVtable<Self>,
        *mut *const ObjectVtable<CacheDatabaseInterfaceVtable>,
    ) -> *mut *const ObjectVtable<ChocStringVtable>,
    create_performer: unsafe fn(
        *mut *const ObjectVtable<Self>,
    ) -> *mut *const ObjectVtable<PerformerInterfaceVtable>,
    get_last_build_log:
        unsafe fn(*mut *const ObjectVtable<Self>) -> *mut *const ObjectVtable<ChocStringVtable>,
    is_loaded: unsafe fn(*mut *const ObjectVtable<Self>) -> bool,
    is_linked: unsafe fn(*mut *const ObjectVtable<Self>) -> bool,
    generate_code: unsafe fn(
        *mut *const ObjectVtable<Self>,
        target_type: *const i8,
        options: *const i8,
        callback_context: *mut c_void,
        HandleCodeGenOutput,
    ),
    get_available_code_gen_target_types: unsafe fn(*mut *const ObjectVtable<Self>) -> *const i8,
}

impl Object<EngineInterfaceVtable> {
    pub fn get_build_settings(&self) -> String {
        unsafe { ((**self.ptr).table.get_build_settings)(self.ptr).to_string() }
    }

    pub fn set_build_settings(&self, settings: &str) {
        let settings = CString::new(settings).unwrap();
        unsafe {
            ((**self.ptr).table.set_build_settings)(self.ptr, settings.as_ptr());
        }
    }

    pub fn load(&self, program: Object<ProgramInterfaceVtable>, request_variable_context: *const c_void, v: RequestExternalVariableFn, request_function_context: *const c_void, f: RequestExternalFunctionFn) -> Option<String> {
        unsafe {
            let ptr = ((**self.ptr).table.load)(
                self.ptr,
                program,
                request_variable_context,
                v,
                request_function_context,
                f,
            );

            if ptr as usize != 0 {
                Some(Object::from(ptr).to_string())
            } else {
                None
            }
        }
    }

    pub fn set_external_variable(
        &self,
        name: &str,
        serialized_value_data: *const c_void,
        size: usize,
    ) -> bool {
        let name = CString::new(name).unwrap();

        unsafe {
            ((**self.ptr).table.set_external_variable)(
                self.ptr,
                name.as_ptr(),
                serialized_value_data,
                size,
            )
        }
    }

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

    pub fn get_endpoint_handle(&self, endpoint_id: &str) -> EndpointHandle {
        let endpoint_id = CString::new(endpoint_id).unwrap();
        unsafe { ((**self.ptr).table.get_endpoint_handle)(self.ptr, endpoint_id.as_ptr()) }
    }

    pub fn link(&self, database: Option<Object<CacheDatabaseInterfaceVtable>>) -> Result<(), String> {
        unsafe {
            let ptr = if let Some(database) = database {
                ((**self.ptr).table.link)(self.ptr, database.ptr)
            } else {
                ((**self.ptr).table.link)(self.ptr, std::ptr::null_mut())
            };

            if ptr as usize == 0 {
                Ok(())
            } else {
                Err(Object::from(ptr).to_string())
            }
        }
    }

    pub fn create_performer(&self) -> Result<Object<PerformerInterfaceVtable>, ()> {
        unsafe {
            let ptr = ((**self.ptr).table.create_performer)(self.ptr);
            if ptr as usize != 0 {
                Ok(Object::from(ptr))
            } else {
                Err(())
            }
        }
    }

    pub fn get_last_build_log(&self) -> String {
        unsafe {
            let ptr = ((**self.ptr).table.get_last_build_log)(self.ptr);
            Object::from(ptr).to_string()
        }
    }

    pub fn is_loaded(&self) -> bool {
        unsafe { ((**self.ptr).table.is_loaded)(self.ptr) }
    }

    pub fn is_linked(&self) -> bool {
        unsafe { ((**self.ptr).table.is_linked)(self.ptr) }
    }

    pub fn generate_code(
        &self,
        target_type: &str,
        options: &str,
        callback_context: *mut c_void,
        handle: HandleCodeGenOutput,
    ) {
        let target_type = CString::new(target_type).unwrap();
        let options = CString::new(options).unwrap();

        unsafe {
            ((**self.ptr).table.generate_code)(
                self.ptr,
                target_type.as_ptr(),
                options.as_ptr(),
                callback_context,
                handle,
            );
        }
    }

    pub fn get_available_code_gen_target_types(&self) -> String {
        unsafe {
            let ptr = ((**self.ptr).table.get_available_code_gen_target_types)(self.ptr);
            let cstr = CStr::from_ptr(ptr);
            cstr.to_str().unwrap().to_owned()
        }
    }
}

/*fn request_external_variable_function(context: *const c_void, external_variable: *const i8) {
    panic!("Requesting external variable");
}

fn request_external_function_function(
    context: *const c_void,
    function_name: *const i8,
    external_variable: *const i8,
) {
    panic!("Requesting external function");
}*/
