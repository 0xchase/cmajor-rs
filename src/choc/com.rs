use crate::com::*;

#[repr(C)]
pub struct ChocStringVtable {
    begin: unsafe fn(*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8,
    end: unsafe fn(*mut *const ObjectVtable<ChocStringVtable>) -> *mut u8,
}

impl ToString for Object<ChocStringVtable> {
    fn to_string(&self) -> String {
        unsafe {
            /*println!("ChocString at {:p}", self);
            println!("ChocString vtable at {:p}", self.ptr);
            println!("ChocStringVtable begin at {:p}", (*self.ptr).table.begin);
            println!("ChocStringVtable end at {:p}", (*self.ptr).table.end);*/

            let begin = ((**self.ptr).table.begin)(self.ptr);
            let end = ((**self.ptr).table.end)(self.ptr);
            let len = end as usize - begin as usize;

            let slice = std::slice::from_raw_parts(begin, len);

            let vec = slice.to_owned();

            String::from_utf8(vec).unwrap()
        }
    }
}