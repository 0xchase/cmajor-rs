use std::ffi::c_void;
use std::ffi::{CStr, CString};

type ChocString = c_void;

// Entry Points
pub fn get_version() -> &'static str {
    unsafe {
        let entries = get_entry_points();

        println!("Entries at {:p}", entries);
        println!("Get version at {:p}", entries.get_version);
        println!("Get program at {:p}", entries.get_program);
        println!("Get engine types at {:p}", entries.get_engine_types);
        println!("Create engine factory at {:p}", entries.create_engine_factory);

        let ptr = (entries.get_version)();
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap()
    }
}

pub fn get_program() -> Box<ProgramInterface> {
    unsafe {
        let entries = get_entry_points();
        let ptr = (entries.get_program)();
        Box::from_raw(ptr)
    }
}

pub fn get_engine_types() -> &'static str {
    unsafe {
        let entries = get_entry_points();
        let ptr = (entries.get_engine_types)();
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap()
    }
}

fn get_entry_points() -> &'static EntryPoints {
    unsafe {
        cmajor_getEntryPointsV9()
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
    }
}

#[link(name = "CmajPerformer")]
extern "C" {
    fn cmajor_getEntryPointsV9() -> *const *const EntryPoints;
}

#[repr(C)]
pub struct EntryPoints {
    get_version: unsafe fn () -> *mut i8,
    get_program: unsafe fn () -> *mut ProgramInterface,
    get_engine_types: unsafe fn () -> *const i8,
    create_engine_factory: unsafe fn (*const i8),
}

// Cache database interface

#[repr(C)]
pub struct CacheDatabaseInterface {
    store: unsafe fn (
        key: *const u8,
        data_to_save: *const c_void,
        data_size: u64
    ),
    reload: unsafe fn (
        key: *const u8,
        dest_address: *const c_void,
        dest_size: u64
    ) -> u64
}

// Engine factory interface

#[repr(C)]
pub struct EngineFactoryInterface {
    create_engine: unsafe fn (engine_creation_options: *const u8) -> *mut EngineInterface,
    get_name: unsafe fn (get_name: *const u8)
}

type EngineFactoryPtr = *mut EngineFactoryInterface;

// Engnie interface

type RequestExternalVariableFn = fn (context: c_void, external_variable: *const u8);
type RequestExternalFunctionFn = fn (context: c_void, function_name: *const u8, function_signature: *const u8);
type HandleCodeGenOutput = fn (context: c_void, generated_code: *const u8, generated_code_size: usize, main_class_name: *const u8, message_list_json: *const u8);

#[repr(C)]
pub struct EngineInterface {
    get_build_settings: unsafe fn () -> ChocString,
    set_build_settings: unsafe fn (settings: *const u8),
    load: unsafe fn (program: *mut ProgramInterface, request_variable_context: c_void, v: RequestExternalVariableFn, request_function_context: c_void, f: RequestExternalFunctionFn),
    set_external_variable: unsafe fn (name: *const u8, serialised_value_data: *const c_void, serialized_value_size: usize),
    unload: unsafe fn (),
    get_program_details: unsafe fn () -> *mut ChocString,
    get_endpoint_handle: unsafe fn (endpoint_id: *const u8),
    link: unsafe fn (*mut CacheDatabaseInterface),
    create_performer: unsafe fn () -> *mut PerformerInterface,
    get_last_build_log: unsafe fn () -> *mut ChocString,
    is_loaded: unsafe fn () -> bool,
    is_linked: unsafe fn () -> bool,
    generate_code: unsafe fn (target_type: *const u8, options: *const u8, callback_context: *mut c_void, HandleCodeGenOutput),
    get_available_code_gen_target_types: unsafe fn () -> *const u8
}

        // CString::from_raw(ptr)
            // .into_string()
            // .unwrap()

// Performer interface

type EndpointHandle = u32;
type HandleOutputEventCallback = unsafe fn (context: *const c_void, EndpointHandle, data_type_index: u32, frame_offset: u32, value_data: *const c_void, value_data_size: u32);

#[repr(C)]
pub struct PerformerInterface {
    set_block_size: unsafe fn (num_frames_for_next_block: u32),
    set_input_frames: unsafe fn (EndpointHandle, frame_data: *const c_void, num_frames: u32),
    set_input_value: unsafe fn (EndpointHandle, value_data: *const c_void, num_frames_to_reach_value: u32),
    add_input_event: unsafe fn (EndpointHandle, type_index: u32, event_data: *const c_void),
    copy_output_value: unsafe fn (EndpointHandle, dest: *const c_void),
    copy_output_frames: unsafe fn (EndpointHandle, dest: *const c_void, num_frames_to_copy: u32),
    iterate_output_evens: unsafe fn (EndpointHandle, context: *const c_void, HandleOutputEventCallback),
    advance: unsafe fn (),
    get_string_for_handle: unsafe fn (handle: u32, string_length: &mut usize),
    get_x_runs: unsafe fn () -> u32,
    get_maximum_block_size: unsafe fn () -> u32,
    get_event_buffer_size: unsafe fn () -> u32,
    get_latency: unsafe fn (),
    get_runtime_error: unsafe fn () -> *const u8,
}

type PerformerPtr = *mut PerformerInterface;

// Program Interface

#[repr(C)]
pub struct SyntaxTreeOptions {
    namespace_or_module: *const u8,
    include_source_locations: bool,
    include_comments: bool,
    include_function_contents: bool
}

#[repr(C)]
pub struct ProgramInterface {
    parse: unsafe fn (filename: *const u8, file_content: *const u8, file_content_size: usize) -> *mut ChocString,
    get_syntax_tree: unsafe fn (&SyntaxTreeOptions) -> *const ChocString
}

type ProgramPtr = *mut ProgramInterface;
