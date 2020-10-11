# Node.js bindings

## Build

In order to build Node.js bindings, from the project root follow the _Local_ or
_In docker_ instructions.

Both instructions will generate the files `librgb.so` in `target/debug/`
and `rgb_node.node` in `ffi/nodejs/build/Release/`.

### Local

```bash
sudo apt install -y swig node-gyp
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash
nvm install v10
cd ffi/nodejs
npm install
```

### In docker

```bash
docker build -f ffi/nodejs/Dockerfile -t rgb-sdk-nodejs .
mkdir -p rust-lib/target/debug ffi/nodejs/build/Release
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-nodejs \
    -c 'cp /rgb-sdk/target/debug/librgb.so /opt/mount/rust-lib/target/debug/librgb.so \
    && cp /rgb-sdk/rgb_node.node /opt/mount/ffi/nodejs/build/Release/rgb_node.node'
```

## Usage

To try the generated library, from the project root run:
```bash
cd demo/nodejs
node example.js
```
