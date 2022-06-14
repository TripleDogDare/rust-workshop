use safer_ffi::prelude::*;

use crate::api;

// Example 1: take a primitive type, return a primitive type

#[ffi_export]
fn example1(x: u32) -> u32 {
    api::example1(x)
}

// Example 2: passing around vectors

#[ffi_export]
fn example2_1(x: u32) -> safer_ffi::Vec<u32> {
    // There are `From` traits defined to convert between `safer_ffi::Vec` and `std::Vec`
    api::example2_1(x).into()
}

// This function takes a reference to a vector and hence does not consume it on the C side
#[ffi_export]
fn example2_2(x: &safer_ffi::Vec<u32>) -> u32 {
    api::example2_2(&x)
}

// This function transfers the ownership from C to Rust, and frees the buffer on the C side
#[ffi_export]
fn example2_2_consume(x: safer_ffi::Vec<u32>) -> u32 {
    api::example2_2(&x)
}

#[ffi_export]
fn example2_drop(x: safer_ffi::Vec<u32>) {
    // Technically, `x` will be dropped anyway when the function returns,
    // but an explicit `drop` prevents a warning about an unused variable,
    // and makes the intent more clear.
    drop(x)
}

// Example 3: transparent struct

#[derive_ReprC]
#[repr(C)]
pub struct Example3 {
    pub(crate) x: u32,
    pub(crate) y: bool,
}

#[ffi_export]
fn example3_make() -> Example3 {
    Example3 { x: 0, y: false }
}

#[ffi_export]
fn example3_fill(s: &mut Example3) {
    s.x = 3;
    s.y = true;
}

// Example 4: opaque struct

#[derive_ReprC]
#[ReprC::opaque]
pub struct Example4 {
    backend: api::Example4,
}

// We cannot return an opaque struct by value, only a pointer to it.
// On the Rust side it means we need to put it in a `Box`.
#[ffi_export]
fn example4_make() -> repr_c::Box<Example4> {
    repr_c::Box::new(Example4 {
        backend: api::Example4 { x: vec![1, 2, 3] },
    })
}

#[ffi_export]
fn example4_read(s: &Example4) -> u32 {
    s.backend.x[0]
}

#[ffi_export]
fn example4_read_box(s: &repr_c::Box<Example4>) -> u32 {
    (*s).backend.x[0]
}

#[ffi_export]
fn example4_mutate(s: &mut Example4) {
    s.backend.x[0] = 10;
}

#[ffi_export]
fn example4_drop(s: repr_c::Box<Example4>) {
    drop(s)
}

// Example 4a: nullable pointers

#[ffi_export]
fn example4a_read(s: Option<&Example4>) -> u32 {
    s.map(|s| s.backend.x[0]).unwrap_or(4)
}

// Example 5: output parameters

#[ffi_export]
fn example5_value(result: Out<u32>) {
    result.write(10);
}

#[ffi_export]
fn example5_pointer(result: Out<repr_c::Box<Example4>>) -> bool {
    let s = repr_c::Box::new(Example4 {
        backend: api::Example4 { x: vec![1, 2, 3] },
    });
    result.write(s);
    true
}

#[ffi_export]
fn example5_nullable(result: Option<Out<repr_c::Box<Example4>>>) -> bool {
    match result {
        Some(out) => {
            let s = repr_c::Box::new(Example4 {
                backend: api::Example4 { x: vec![1, 2, 3] },
            });
            out.write(s);
            true
        }
        None => false,
    }
}
