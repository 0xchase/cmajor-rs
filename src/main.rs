mod com;
mod api;

use std::ffi::{CStr, CString};

use com::*;
use api::*;

pub fn main() {
    let library = Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "../cmajor/examples/patches/Filter/Filter.cmajor";
    let contents = std::fs::read_to_string(path)
        .unwrap();

    // Library stuff

    let version = library
        .get_version()
        .to_str()
        .unwrap();

    println!("\nVersion is {}", version);

    let types = library.get_engine_types();
    unsafe {
        let types2 = CStr::from_ptr(types)
            .to_str()
            .unwrap();

        println!("Types {}", types2);
    }

    // Program stuff

    let name = CString::new("Filter.cmajor").unwrap();
    let contents2 = CString::new(contents.as_bytes()).unwrap();

    let program = library.create_program();
    let info = program.parse("Filter.cmajor", &contents);
    let data = program.get_syntax_tree("", false, true, true);

    // Engine stuff

    let name = CString::new("llvm").unwrap();
    let factory = library.create_engine_factory(name.as_ptr());

    println!("Engine:");
    factory.get_name();

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
