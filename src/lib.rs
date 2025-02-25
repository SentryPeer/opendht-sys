// SPDX-License-Identifier: GPL-3.0
// Copyright (c) 2025 Gavin Henry <ghenry@sentrypeer.org>

// Ignore Rust's style conventions for our C FFI bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Use the include! macro to dump our generated bindings right into our crate's main entry point,
// src/lib.rs:
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Some basic tests to ensure that our bindings are working as expected with
// OpenDHT's C API

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dht_calls() {
        unsafe {
            let dht = dht_runner_new();
            assert!(!dht.is_null());
            dht_runner_delete(dht);
        }
    }
}
