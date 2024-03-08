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
pub struct ProgramInterfaceVtable {
    pub object: ObjectVtable,
    pub parse2: unsafe fn (
        *const ProgramInterface,
        filename: *const i8,
        file_content: *const i8,
        file_content_size: usize
    ) -> *mut ChocString,
    pub get_syntax_tree: unsafe fn (
        *const ProgramInterface,
        &SyntaxTreeOptions
    ) -> *mut ChocString
}

#[repr(C)]
pub struct ProgramInterface {
    vtable: *const ProgramInterfaceVtable
}

impl ProgramInterface {
    pub fn parse(&self, filename: &str, file_contents: &str) {
        let filename = CString::new(filename).unwrap();
        let contents = CString::new(file_contents).unwrap();

        unsafe {
            let string = ((*self.vtable).parse2)(
                self,
                filename.as_ptr(),
                contents.as_ptr(),
                file_contents.len()
            );

            if string != std::ptr::null_mut() {
                panic!("Error in parsing");
            }
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
            let ptr = ((*self.vtable).get_syntax_tree)(self, options);
            (*ptr).to_string()
        }
    }
}
