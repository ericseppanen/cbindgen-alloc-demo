
//  Build with alloc (requires nightly)
//  > cargo +nightly build --release
//  > gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
//  > strip demo
//  > ./demo

//  Build without alloc:
//  > cargo build --release --no-default-features
//  > gcc -O2 -Wl,--gc-sections main.c target/release/libdemo.a -o demo
//  > strip demo
//  > ./demo

#include <stdio.h>

#include "bindings.h"

int main() {
    char mybuf[32];

    // Have the rust code write a string into our buffer.
    fill_buffer(mybuf, sizeof(mybuf));
    printf("%s\n", mybuf);


    TestStruct ts = { 0, 0 };

    // Have the rust code initialize a struct for us.
    fill_struct(&ts);

    printf("%u %lu\n", ts.x, ts.y);

    bool x = handle_enum(Two);
    printf("enum test: %u\n", x);

    TestStruct * ts2 = allocate_struct();
    printf("%u %lu\n", ts2->x, ts2->y);
    free(ts2);
}
