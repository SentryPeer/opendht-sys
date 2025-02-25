// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Gavin Henry <ghenry@sentrypeer.org>

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
        .args(["submodule", "update", "--init", "--recursive"])
        .status()
        .expect("Failed to initialize/update submodules");

    let submodule_path = "vendor/opendht";

    // Try to find the OpenDHT library using pkg-config first
    if pkg_config::Config::new()
        .atleast_version("2.3.5")
        .probe("opendht-c")
        .is_ok()
    {
        // If found, the necessary env vars will get set automatically
    } else {
        // try cmake
        let dst = Config::new(submodule_path)
            .generator("Ninja")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .define("OPENDHT_C", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("CMAKE_BUILD_TYPE", "MinSizeRel")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());

        // TODO: Handle macOS (usages static linking by default) and Windows
        // (if Windows is supported)
        println!("cargo:rustc-link-lib=opendht");
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
