#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_parens)]

include!(concat!(env!("OUT_DIR"), "/nvenc_v10_0.rs"));
include!(concat!(env!("OUT_DIR"), "/nvenc_v10_0_struct_versions.rs"));

const fn NVENCAPI_STRUCT_VERSION(ver: u32) -> u32 {
    NVENCAPI_VERSION | (ver << 16) | (0x7 << 28)
}
