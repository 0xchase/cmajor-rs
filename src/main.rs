mod api;
mod com;

use std::ffi::{CStr, CString};

use api::*;
use com::*;

pub fn main() {
    let library = Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "test.cmajor";
    let contents = std::fs::read_to_string(path).unwrap();

    println!("{}", contents);

    // Library stuff

    let version = library.get_version().to_str().unwrap();

    println!("\nVersion is {}", version);

    // Program stuff

    // println!("Size is {}", std::mem::size_of::<ProgramInterfaceVtable>());
    // println!("Size is {}", std::mem::size_of::<ObjectVtable<ProgramInterfaceVtable>>());
    let program = library.create_program();

    program.parse("test.cmajor", &contents);

    /*let tree = program.get_syntax_tree("", false, true, true);
    println!("Tree is {}", tree);*/

    // Engine Factory stuff

    println!("Size is {}", std::mem::size_of::<EntryPoints>());
    let types = library.get_engine_types();
    let factory = library.create_engine_factory("llvm");
    let name = factory.get_name();
    println!("Factory name is {}", name);

    // Engine stuff

    let engine = factory.create_engine("");

    let linked = engine.is_linked();
    let loaded = engine.is_loaded();
    println!("Linked: {}, Loaded: {}", linked, loaded);

    // let settings = engine.get_build_settings();
    // engine.set_build_settings(&settings);

    engine.load(&program).unwrap();

    let linked = engine.is_linked();
    let loaded = engine.is_loaded();
    println!("Linked: {}, Loaded: {}", linked, loaded);

    let details = engine.get_program_details().unwrap();

    println!("Details: {}", details);

    /*println!("\nEngine factory:");
    let name = factory.get_name();
    println!("> Factory name {}", name);

    let engine = factory.create_engine("");*/

    // println!(" > Loaded {}", factory.get_name())

    // let name = factory.get_name();

    // let engine = factory.create_engine(std::ptr::null());

    // println!("{}", name.to_str().unwrap());

    /*println!("Parsing program");
    program.parse("Filter.cmajor", &contents);

    println!("Getting syntax tree");
    let tree = program.get_syntax_tree("", false, false, false);
    println!("Got syntax tree {}", tree);*/
}
