use cc;
use std::{env, fs, path::PathBuf};

fn main() {
    let udt4_src = PathBuf::from("udt-git/udt4/src");
    compile_wrapper();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Tell cargo to tell rustc to link the system udt shared library.
    println!("cargo:rustc-link-lib=udt");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg(format!(
            "-I{}",
            udt4_src.to_str().expect("malformed udt src path")
        ))
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

fn compile_wrapper() {
    let udt4_src = PathBuf::from("udt-git/udt4/src");
    let mut cfg = cc::Build::new();

    // g++ -fPIC -Wall -Wextra -DLINUX -finline-functions -O3 -fno-strict-aliasing -fvisibility=hidden -DAMD64 ccc.cpp -c
    cfg.include(&udt4_src);
    cfg.file("wrapper.cpp");

    if cfg!(feature = "vendored") {
        // get list of .cpp files
        for dir in fs::read_dir(&udt4_src).unwrap() {
            let path = dir.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "cpp" {
                    cfg.file(path);
                }
            }
        }
        if cfg!(target_os = "macox") {
            cfg.define("osx", None);
        } else if cfg!(target_os = "unix") {
            cfg.define("LINUX", None).define("AMD64", None);
        } else if cfg!(target_os = "windows") {
            // These flags were discovered by opening the UDT vcproject file and viewing the 'Command Line' section of the Release Configuration
            cfg.flag("/GS")
                .flag("/analyze-")
                .flag("/Zc:wchar_t")
                .flag("/Zi")
                .flag("/Zc:inline")
                .flag("/fp:precise")
                .define("WIN32", None)
                .define("NDEBUG", None)
                .define("_CONSOLE", None)
                .define("UDT_EXPORTS", None)
                .define("_WINDLL", None)
                .flag("/errorReport:prompt")
                //.flag("/WX")
                .flag("/Zc:forScope")
                .flag("/Gd")
                .flag("/O2") // optimize for speed
                .flag("/Ot") // favor fast code
                .flag("/Ob2") // inline any suitable function
                .flag("/Oy") // omit frame pointers
                .flag("/nologo") // suppress startup banner
                .flag("/W4") // warning level
                .flag("/Gm-") // disable minimal rebuild
                .flag("/EHsc") // enable c++ exceptions
                .flag("/MD"); // multi-threaded DLL runtime
        } else {
            cfg.flag("-fPIC")
                .opt_level(3)
                .flag("-Wextra")
                .flag("-Wall")
                .flag("-finline-functions")
                .flag("-fno-strict-aliasing")
                .flag("-fvisibility=hidden");
        }
    }

    cfg.cpp(true).compile("udtwrap");
}
