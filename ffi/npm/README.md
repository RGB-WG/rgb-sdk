# Node.js bindings

## Build

In order to build Node.js bindings, from the project root follow the
[Local](#local) or [In docker](#in-docker) instructions.

Both instructions will generate the files `librgb.so` in `rust-lib/target/release/`
and `rgb.node` in `ffi/nodejs/build/Release/`.

### Local

* Install dependencies: Node.js v10, node-gyp, swig 4.0
* From the project root run:
```bash
cd ffi/npm
npm install
```

### In docker

```bash
docker build -f ffi/npmnpm/Dockerfile -t rgb-sdk-npm .
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-npm \
    -c 'mkdir -p /opt/mount/librgb/target/release /opt/mount/ffi/npm/build/Release \
    && cp /rgb-sdk/target/release/librgb.so /opt/mount/librgb/target/release/librgb.so \
    && cp /rgb-sdk/rgb.node /opt/mount/ffi/npm/build/Release/rgb.node'
```

## Usage

To try the generated library, you can use:
- [node.js demo](/demo/nodejs)
