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

    // ===== Library stuff =====

    let version = Library::get_version().to_str().unwrap();

    println!("\nVersion is {}", version);

    // ===== Program stuff =====

    // println!("Size is {}", std::mem::size_of::<ProgramInterfaceVtable>());
    // println!("Size is {}", std::mem::size_of::<ObjectVtable<ProgramInterfaceVtable>>());
    let program = Library::create_program();

    program.parse("test.cmajor", &contents);

    /*let tree = program.get_syntax_tree("", false, true, true);
    println!("Tree is {}", tree);*/

    // ===== Engine Factory stuff =====

    println!("Size is {}", std::mem::size_of::<EntryPoints>());
    let types = Library::get_engine_types();
    let factory = Library::create_engine_factory("llvm").unwrap();
    let name = factory.get_name();
    println!("Factory name is {}", name);

    // ===== Engine stuff =====

    let engine = factory.create_engine("sdfkh").unwrap();

    let linked = engine.is_linked();
    let loaded = engine.is_loaded();
    println!("Linked: {}, Loaded: {}", linked, loaded);

    let settings = engine.get_build_settings();
    println!("Settings: {}", settings);

    engine.load(&program).unwrap();
    // engine.link(database).unwrap();

    let details = engine.get_program_details().unwrap();
    println!("Details: {}", details);

    let log = engine.get_last_build_log();
    println!("Log: {}", log);

    let targets = engine.get_available_code_gen_target_types();
    println!("Targets: {}", targets);

    engine.generate_code("llvm", "", std::ptr::null_mut(), handle);

    let performer = engine.create_performer().unwrap();

    // let engine = Engine::create("llvm").unwrap();
    // let performer = engine.create_performer().unwrap();
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
