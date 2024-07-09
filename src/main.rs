mod api;
mod com;
mod helpers;
mod choc;

use std::{ffi::{c_void, CStr, CString}};

use api::*;
use com::*;
use helpers::*;
use choc::*;

pub fn main() {
    Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "test_2.cmajor";
    let contents = std::fs::read_to_string(path).unwrap();

    // ===== Engine stuff =====

    println!("Creating diagnostic message list");
    let mut messages = DiagnosticMessageList::new();

    println!("Creating engine");
    let mut engine = Engine::create("").unwrap();

    println!("Creating program");
    let mut program = Program::new();

    println!("Parsing program");
    program.parse(&mut messages, path, &contents);

    // println!("Getting syntax tree");
    // let tree = program.get_syntax_tree("", true, true, true);
    // println!("Syntax tree is {}", tree);

    println!("Loading engine");
    if !engine.load(&mut messages, &program, get_external_variable, get_external_function) {
        panic!("Failed to load engine");
    }

    println!("Setting build settings");
    let settings = BuildSettings::new();
    engine.set_build_settings(&settings);

    let endpoints = engine.get_input_endpoints();
    for endpoint in endpoints {
        println!(" > Found input handle {} {}", endpoint.id, endpoint.annotation.unwrap_or(serde_json::Value::String(String::new())));
    }

    let endpoints = engine.get_output_endpoints();
    for endpoint in endpoints {
        println!(" > Found output handle {}", endpoint.id);
    }

    println!("Getting endpoint handle");
    let in_handle = engine.get_endpoint_handle("in_1").unwrap();
    let out_handle = engine.get_endpoint_handle("out_1").unwrap();
    println!("Got handles {} {}", in_handle, out_handle);

    println!("Linking engine");
    if !engine.link(&mut messages, None) {
        panic!("Failed to link engine");
    }

    println!("Create performer");
    let mut performer = engine
        .create_performer()
        .unwrap();

    println!("Set block size");
    const BLOCK_SIZE: usize = 64;
    performer.set_block_size(BLOCK_SIZE as u32);

    // ===========================

    let input = &[1.0; BLOCK_SIZE];
    let output = &mut [0.0; BLOCK_SIZE];

    println!("Set input frames");
    performer.set_input_frames(in_handle, input);

    println!("Advancing {} frames", BLOCK_SIZE);
    for _ in 0..BLOCK_SIZE {
        match  performer.advance() {
            CmajResult::Ok => (),
            CmajResult::InvalidEndpointHandle => panic!("Invalid endpoint handle"),
            CmajResult::InvalidBlockSize => panic!("Invalid block size"),
            CmajResult::TypeIndexOutOfRange => panic!("Type index out of range"),
        }
    }

    println!("Copying output frames");
    match performer.copy_output_frames(out_handle, output) {
        CmajResult::Ok => (),
        CmajResult::InvalidEndpointHandle => panic!("Invalid endpoint handle"),
        CmajResult::InvalidBlockSize => panic!("Invalid block size"),
        CmajResult::TypeIndexOutOfRange => panic!("Type index out of range"),
    }

    println!("Output frames {:?}", output);
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
