mod api;
mod com;
mod helpers;

use std::ffi::{c_void, CStr, CString};

use api::*;
use com::*;
use helpers::*;

pub fn main() {
    // Library::load("cmajor/x64/libCmajPerformer.so");
    Library::load("./cmajor/build/tools/CmajDLL/Release/libCmajPerformer.dylib");

    let path = "test.cmajor";
    let contents = std::fs::read_to_string(path).unwrap();

    // println!("{}", contents);

    let mut list = DiagnosticMessageList::new();
    let mut program = Program::new();
    println!("Created program");

    program.parse(&mut list, path, &contents);

    // ===== Engine stuff =====

    let mut messages = DiagnosticMessageList::new();
    let engine = Engine::create("").unwrap();

    engine.load(&mut messages, &program, None, None);

    let id = "handle_1";
    let handle = engine.get_endpoint_handle(id).unwrap();

    // let cache = CacheDatabase::new();

    // engine.link(&mut messages, cache);
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

pub fn variable_provider(v: &ExternalVariable) -> Value {

}
pub fn function_provider(&str, &[Type]) -> Value {

}