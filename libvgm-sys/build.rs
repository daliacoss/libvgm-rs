use cmake::Config;
use bindgen::Builder;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if !Path::new("libvgm/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bld = dst.join("build");

    fs::create_dir_all(&bld).unwrap();

    let mut include_root = "-I".to_owned();
    include_root.push_str(&String::from(
        env::current_dir()
        .unwrap()
        .join("libvgm")
        .into_os_string()
        .into_string()
        .unwrap()));

    let dst = Config::new(Path::new("libvgm"))
        .cflag(include_root)
        .define("BUILD_LIBAUDIO", "OFF")
        .define("BUILD_LIBPLAYER", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_PLAYER", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    println!("cargo:rustc-link-lib=static=vgm-emu");
    println!("cargo:rustc-link-lib=static=vgm-utils");
    println!("cargo:rustc-link-lib=vgm-player");

    let bindings = Builder::default()
        .header("wrapper.h")
        //.clang_args(&["-x", "c++"])
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings to file!");
}
