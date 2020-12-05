# Python bindings

## Build

In order to build Python bindings, first follow
[main README's instructions](/README.md) and then, from the project root,
follow the [Local](#local) or [In docker](#in-docker) instructions.

Both instructions will generate the files `librgb.so` in
`rust-lib/target/debug/` and a shared object file
(e.g. `_rgb_node.cpython-37m-x86_64-linux-gnu.so`)
and `rgb_node.py` in `ffi/python/`.

### Local

* Install dependencies: python3-dev, swig 4.0
* From the project root run:
```bash
cd ffi/python
python3 setup.py build_ext
```

### In docker

```bash
docker build -f ffi/python/Dockerfile -t rgb-sdk-python .
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-python \
    -c 'mkdir -p /opt/mount/rust-lib/target/debug \
    && cp /rgb-sdk/rust-lib/target/debug/librgb.so /opt/mount/rust-lib/target/debug/ \
    && cp /rgb-sdk/ffi/python/*.so /opt/mount/ffi/python/ \
    && cp /rgb-sdk/ffi/python/rgb_node.py /opt/mount/ffi/python/'
```

## Usage

To try the generated library, you can use:
- [python demo](/demo/python)
