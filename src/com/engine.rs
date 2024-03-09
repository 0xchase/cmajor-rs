use std::ffi::c_void;

use super::*;

// Engine interface

type RequestExternalVariableFn = fn (context: c_void, external_variable: *const u8);
type RequestExternalFunctionFn = fn (context: c_void, function_name: *const u8, function_signature: *const u8);
type HandleCodeGenOutput = fn (context: c_void, generated_code: *const u8, generated_code_size: usize, main_class_name: *const u8, message_list_json: *const u8);

#[repr(C)]
pub struct EngineInterfaceVtable {
    get_build_settings_: unsafe fn () -> *mut ChocStringVtable,
    set_build_settings: unsafe fn (settings: *const u8),
    load: unsafe fn (program: *mut *const ObjectVtable<ProgramInterfaceVtable>, request_variable_context: c_void, v: RequestExternalVariableFn, request_function_context: c_void, f: RequestExternalFunctionFn),
    set_external_variable: unsafe fn (name: *const u8, serialised_value_data: *const c_void, serialized_value_size: usize),
    unload: unsafe fn (),
    get_program_details: unsafe fn () -> *mut ChocStringVtable,
    get_endpoint_handle: unsafe fn (endpoint_id: *const u8),
    link: unsafe fn (*mut CacheDatabaseInterface),
    create_performer: unsafe fn () -> *mut PerformerInterfaceVtable,
    get_last_build_log: unsafe fn () -> *mut ChocStringVtable,
    is_loaded: unsafe fn (*const Object<Self>) -> bool,
    is_linked: unsafe fn (*const Object<Self>) -> bool,
    generate_code: unsafe fn (target_type: *const u8, options: *const u8, callback_context: *mut c_void, HandleCodeGenOutput),
    get_available_code_gen_target_types: unsafe fn () -> *const u8
}

impl Object<EngineInterfaceVtable> {
    pub fn is_loaded(&self) -> bool {
        unsafe {
            ((**self.ptr).table.is_loaded)(self)
        }
    }

    pub fn is_linked(&self) -> bool {
        unsafe {
            ((**self.ptr).table.is_linked)(self)
        }
    }

    // set_build_settings
}