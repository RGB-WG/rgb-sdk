# iOS bindings

## Build

In order to include RGB into iPhone, iPad or Mac application, on Mac OS,
from the project root, run:

```bash
# install dependencies
brew install cmake openssl zmq
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo install cargo-lipo
# build bindings
cargo lipo --manifest-path librgb/Cargo.toml --release
```

These instructions will generate the files `librgb.a` in
`librgb/target/universal/release/` and `librgb.h` in `librgb/`.

Add `librgb.a` to your project as an external framework/library and add
`librgb.h` as an Objective-C bridging header.

You will also need to add `libzmq.a` as a library dependency. For this you will
need to do manually compile ZMQ library from sources for iOS target and copy
the resulting library as a dependency. Please make sure that you are checking out
exactly the same version of the code as used by RGB library.
Build script can be found
[here](https://github.com/zeromq/libzmq/blob/master/builds/ios/build_ios.sh).

## Usage

To try the generated library, you can use:
- [iOS demo](/demo/ios)
