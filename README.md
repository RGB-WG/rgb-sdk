# RGB SDK

This repository contains FFI bindings and SDK for wallet & server-side development,
plus some sample applications

## Building RGB SDK

```shell script
cd rust-ffi
cargo lipo
cargo lipo --release
```

Then check гыу `rust-ffi/target/universal/release/rgb.a` in any application integration.

## Language bindings

The following bindings are available:
- [Android](/rust-ffi/android)
- [iOS](/rust-ffi/ios)
- [Node.js](/rust-ffi/nodejs)

## Developer guidelines

In order to update the project dependencies, run `cargo update`.
If any dependency updates, the `Cargo.lock` file will be updated, keeping
track of the exact package version.
After an update, run tests (`cargo test`) and manually test the software
in order to stimulate function calls from updated libraries.
If any problem arises, open an issue.
