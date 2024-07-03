use std::ffi::{c_void, CStr};

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
    set_block_size: unsafe fn(*mut *const ObjectVtable<Self>, num_frames_for_next_block: u32),
    set_input_frames: unsafe fn(
        *mut *const ObjectVtable<Self>,
        EndpointHandle,
        frame_data: *const c_void,
        num_frames: u32,
    ),
    set_input_value: unsafe fn(
        *mut *const ObjectVtable<Self>,
        EndpointHandle,
        value_data: *const c_void,
        num_frames_to_reach_value: u32,
    ),
    add_input_event: unsafe fn(
        *mut *const ObjectVtable<Self>,
        EndpointHandle,
        type_index: u32,
        event_data: *const c_void,
    ),
    copy_output_value:
        unsafe fn(*mut *const ObjectVtable<Self>, EndpointHandle, dest: *const c_void),
    copy_output_frames: unsafe fn(
        *mut *const ObjectVtable<Self>,
        EndpointHandle,
        dest: *mut c_void,
        num_frames_to_copy: u32,
    ),
    iterate_output_evens: unsafe fn(
        *mut *const ObjectVtable<Self>,
        EndpointHandle,
        context: *const c_void,
        HandleOutputEventCallback,
    ),
    advance: unsafe fn(*mut *const ObjectVtable<Self>),
    get_string_for_handle: unsafe fn(
        *mut *const ObjectVtable<Self>,
        handle: u32,
        string_length: *mut usize,
    ) -> *const i8,
    get_x_runs: unsafe fn(*mut *const ObjectVtable<Self>) -> u32,
    get_maximum_block_size: unsafe fn(*mut *const ObjectVtable<Self>) -> u32,
    get_event_buffer_size: unsafe fn(*mut *const ObjectVtable<Self>) -> u32,
    get_latency: unsafe fn(*mut *const ObjectVtable<Self>) -> f64,
    get_runtime_error: unsafe fn(*mut *const ObjectVtable<Self>) -> *const i8,
}

impl Object<PerformerInterfaceVtable> {
    pub fn set_block_size(&self, num_frames_for_next_block: u32) {
        unsafe {
            ((**self.ptr).table.set_block_size)(self.ptr, num_frames_for_next_block);
        }
    }

    pub fn set_input_frames(
        &self,
        handle: EndpointHandle,
        frame_data: *const c_void,
        num_frames: u32,
    ) {
        unsafe {
            ((**self.ptr).table.set_input_frames)(self.ptr, handle, frame_data, num_frames);
        }
    }

    pub fn set_input_value(
        &self,
        handle: EndpointHandle,
        value_data: *const c_void,
        num_frames_to_reach_value: u32,
    ) {
        unsafe {
            ((**self.ptr).table.set_input_value)(
                self.ptr,
                handle,
                value_data,
                num_frames_to_reach_value,
            );
        }
    }

    pub fn add_input_event(
        &self,
        handle: EndpointHandle,
        type_index: u32,
        event_data: *const c_void,
    ) {
        unsafe {
            ((**self.ptr).table.add_input_event)(self.ptr, handle, type_index, event_data);
        }
    }

    pub fn copy_output_value(&self, handle: EndpointHandle, dest: *const c_void) {
        unsafe {
            ((**self.ptr).table.copy_output_value)(self.ptr, handle, dest);
        }
    }

    pub fn copy_output_frames(
        &self,
        handle: EndpointHandle,
        dest: *mut c_void,
        num_frames_to_copy: u32,
    ) {
        unsafe {
            ((**self.ptr).table.copy_output_frames)(self.ptr, handle, dest, num_frames_to_copy);
        }
    }

    pub fn iterate_output_events(
        &self,
        handle: EndpointHandle,
        context: *const c_void,
        callback: HandleOutputEventCallback,
    ) {
        unsafe {
            ((**self.ptr).table.iterate_output_evens)(self.ptr, handle, context, callback);
        }
    }

    pub fn advance(&self) {
        unsafe {
            ((**self.ptr).table.advance)(self.ptr);
        }
    }

    pub fn get_string_for_handle(&self, handle: u32, string_length: &mut usize) -> String {
        unsafe {
            let ptr = ((**self.ptr).table.get_string_for_handle)(self.ptr, handle, string_length);
            CStr::from_ptr(ptr).to_str().unwrap().to_owned()
        }
    }

    pub fn get_x_runs(&self) -> u32 {
        unsafe { ((**self.ptr).table.get_x_runs)(self.ptr) }
    }

    pub fn get_maximum_block_size(&self) -> u32 {
        unsafe { ((**self.ptr).table.get_maximum_block_size)(self.ptr) }
    }

    pub fn get_event_buffer_size(&self) -> u32 {
        unsafe { ((**self.ptr).table.get_event_buffer_size)(self.ptr) }
    }

    pub fn get_latency(&self) -> f64 {
        unsafe { ((**self.ptr).table.get_latency)(self.ptr) }
    }

    pub fn get_runtime_error(&self) -> Option<String> {
        unsafe {
            let ptr = ((**self.ptr).table.get_runtime_error)(self.ptr);

            if ptr as usize != 0 {
                Some(CStr::from_ptr(ptr).to_str().unwrap().to_owned())
            } else {
                None
            }
        }
    }
}
