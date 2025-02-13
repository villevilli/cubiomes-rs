#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// This module contains generated rust enums from biomes.h
///
/// ## Safety
/// The enums should not be used directly as outputs for ffi, as that can cause
/// ub if cubiomes returns something that doesnt fit the enum. They are included
/// with the assumption that the user validates cubiomes output before constructing
/// them.
pub mod biome_enum {
    include!(concat!(env!("OUT_DIR"), "/biome_enums.rs"));
}
