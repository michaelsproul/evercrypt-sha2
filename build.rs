use std::env;
use std::path::PathBuf;

// FIXME(sproul): fix path
fn main() {
    println!("cargo:rustc-link-lib=static=evercrypt");
    println!("cargo:rustc-link-search=/home/michael/Programming/hacl-star/dist/gcc-compatible");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/home/michael/Programming/hacl-star/dist/gcc-compatible")
        .clang_arg("-I/home/michael/Programming/hacl-star/dist/karamel/include")
        .clang_arg("-I/home/michael/Programming/hacl-star/dist/karamel/krmllib/dist/minimal")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
