mod api;
mod com;

use std::ffi::{c_void, CStr, CString};

use api::*;
use com::*;

pub fn main() {
    Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "test.cmajor";
    let contents = std::fs::read_to_string(path).unwrap();

    println!("{}", contents);

    // let list = DiagnosticMessageList::new();
    let mut program = Program::new();

    // program.parse(path, &contents).;

    // ===== Engine stuff =====

    let id = "hi";
    let engine = Engine::create("llvm").unwrap();
    let handle = engine.get_endpoint_handle(id).unwrap();

    // engine.link(messages, cache);
    let mut performer = engine
        .create_performer()
        .unwrap();

    let input = &[0.0, 0.0, 0.0, 0.0];

    performer.set_input_frames(handle, input);

    for _ in 0..64 {
        performer.advance();
    }

    let output: &mut [f32; 4] = &mut [0.0, 0.0, 0.0, 0.0];
    performer.copy_output_frames(handle, output);
}

fn handle(
    context: *const c_void,
    generated_code: *const i8,
    generated_code_size: usize,
    main_class_name: *const i8,
    message_list_json: *const i8,
) {
    println!("Generate code callback");
}
