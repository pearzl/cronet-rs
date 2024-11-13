#[allow(nonstandard_style, dead_code)]
mod bindings {
    include!(concat!{env!("OUT_DIR"), "/bindings.rs"});
}
pub mod sys;

pub mod body;
pub mod client;
pub mod error;
