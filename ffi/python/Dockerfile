FROM rust:1.47.0-slim-buster

RUN apt-get update -y \
    && apt-get install -y \
        cmake \
        g++ \
        git \
        libpcre3-dev \
        libpq-dev \
        libssl-dev \
        libzmq3-dev \
        make \
        perl \
        pkg-config \
        python3-pip \
        wget

RUN wget https://freefr.dl.sourceforge.net/project/swig/swig/swig-4.0.1/swig-4.0.1.tar.gz && \
    tar xf swig-*.tar.gz && \
    cd swig-* && \
    ./configure && make -j12 && \
    make install

COPY ffi/python /rgb-sdk/ffi/python
COPY rust-lib /rgb-sdk/rust-lib

WORKDIR /rgb-sdk/ffi/python

RUN python3 -m pip install --upgrade pip setuptools wheel \
    && python3 setup.py build_ext
