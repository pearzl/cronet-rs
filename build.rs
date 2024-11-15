use bindgen::builder;

fn main() {
    bindgen_ffi();

    println!("cargo:rustc-link-search={}", std::env::var("CRONET_RS_LIB_DIR").unwrap());

    println!("cargo:rustc-link-lib=cronet");
}

fn bindgen_ffi() {
    const CRONET_PREFIX: &str = "^Cronet_.+";
    let bindings = builder()
        .headers(["stdbool.h", "include/cronet.idl_c.h"])
        .allowlist_item(CRONET_PREFIX)
        .parse_callbacks(Box::new(BindgenParseCallback))
        .rustified_non_exhaustive_enum(CRONET_PREFIX)
        .prepend_enum_name(false)
        .generate()
        .unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut binding_file = std::path::PathBuf::from(out_dir);
    binding_file.push("bindings.rs");

    bindings.write_to_file(binding_file).unwrap();
}

#[derive(Debug)]
struct BindgenParseCallback;

impl bindgen::callbacks::ParseCallbacks for BindgenParseCallback {
    fn header_file(&self, filename: &str) {
        println!("cargo:rerun-if-changed={}", filename);
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let prefix = &enum_name?[5..]; // "trim starting `enum `"
        let new_name = original_variant_name.trim_start_matches(prefix);
        Some(new_name[1..].to_string())
    }
}
