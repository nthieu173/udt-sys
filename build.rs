use cc;
use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .file("wrapper.cpp")
        .compile("libudtwrapper");

    // Tell cargo to tell rustc to link the system udt shared library.
    println!("cargo:rustc-link-lib=udt");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-x")
        .clang_arg("c++")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .bitfield_enum("EPOLLOpt")
        .whitelist_type("EPOLLOpt")
        .whitelist_function("udt_.*")
        .whitelist_var("UDT_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
