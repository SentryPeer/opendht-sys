// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Gavin Henry <ghenry@sentrypeer.org>

// Ignore Rust's style conventions for our C FFI bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Use the include! macro to dump our generated bindings right into our crate's main entry point,
// src/lib.rs:
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
