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

    // ===== Engine stuff =====

    let engine = Engine::create("llvm").unwrap();
    let performer = engine.create_performer().unwrap();
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
