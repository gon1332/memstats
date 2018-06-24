extern crate libc;

use libc::{
    c_void,
    size_t,
    dlsym,
    RTLD_NEXT
};

use std::mem::transmute;


macro_rules! c_str {
    ($s:expr) => {
        {
            concat!($s, "\0").as_ptr() as *const i8
        }
    }
}


type MallocFunc  = extern "C" fn (size: size_t) -> *mut c_void;
type FreeFunc    = extern "C" fn (ptr: *mut c_void);
type ReallocFunc = extern "C" fn (ptr: *mut c_void, size: size_t) -> *mut c_void;


macro_rules! real_malloc {
    ($size:expr) => {
        {
            let fnptr = dlsym(RTLD_NEXT, c_str!("malloc")) as *const ();
            if fnptr.is_null() {
                panic!("dlsym failed");
            }
            let fnptr: MallocFunc = transmute(fnptr);

            fnptr($size)
        }
    }
}


macro_rules! real_free {
    ($ptr:expr) => {
        let fnptr = dlsym(RTLD_NEXT, c_str!("free")) as *const ();
        if fnptr.is_null() {
            panic!("dlsym failed");
        }
        let fnptr: FreeFunc = transmute(fnptr);

        fnptr($ptr);
    }
}


macro_rules! real_realloc {
    ($ptr:expr, $size:expr) => {
        {
            let fnptr = dlsym(RTLD_NEXT, c_str!("realloc")) as *const ();
            if fnptr.is_null() {
                panic!("dlsym failed");
            }
            let fnptr: ReallocFunc = transmute(fnptr);

            fnptr($ptr, $size)
        }
    }
}


#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    unsafe {
        libc::write(1, c_str!("malloc\n") as *mut c_void, 7);
        let addr = real_malloc!(size);
        addr
    }
}


#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe {
        libc::write(1, c_str!("free\n") as *mut c_void, 5);
        real_free!(ptr);
    }
}


#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        libc::write(1, c_str!("realloc\n") as *mut c_void, 8);
        let addr = real_realloc!(ptr, size);
        addr
    }
}
