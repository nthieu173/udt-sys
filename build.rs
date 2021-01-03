use cc;
use std::{env, fs, path::PathBuf};

fn main() {
    let udt4_src = PathBuf::from("udt-git/udt4/src");
    let mut cfg = cc::Build::new();

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
        // g++ -fPIC -Wall -Wextra -DLINUX -finline-functions -O3 -fno-strict-aliasing -fvisibility=hidden -DAMD64 ccc.cpp -c
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

    cfg.debug(false).cpp(true).compile("udtwrap");
}
