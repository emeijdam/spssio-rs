use std::{env, fs};
use std::path::PathBuf;

fn main() {
    // This is the directory where the `c` library is located.
    
    println!("{}", env::consts::OS);

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir).join("../../../..");

    let profile = env::var("PROFILE").unwrap();
    let target_dir = target_dir.join(profile);


    if env::consts::OS == "windows" {
        println!("win");

        let indir_libdir_path = PathBuf::from(r".");
        println!("cargo:rustc-link-search={}", indir_libdir_path.to_str().unwrap());


        let libdir_path = PathBuf::from(r".\iomod\win64")
            // Canonicalize the path as `rustc-link-search` requires an absolute
            // path.
            .canonicalize()
            .expect("cannot canonicalize path");

        println!("{:?}", libdir_path);
        println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=spssio64");
        println!("cargo:rustc-link-arg=-rpath");
    } else if env::consts::OS == "macos" {
        println!("macos1");

        let indir_libdir_path = PathBuf::from(r"./");
        println!("cargo:rustc-link-search={}", indir_libdir_path.to_str().unwrap());

        let libdir_path = PathBuf::from(r"./iomod/macos")
            // Canonicalize the path as `rustc-link-search` requires an absolute
            // path.
            .canonicalize()
            .expect("cannot canonicalize path");

        println!("{:?}", libdir_path);
        println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=dylib=spssdio");

        let libs = vec!["libspssdio.dylib", "libgsk8iccs.dylib", "libzlib1213spss.dylib", "libicuuc.61.2.dylib", 
        "libicudata.61.2.dylib", "libicui18n.61.2.dylib"];

        for lib in libs {
            let src = PathBuf::from("./iomod/macos").join(lib);
            let dest = target_dir.join(lib);
            fs::copy(&src, &dest).expect("Failed to copy library");
        }

       // println!("cargo:rustc-link-arg=-rpath=./");
    } else if env::consts::OS == "linux" {
        println!("linux");
    };

    // #[cfg(target_arch = "x86_64")]
    // let libdir_path = PathBuf::from(r".\iomod\win64")
    //     // Canonicalize the path as `rustc-link-search` requires an absolute
    //     // path.
    //     .canonicalize()
    //     .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    //let headers_path = libdir_path.join("spssdio.h");
    //let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.

    // println!("cargo:rustc-link-lib=dylib=spssdio");

    // #[cfg(target_arch = "x86_64")]
    // println!("cargo:rustc-link-lib=spssio64");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(&["-I./"])
        .clang_args(&["-I./imod/win64"])
        .clang_arg("-IC:\\dev\\what-io\\iomod\\win64\\")
        .clang_arg("-I./iomod/macos/")
        .clang_arg("-v")
        .header("wrapper.h")
        .array_pointers_in_arguments(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

        // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        // println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
