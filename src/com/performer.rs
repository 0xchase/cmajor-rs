use std::ffi::c_void;

use super::*;

type EndpointHandle = u32;
type HandleOutputEventCallback = unsafe fn(
    context: *const c_void,
    EndpointHandle,
    data_type_index: u32,
    frame_offset: u32,
    value_data: *const c_void,
    value_data_size: u32,
);

#[repr(C)]
pub struct PerformerInterfaceVtable {
    set_block_size: unsafe fn(num_frames_for_next_block: u32),
    set_input_frames: unsafe fn(EndpointHandle, frame_data: *const c_void, num_frames: u32),
    set_input_value:
        unsafe fn(EndpointHandle, value_data: *const c_void, num_frames_to_reach_value: u32),
    add_input_event: unsafe fn(EndpointHandle, type_index: u32, event_data: *const c_void),
    copy_output_value: unsafe fn(EndpointHandle, dest: *const c_void),
    copy_output_frames: unsafe fn(EndpointHandle, dest: *const c_void, num_frames_to_copy: u32),
    iterate_output_evens:
        unsafe fn(EndpointHandle, context: *const c_void, HandleOutputEventCallback),
    advance: unsafe fn(),
    get_string_for_handle: unsafe fn(handle: u32, string_length: &mut usize),
    get_x_runs: unsafe fn() -> u32,
    get_maximum_block_size: unsafe fn() -> u32,
    get_event_buffer_size: unsafe fn() -> u32,
    get_latency: unsafe fn(),
    get_runtime_error: unsafe fn() -> *const u8,
}
