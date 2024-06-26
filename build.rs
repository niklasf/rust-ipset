use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=ipset");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=wrapper.h");

    cc::Build::new().file("wrapper.c").compile("aux");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(src_dir.join("src/binding.rs"))
        .expect("Unable to write bindings.rs");
}
