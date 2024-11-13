#[allow(nonstandard_style, dead_code)]
mod bindings {
    include!(concat!{env!("OUT_DIR"), "/bindings.rs"});
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub mod sys;

pub mod body;
pub mod client;
pub mod error;
pub mod util;
