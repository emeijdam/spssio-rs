use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // This is the directory where the `c` library is located.
    println!("{}", env::consts::OS);

    let version = env!("CARGO_PKG_VERSION");
    println!("version: env{}", version);

    let out_dir = env::var("OUT_DIR").unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let target_dir = PathBuf::from(out_dir).join("../../../..");

    let profile = env::var("PROFILE").unwrap();
    let target_dir = target_dir.join(profile);

    let spssio_lib = PathBuf::from(r"../../IO_Module_for_SPSS_Statistics_29.0.2/macos");

    let libdir_path = spssio_lib
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    println!("macos1");

    let indir_libdir_path = PathBuf::from(r"./");
    println!(
        "cargo:rustc-link-search={}",
        indir_libdir_path.to_str().unwrap()
    );

    println!("{:?}", libdir_path);
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=spssdio");

    let files = fs::read_dir(&spssio_lib).unwrap();

    for file in files {
        let file = file.unwrap();
        println!("{}", file.path().display());

        match file.path().extension() {
            Some(ext) => {
                if ext == "dylib" {
                    let src = file.path();
                    let dest = target_dir.join(file.file_name());
                    fs::copy(&src, &dest).expect("Failed to copy library");
                }
            }
            None => {}
        }
    }

    let mut clangarg = "-I".to_string();
    clangarg.push_str(spssio_lib.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .clang_arg("-I../../IO_Module_for_SPSS_Statistics_29.0.2/include")
        .header("wrapper.h")
        .array_pointers_in_arguments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
