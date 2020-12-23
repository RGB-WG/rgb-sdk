# Node.js bindings

## Build

In order to build Node.js bindings, from the project root follow the
[Local](#local) or [In docker](#in-docker) instructions.

Both instructions will generate the files `librgb.so` in `rust-lib/target/release/`
and `rgb.node` in `bindings/nodejs/build/Release/`.

### Local

* Install dependencies: Node.js v10, node-gyp, swig 4.0
* From the project root run:
```bash
cd bindings/npm
npm install
```

### In docker

```bash
docker build -f bindings/npm/Dockerfile -t rgb-sdk-npm .
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-npm \
    -c 'mkdir -p /opt/mount/librgb/target/release /opt/mount/bindings/npm/build/Release \
    && cp /rgb-sdk/librgb/target/release/librgb.so /opt/mount/librgb/target/release/librgb.so \
    && cp /rgb-sdk/bindings/npm/build/Release/rgblib.node /opt/mount/bindings/npm/build/Release/rgblib.node'
```

## Usage

To try the generated library, you can use:
- [node.js demo](/demo/nodejs)