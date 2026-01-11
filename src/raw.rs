#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use autogen::*;

#[allow(unused_imports)]
pub use additions::*;

// Several functions in this library are deprecated. We allow dead code to avoid warnings.
#[allow(dead_code)]
pub mod autogen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod additions {
    #[allow(unused_imports)]
    use super::autogen;
}
