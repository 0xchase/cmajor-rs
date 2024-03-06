mod com;
mod api;

use com::*;
use api::*;

pub fn main() {
    let library = Library::load("cmajor/x64/libCmajPerformer.so");

    let path = "../cmajor/examples/patches/Filter/Filter.cmajor";
    let contents = std::fs::read_to_string(path)
        .unwrap();

    println!("Version is {}", library.get_version());
    println!("Engine types are {}", library.get_engine_types());

    println!("Creating program");
    let program = library.create_program();

    println!("Parsing program");
    program.parse("Filter.cmajor", &contents);

    println!("Getting syntax tree");
    let tree = program.get_syntax_tree("", false, false, false);
    println!("Got syntax tree {}", tree);
}
