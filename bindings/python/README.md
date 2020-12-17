# Python bindings

## Build

In order to build Python bindings, first follow
[main README's instructions](/README.md) and then, from the project root,
follow the [Local](#local) or [In docker](#in-docker) instructions.

Both instructions will generate the files `librgb.so` in
`rust-lib/target/release/` and a shared object file
(e.g. `_rgb_node.cpython-37m-x86_64-linux-gnu.so`)
and `rgb_node.py` in `bindings/python/`.

### Local

* Install dependencies: python3-dev, swig 4.0
* From the project root run:
```bash
cd bindings/python
python3 setup.py build_ext
```

### In docker

```bash
docker build -f bindings/python/Dockerfile -t rgb-sdk-python .
docker run --rm -v $(pwd):/opt/mount --entrypoint bash \
    rgb-sdk-python \
    -c 'mkdir -p /opt/mount/librgb/target/release \
    && cp /rgb-sdk/librgb/target/release/librgb.so /opt/mount/librgb/target/release/ \
    && cp /rgb-sdk/bindings/python/*.so /opt/mount/bindings/python/ \
    && cp /rgb-sdk/bindings/python/rgb.py /opt/mount/bindings/python/'
```

## Usage

To try the generated library, you can use:
- [python demo](/demo/python)
