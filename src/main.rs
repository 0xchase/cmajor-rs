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

    let version = library
        .get_version()
        .to_str()
        .unwrap();

    println!("\nVersion is {}", version);

    println!("Creating program");
    let program = library.create_program();

    let types = library.get_engine_types();

    println!("{}", unsafe { CStr::from_ptr(types).to_str().unwrap() });

    let name = CString::new("llvm").unwrap();

    println!("Create engine factory");
    let factory = library.create_engine_factory(
        name.as_ptr()
    );

    println!("Factory {:p}", Box::into_raw(factory));

    unsafe {
        let types2 = CStr::from_ptr(types)
            .to_str()
            .unwrap();

        println!("Types {}", types2);
    }

    // let name = factory.get_name();

    // let engine = factory.create_engine(std::ptr::null());

    // println!("{}", name.to_str().unwrap());

    /*println!("Parsing program");
    program.parse("Filter.cmajor", &contents);

    println!("Getting syntax tree");
    let tree = program.get_syntax_tree("", false, false, false);
    println!("Got syntax tree {}", tree);*/
}
