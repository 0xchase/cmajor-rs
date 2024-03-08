mod com;
mod api;

use std::ffi::{CStr, CString};

use com::*;
use api::*;

pub fn main() {
    let library = Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "test.cmajor";
    let contents = std::fs::read_to_string(path)
        .unwrap();

    println!("{}", contents);

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

    let program = library.create_program();
    program.parse("test.cmajor", &contents);

    let tree = program.get_syntax_tree("", false, true, true);
    println!("Tree is {}", tree);

    // Engine stuff

    let factory = library.create_engine_factory("llvm");

    println!("Engine:");
    unsafe {
        let name = (*factory).get_name();
        let name = CStr::from_ptr(name);
        println!("Other {}", name.to_str().to_owned().unwrap());
    }

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
