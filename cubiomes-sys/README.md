# Cubiomes-sys
Rust raw-ffi bindings for the cubiomes library made by cubitect
Original library can be found in https://github.com/Cubitect/cubiomes

Bindings generated with bindgen. The crate also statically links cubiomes.

For usage you probably want the safe rust binding found in the cubiomes crate.

## Cargo features
``cc_build``, use the cc create for building, enabled by default. If not enabled
cubiomes is built with make