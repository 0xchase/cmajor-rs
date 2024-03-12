use std::ffi::c_void;

use crate::EndpointHandle;
use crate::Object;
use crate::PerformerInterfaceVtable;

pub struct Performer {
    object: Object<PerformerInterfaceVtable>
}

impl Performer {
    pub fn from(object: Object<PerformerInterfaceVtable>) -> Self {
        Self { object }
    }

    pub fn set_block_size(&mut self, size: u32) {
        self.object.set_block_size(size);
    }

    pub fn set_input_frames(&mut self, handle: EndpointHandle, frames: &[f32]) {
        self.object.set_input_frames(
            handle,
            frames.as_ptr() as *const c_void,
            frames.len() as u32);
    }

    pub fn set_input_value<T>(&mut self, handle: EndpointHandle, new_value: T, num_frames_to_reach_value: u32) {
        // self.object.set_input_value(handle, value_data, num_frames_to_reach_value);
        todo!()
    }

    pub fn add_input_event<T>(&mut self, handle: EndpointHandle, type_index: u32, event_value: T) {
        todo!()
    }

    pub fn copy_output_frames(&self, handle: EndpointHandle, dest: &mut [f32]) {
        todo!()
    }

    pub fn copy_output_value<T>(&self, handle: EndpointHandle, dest: &T) {
        todo!()
    }

    pub fn iterate_output_events(&self, handle: EndpointHandle, handler: fn()) {
        todo!()
    }

    pub fn advance(&mut self) {
        todo!()
    }

    pub fn get_x_runs(&self) -> u32 {
        todo!()
    }

    pub fn get_maximum_block_size(&self) -> u32 {
        todo!()
    }

    pub fn get_event_buffer_size(&self) -> u32 {
        todo!()
    }

    pub fn get_latency(&self) -> f64 {
        todo!()
    }

    pub fn get_runtime_error(&self) -> Option<String> {
        todo!()
    }
}
