use std::ffi::{c_char, CStr};
use std::path::Path;

pub struct DB {}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    println!("Before Null");
    if s.is_null() {
        return 0;
    }
    println!("Before CStr");
    let s = CStr::from_ptr(s);
    println!("Before len");
    s.to_bytes().len()
}

#[no_mangle]
pub extern "C" fn rust_open(path: *const c_char) -> *mut rocksdb::DB {
    let rust_path_str = unsafe { CStr::from_ptr(path).to_string_lossy().into_owned() };
    let path = Path::new(&rust_path_str);
    let db = rocksdb::DB::open_default(path);
    let db = db.unwrap();
    let boxed = Box::new(db);
    Box::into_raw(boxed)
}
