use bindgen::builder;

fn main() {
    bindgen_ffi();

    // println!("cargo:rustc-link-search={}", std::env::var("CRONET_RS_LIB_DIR").unwrap());

    // println!("cargo:rustc-link-lib=cronet");
}

fn bindgen_ffi() {
    const CRONET_PREFIX: &str = "^Cronet_.+";
    let bindings = builder()
        .headers(["stdbool.h", "include/cronet.idl_c.h"])
        .allowlist_item(CRONET_PREFIX)
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut binding_file = std::path::PathBuf::from(out_dir);
    binding_file.push("bindings.rs");

    bindings.write_to_file(binding_file).unwrap();
}
