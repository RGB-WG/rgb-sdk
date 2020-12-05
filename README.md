# RGB SDK

![Bindings](https://github.com/LNP-BP/rgb-sdk/workflows/Bindings/badge.svg)
![Lints](https://github.com/LNP-BP/rgb-sdk/workflows/Lints/badge.svg)

This repository contains FFI bindings and SDK for wallet & server-side development,
plus some sample applications.

## Building RGB SDK

First, you'll need to [install Rust](https://www.rust-lang.org/tools/install),
then follow the instructions specific to your language binding of choice.

### Language bindings

The following bindings are available:
- [Android](/ffi/android)
- [iOS](/ffi/ios)
- [Node.js](/ffi/nodejs)
- [Python](/ffi/python)

## Developer guidelines

In order to update the project dependencies, run `cargo update`.
If any dependency updates, the `Cargo.lock` file will be updated, keeping
track of the exact package version.
After an update, run tests (`cargo test`) and manually test the software
in order to stimulate function calls from updated libraries.
If any problem arises, open an issue.
