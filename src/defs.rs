extern crate libc;

use libc::{
    c_void,
    size_t,
    dlsym,
    RTLD_NEXT
};

use std::mem::transmute;


//
// Macro Definitions
//
macro_rules! c_str {
    ($s:expr) => {
        {
            concat!($s, "\0").as_ptr() as *const i8
        }
    }
}


//
// Type Definitions
//
type MallocFunc  = extern "C" fn (size: size_t) -> *mut c_void;
type FreeFunc    = extern "C" fn (ptr: *mut c_void);
type ReallocFunc = extern "C" fn (ptr: *mut c_void, size: size_t) -> *mut c_void;


//
// Public Function Definitions
//

// Grab the malloc function pointer and call it.
pub unsafe fn real_malloc(size: size_t) -> *mut c_void {
    let fnptr = dlsym(RTLD_NEXT, c_str!("malloc")) as *const ();
    if fnptr.is_null() {
        panic!("dlsym failed");
    }
    let fnptr: MallocFunc = transmute(fnptr);

    fnptr(size)
}


// Grab the free function pointer and call it.
pub unsafe fn real_free(ptr: *mut c_void) {
    let fnptr = dlsym(RTLD_NEXT, c_str!("free")) as *const ();
    if fnptr.is_null() {
        panic!("dlsym failed");
    }
    let fnptr: FreeFunc = transmute(fnptr);

    fnptr(ptr);
}


// Grab the realloc function pointer and call it.
pub unsafe fn real_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let fnptr = dlsym(RTLD_NEXT, c_str!("realloc")) as *const ();
    if fnptr.is_null() {
        panic!("dlsym failed");
    }
    let fnptr: ReallocFunc = transmute(fnptr);

    fnptr(ptr, size)
}
