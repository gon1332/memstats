extern crate libc;

use libc::{
    c_void,
    size_t,
};


#[macro_use]
mod defs;
use defs::{real_malloc, real_free, real_realloc};


//
// Hook definitions
//

// malloc hook.
#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe {
        libc::write(1, c_str!("malloc\n") as *mut c_void, 7);
        let addr = real_malloc(size);
        addr
    }
}


// free hook.
#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe {
        libc::write(1, c_str!("free\n") as *mut c_void, 5);
        real_free(ptr);
    }
}


// realloc hook.
#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        libc::write(1, c_str!("realloc\n") as *mut c_void, 8);
        let addr = real_realloc(ptr, size);
        addr
    }
}
