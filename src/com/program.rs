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
    pub parse2: unsafe fn (
        *mut *const ObjectVtable<Self>,
        filename: *const i8,
        file_content: *const i8,
        file_content_size: usize
    ) -> *mut *const ObjectVtable<ChocStringVtable>,
    pub get_syntax_tree: unsafe fn (
        *mut *const ObjectVtable<Self>,
        &SyntaxTreeOptions
    ) -> *mut *const ObjectVtable<ChocStringVtable>
}

impl Object<ProgramInterfaceVtable> {
    pub fn parse(&self, filename: &str, file_contents: &str) {
        let filename = CString::new(filename).unwrap();
        let contents = CString::new(file_contents).unwrap();

        unsafe {
            let string = ((**self.ptr).table.parse2)(
                self.ptr,
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
            let ptr = ((**self.ptr).table.get_syntax_tree)(self.ptr, options);
            Object::from(ptr).to_string()
        }
    }
}