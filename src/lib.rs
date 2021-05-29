#[cfg_attr(not(target_arch = "spirv"), macro_use)]
pub extern crate spirv_std_macros as macros;
pub use crate::macros::Foo;
