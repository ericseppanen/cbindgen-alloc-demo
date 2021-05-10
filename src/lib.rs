#![no_std]
#![cfg_attr(feature = "alloc", feature(default_alloc_error_handler))]

/// There are two different versions of the needs_alloc sub-module.
///
/// This version sets up a global allocator, so it can use `Box`, `Vec`, etc.
#[cfg(feature = "alloc")]
mod needs_alloc {
    extern crate alloc;
    use super::TestStruct;
    use alloc::boxed::Box;

    use libc_alloc::LibcAlloc;

    #[global_allocator]
    static ALLOCATOR: LibcAlloc = LibcAlloc;

    #[no_mangle]
    pub extern "C" fn allocate_struct() -> *mut TestStruct {
        let ts = Box::new(TestStruct { x: 7, y: 78901234 });
        Box::into_raw(ts)
    }
}

/// There are two different versions of the needs_alloc sub-module.
///
/// This version uses libc::malloc directly. It's a very delicate dance
/// with unsafe code. This is probably not the version you want unless
/// you have no other choice.
#[cfg(not(feature = "alloc"))]
mod needs_alloc {
    use super::TestStruct;
    use core::mem::size_of;
    use libc::{malloc, size_t};

    #[no_mangle]
    pub extern "C" fn allocate_struct() -> *mut TestStruct {
        let size: size_t = size_of::<TestStruct>();

        // SAFETY: We must avoid Rust seeing an uninitialized struct,
        // even for a moment, even via a `&` or `&mut`. `ptr::write`
        // is the only stable way I found to transition from
        // uninitialized memory to initialized memory without a
        // transient uninitialized `&` or `&mut`. The combination of
        //  `ptr::as_uninit_mut` and `MaybeUninit::write` would also
        // probably work (but are nightly only as of 2021-05).

        unsafe {
            let ts = malloc(size) as *mut TestStruct;
            ts.write(TestStruct { x: 8, y: 89012345 });
            ts
        }
    }
}

pub use needs_alloc::allocate_struct;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

#[repr(C)]
pub struct TestStruct {
    pub x: u8,
    pub y: u64,
}

pub const SOME_CONSTANT: u64 = 12345678;

#[repr(C)]
#[derive(PartialEq)]
pub enum TestEnum {
    One = 1,
    Two = 2,
    Three = 3,
}

#[no_mangle]
pub extern "C" fn fill_buffer(buf: *mut u8, len: usize) {
    assert!(!buf.is_null());
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, len as usize) };

    let s = b"hello world\0";
    buf[0..s.len()].copy_from_slice(s);
}

#[no_mangle]
pub extern "C" fn fill_struct(ptr: *mut TestStruct) {
    let test_struct = unsafe { ptr.as_mut().unwrap() };
    test_struct.x = 42;
    test_struct.y = 12345678;
}

#[no_mangle]
pub extern "C" fn handle_enum(x: TestEnum) -> bool {
    x == TestEnum::Two
}
