# Low-level bindings to the OpenDHT C library

[![Stability: Active](https://masterminds.github.io/stability/active.svg)](https://masterminds.github.io/stability/active.html)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/sentrypeer/opendht-sys?sort=semver)](https://github.com/SentryPeer/opendht-sys/releases)
[![CI](https://github.com/SentryPeer/opendht-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/SentryPeer/opendht-sys/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/opendht-sys)](https://crates.io/crates/opendht-sys)
[![Docs.rs](https://docs.rs/opendht-sys/badge.svg)](https://docs.rs/opendht-sys)

# Building opendht-sys

## Steps we take

1. `pkg-config` is used to find the `opendht-c` library on the current system.
2. `cmake` is tried next with the opendht source in `./vendor/opendht`.

## Build Requirements

If we build via `cmake`, we need the following installed first (Debian or Ubuntu):

* `fmt-dev`
* `pkg-config` 
* `libgnutls28-dev` - yes, on crates.io
* `libmsgpack-dev`  - no
* `libargon2-dev`   - yes, on crates.io
* `libasio-dev`     - no
* `libfmt-dev`      - no, needed?
* `nettle-dev`      - yes, on crates.io
* `ninja-build` 

### -sys crates

TODO: All of the above will be `-sys` crates and set as dependencies in our `Cargo.toml` file.

