use std::ffi::CString;

use super::*;

#[repr(C)]
pub struct SyntaxTreeOptions {
    namespace_or_module: *const i8,
    include_source_locations: bool,
    include_comments: bool,
    include_function_contents: bool
}

#[repr(C)]
pub struct ProgramInterface {
    parse2: unsafe fn (&Self, filename: *const i8, file_content: *const u8, file_content_size: usize) -> *mut ChocString,
    get_syntax_tree_: unsafe fn (&Self, &SyntaxTreeOptions) -> *mut ChocString
}

impl ProgramInterface {
    pub fn parse(&self, filename: &str, file_contents: &str) -> String {
        println!("Parsing");
        let filename = CString::new(filename).unwrap();
        println!("Calling parse");
        unsafe {
            (self.parse2)(
                self,
                filename.as_ptr(),
                file_contents.as_ptr(),
                file_contents.len()
            );

            todo!()
            /*Box::from_raw((self.parse_)(
                filename.as_ptr(),
                file_contents.as_ptr(),
                file_contents.len()
            )).to_string()*/
        }
    }

    pub fn get_syntax_tree(&self, namespace_or_module: &str, include_source_locations: bool, include_comments: bool, include_function_contents: bool) -> String {
        let namespace_or_module = CString::new(namespace_or_module).unwrap();
        let options = &SyntaxTreeOptions {
            namespace_or_module: namespace_or_module.as_ptr(),
            include_source_locations,
            include_comments,
            include_function_contents
        };

        unsafe {
            Box::from_raw(
                (self.get_syntax_tree_)(self, options)
            ).to_string()
        }
    }
}

pub type ProgramPtr = *mut ProgramInterface;