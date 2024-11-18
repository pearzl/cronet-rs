#[allow(nonstandard_style, clippy::upper_case_acronyms)]
#[allow(dead_code)] //todo
mod bindings {
    // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/components/cronet/native/cronet.idl
    include!(concat! {env!("OUT_DIR"), "/bindings.rs"});
}
#[allow(dead_code, unused_imports)] // todo
pub(crate) mod sys;
pub(crate) mod util;

pub mod body;
pub mod client;
pub mod error;
pub mod fetch;
