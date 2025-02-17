# Building opendht-sys

## Steps we take

1. `pkg-config` is used to find the `opendht-c` library on the current system.
2. `cmake` is tried next with the opendht source in `./vendor/opendht`.
3. `autotools` are tried last, again with the opendht source in `./vendor/opendht`.

## Build Requirements

If we build via `cmake` or `autotools`, we need the following installed first:

TODO

- `fmt-dev`

