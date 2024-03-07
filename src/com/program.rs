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
    object_vtable: *const ObjectVtable,
    vtable: *const ProgramInterfaceVtable
}

impl ProgramInterface {
    pub fn parse(&self, filename: &str, file_contents: &str) -> String {
        let filename = CString::new(filename).unwrap();
        let contents = CString::new(file_contents).unwrap();

        unsafe {
            println!("ParseInterfaceVtable at 0x{:x}", self.vtable as usize);
            println!("ParseInterfaceVtable at 0x{:x}", self.vtable as usize);

            let count = ((*self.object_vtable).add_ref)(self as *const ProgramInterface as *const Object);
            println!("Ref count is {}", count);

            let count = ((*self.object_vtable).add_ref)(self as *const ProgramInterface as *const Object);
            println!("Ref count is {}", count);

            let count = ((*self.object_vtable).add_ref)(self as *const ProgramInterface as *const Object);
            println!("Ref count is {}", count);

            // println!("{:#04X?}", self as *const ProgramInterface as *const u8);

            println!("ParseInterfaceVtable parse2 at 0x{:x}", (*self.vtable).parse2 as usize);
            println!("ParseInterfaceVtable get_syntax_tree at 0x{:x}", (*self.vtable).get_syntax_tree as usize);

            // println!("Calling program parse");

            /*let string = ((*self.vtable).parse2)(
                self,
                filename.as_ptr(),
                contents.as_ptr(),
                file_contents.len()
            );*/

            // println!("Calling return value to_string");

            // let string = Box::from_raw(string);
            // println!("\nChocString at {:p}", string);
            // (*string).to_string();

            String::new()

            /*(self.parse2)(
                self,
                filename.as_ptr() as *const i8,
                contents.as_ptr() as *const i8,
                file_contents.len()
            );*/

            /*Box::from_raw((self.parse_)(
                filename.as_ptr(),
                file_contents.as_ptr(),
                file_contents.len()
            )).to_string()*/
        }
    }

    pub fn get_syntax_tree(&self, namespace_or_module: &str, include_source_locations: bool, include_comments: bool, include_function_contents: bool) -> String {
        /*let namespace_or_module = CString::new(namespace_or_module).unwrap();
        let options = &SyntaxTreeOptions {
            namespace_or_module: namespace_or_module.as_ptr(),
            include_source_locations,
            include_comments,
            include_function_contents
        };

        unsafe {
            Box::from_raw(
                ((*self.vtable).get_syntax_tree)(self, options)
            ).to_string()
        }*/

        todo!()
    }
}

pub type ProgramPtr<'a> = &'a &'static ProgramInterfaceVtable;
