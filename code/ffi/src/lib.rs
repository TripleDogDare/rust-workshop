mod api;
mod ffi;

/// The following test function is necessary for the header generation.
#[cfg(feature = "c-headers")]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("./headers/ffi.h")?
        .generate()
}
