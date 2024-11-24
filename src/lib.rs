#[allow(dead_code, nonstandard_style, clippy::upper_case_acronyms)]
mod bindings {
    // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/components/cronet/native/cronet.idl
    include!(concat! {env!("OUT_DIR"), "/bindings.rs"});
}
#[allow(dead_code, unused_imports)]
pub(crate) mod sys;
pub(crate) mod util;

pub mod body;
pub mod client;
pub mod error;
pub mod fetch;
