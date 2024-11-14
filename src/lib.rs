#[allow(nonstandard_style, clippy::upper_case_acronyms)]
#[allow(dead_code)] //todo
mod bindings {
    include!(concat! {env!("OUT_DIR"), "/bindings.rs"});
}
#[allow(clippy::not_unsafe_ptr_arg_deref, dead_code, unused_imports)] // todo
pub mod sys;

pub mod body;
pub mod client;
pub mod error;
pub mod util;
