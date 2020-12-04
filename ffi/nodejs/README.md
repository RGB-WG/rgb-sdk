# Node.js bindings

## Build

In order to build Node.js bindings, first follow
[main README's instructions](/README.md) and then, from the project root,
follow the [Local](#local) or [In docker](#in-docker) instructions.

Both instructions will generate the files `librgb.so` in `rust-lib/target/debug/`
and `rgb_node.node` in `ffi/nodejs/build/Release/`.

### Local

* Install dependencies: Node.js v10, node-gyp, swig 4.0
* From the project root run:
```bash
cd ffi/nodejs
npm install
```

### In docker

```bash
docker build -f ffi/nodejs/Dockerfile -t rgb-sdk-nodejs .
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-nodejs \
    -c 'mkdir -p /opt/mount/rust-lib/target/debug /opt/mount/ffi/nodejs/build/Release \
    && cp /rgb-sdk/target/debug/librgb.so /opt/mount/rust-lib/target/debug/librgb.so \
    && cp /rgb-sdk/rgb_node.node /opt/mount/ffi/nodejs/build/Release/rgb_node.node'
```

## Usage

To try the generated library, you can use:
- [node.js demo](/demo/nodejs)
