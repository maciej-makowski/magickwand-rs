extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{find_magickwand_include, find_magickwand_lib, find_magickwand_search_path};

fn main() {
    let magickwand_include = find_magickwand_include();
    if !Path::new(&magickwand_include).exists() {
        panic!("Expected directory to exist: {}", magickwand_include);
    }

    let magickwand_lib = find_magickwand_lib();
    println!("cargo:rustc-link-lib={}", magickwand_lib);

    find_magickwand_search_path().map(|path| {
        if !Path::new(&path).exists() {
            panic!("MAGICKWAND_PATH={} does not exist", path)
        }
        println!("cargo:rustc-link-search={}", magickwand_lib);
        println!("cargo:rustc-link-search={}/lib", magickwand_lib);
    });

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", magickwand_include))
        .whitelist_var(".*MAGICK.*")
        .whitelist_var(".*Magick.*")
        .whitelist_function(".*Magick.*")
        .whitelist_type(".*Magick.*")
        .generate()
        .unwrap_or_else(|e| panic!("Unable to generate bindings: {:?}", e));

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
