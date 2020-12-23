FROM rust:1.47.0-slim-buster as builder

COPY bindings/npm /rgb-sdk/bindings/npm
COPY librgb /rgb-sdk/librgb

RUN apt-get update -y \
    && apt-get install -y \
        cmake \
        python3 \
        pkg-config \
        make \
        perl \
        g++ \
        git \
        wget \
        libssl-dev \
        libzmq3-dev \
        libpcre3-dev \
        libpq-dev

RUN wget https://freefr.dl.sourceforge.net/project/swig/swig/swig-4.0.1/swig-4.0.1.tar.gz && \
    tar xf swig-*.tar.gz && \
    cd swig-* && \
    ./configure && make -j12 && \
    make install

RUN wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash && \
    . $HOME/.nvm/nvm.sh && \
    nvm install v10 --lts && \
    nvm alias default v10

WORKDIR /rgb-sdk/bindings/npm

RUN . $HOME/.nvm/nvm.sh && npm install --unsafe-perm



FROM node:10-buster-slim

RUN apt-get update -y \
    && apt-get install -y \
        libzmq5 \
        libssl1.1 \
        libpq5 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN mkdir -p /rgb-sdk/librgb/target/release/ /rgb-sdk/bindings/npm/

COPY --from=builder /rgb-sdk/bindings/npm/build/Release/rgblib.node /rgb-sdk/bindings/npm/build/Release/
COPY --from=builder /rgb-sdk/librgb/target/release/librgb.so /rgb-sdk/librgb/target/release/

WORKDIR /rgb-sdk/