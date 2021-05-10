### cbindgen demo project

If you're new to Rust FFI, or if you're using [cbindgen](https://github.com/eqrion/cbindgen) for the first time, it may take a few attempts to get things right.

If you're lazy like me, you might want to start with a sample project, already set up for `no_std`, `staticlib`, `panic="abort"`, and a working `build.rs`.

This version does not include the Rust `std` library, but does do memory allocation, which is nontrivial in a `no_std` build. There are two options:

1. Declare a global memory allocator. This project is configured to use the `libc_alloc` crate, and can allocate with `Box` and `Vec`. This is simpler, but requires using the nightly compiler.
2. Try to run without a global rust allocator. This is pretty challenging, but in a pinch you can call `libc::malloc` directly, and it seems to work.

There are two simpler versions of this project. If you want a library without `std` and without `alloc`, try [cbindgen-demo](https://github.com/ericseppanen/cbindgen-demo). If you want to build a library that has access to the full Rust `std` library, try [cbindgen-std-demo](https://github.com/ericseppanen/cbindgen-std-demo).

Directions for building (with alloc):
```
> cargo +nightly build --release
> gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
> strip demo
> ./demo
```
Build without alloc:
```
> cargo build --release --no-default-features
> gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
> strip demo
> ./demo
```
