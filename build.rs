/* SPDX-License-Identifier: MIT */
/* Copyright (c) 2025 Gavin Henry <ghenry@sentrypeer.org> */

extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Initialize and update the Git submodule
    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to initialize/update submodules");

    let submodule_path = "vendor/opendht";

    // Try to find the OpenDHT library using pkg-config
    if pkg_config::Config::new()
        .atleast_version("2.3.5")
        .probe("opendht-c")
        .is_ok()
    {
        // If pkg-config finds the library, it will set the necessary
        // environment variables for linking.
        //}
        // try cmake
        // else if let dst = Config::new(submodule_path)
        //     .generator("Ninja")
        //     .define("CMAKE_INSTALL_LIBDIR", "lib")
        //     .define("OPENDHT_C", "ON")
        //     .define("CMAKE_BUILD_TYPE", "MinSizeRel")
        //     .build()
        // {
        //     println!("cargo:rustc-link-search=native={}/lib", dst.display());
        //     println!("cargo:rustc-link-lib=static=opendht-c");
    } else {
        // Compile the OpenDHT C library with autotools
        //
        // From autogen.sh as it doesn't have a shebang at the top
        // `autoreconf` will be system wide. No need for ./ in front
        Command::new("autoreconf")
            .args(&["--install", "--verbose", "-Wall"])
            .current_dir(submodule_path)
            .status()
            .expect("Failed to run autoreconf");

        // This is local to the submodule_path, so needs ./ in front
        Command::new("./configure")
            .args(&["--disable-python", "--disable-tools"])
            .current_dir(submodule_path)
            .status()
            .expect("Failed to run ./configure");

        // `make` will be system wide. No need for ./ in front
        Command::new("make")
            .current_dir(submodule_path)
            .status()
            .expect("Failed to run make");

        println!("cargo:rustc-link-search={}/src/.libs", submodule_path);
        println!("cargo:rustc-link-lib=opendht-c");
    }

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.rs!");
}
