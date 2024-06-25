mod api;
mod com;
mod helpers;
mod choc;

use std::ffi::{c_void, CStr, CString};

use api::*;
use com::*;
use helpers::*;
use choc::*;

pub fn main() {
    // Library::load("cmajor/x64/libCmajPerformer.so");
    Library::load("./cmajor/build/tools/CmajDLL/Release/libCmajPerformer.dylib");

    let path = "test.cmajor";
    let contents = std::fs::read_to_string(path).unwrap();

    // println!("{}", contents);

    // ===== Engine stuff =====

    println!("Creating diagnostic message list");
    let mut messages = DiagnosticMessageList::new();

    println!("Creating engine");
    let mut engine = Engine::create("").unwrap();

    println!("Creating program");
    let mut program = Program::new();

    println!("Parsing program");
    program.parse(&mut messages, path, &contents);

    println!("Getting syntax tree");
    let tree = program.get_syntax_tree("", true, true, true);

    println!("Syntax tree is {}", tree);

    println!("Loading engine");
    if !engine.load(&mut messages, &program, get_external_variable, get_external_function) {
        panic!("Failed to load engine");
    }

    println!("Linking engine");
    if !engine.link(&mut messages, None) {
        panic!("Failed to link engine");
    }

    let id = "handle_1";
    let handle = engine.get_endpoint_handle(id).unwrap();

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

pub fn get_external_variable(v: &ExternalVariable) -> Value {
    todo!()
}
pub fn get_external_function(s: *const i8, ts: Span<Type>) -> *const c_void {
    todo!()
}