use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
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
        .whitelist_type("UDTSOCKET")
        .whitelist_type("UDTSTATUS")
        .whitelist_type("UDTOpt")
        .whitelist_type("CPerfMon")
        .whitelist_function("startup")
        .whitelist_function("cleanup")
        .whitelist_function("socket")
        .whitelist_function("bind")
        .whitelist_function("bind2")
        .whitelist_function("listen")
        .whitelist_function("accept")
        .whitelist_function("connect")
        .whitelist_function("close")
        .whitelist_function("getpeername")
        .whitelist_function("getsockname")
        .whitelist_function("getsockopt")
        .whitelist_function("setsockopt")
        .whitelist_function("send")
        .whitelist_function("recv")
        .whitelist_function("sendmsg")
        .whitelist_function("recvmsg")
        .whitelist_function("sendfile2")
        .whitelist_function("recvfile2")
        .whitelist_function("epoll_create")
        .whitelist_function("epoll_add_usock")
        .whitelist_function("epoll_add_ssock")
        .whitelist_function("epoll_remove_usock")
        .whitelist_function("epoll_remove_ssock")
        .whitelist_function("epoll_wait2")
        .whitelist_function("epoll_release")
        .whitelist_function("getlasterror_code")
        .whitelist_function("getlasterror_desc")
        .whitelist_function("getsockstate")
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
