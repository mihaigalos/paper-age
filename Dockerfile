# ------------------------------------------------------------
FROM rust:alpine3.17 as base-paper-age
RUN apk update \
    && apk add \
        git \
        gcc \
        g++ \
        openssl \
        openssl-dev \
        pkgconfig

COPY . /src

RUN rustup update

RUN cd /src && cargo build --release

# ------------------------------------------------------------
FROM rust:alpine3.17 as base-rage
RUN apk update \
    && apk add \
        git \
        gcc \
        g++ \
        pcsc-lite-dev \
        openssl \
        openssl-dev \
        pkgconfig

COPY . /src

WORKDIR /src
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

RUN git clone --depth 1 https://github.com/str4d/rage.git \
    && cd rage \
    && cargo build --release
# ------------------------------------------------------------
# docker run --rm -it -v $(pwd):/src -v /run/pcscd/pcscd.comm:/run/pcscd/pcscd.comm rage-yubikey
FROM rust:alpine3.17 as base-age-plugin-yubikey
RUN apk update \
    && apk add \
        git \
        gcc \
        g++ \
        pcsc-lite-dev \
        openssl \
        openssl-dev \
        pkgconfig

COPY . /src

WORKDIR /src
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

RUN git clone --depth 1 https://github.com/str4d/rage.git \
    && cd rage \
    && cargo build --release

FROM alpine:3.14 as tool

RUN apk update \
    && apk add \
        libgcc \
        pcsc-lite-dev
# ------------------------------------------------------------

FROM alpine:3.17 as tool

RUN apk update \
    && apk add \
        libgcc \
        pcsc-lite-dev

COPY --from=base-paper-age /src/target/release/paper-age /usr/local/bin
COPY --from=base-age-plugin-yubikey /src/target/release/age-plugin-yubikey /usr/local/bin/
COPY --from=base-rage /src/rage/target/release/rage* /usr/local/bin/

RUN adduser -D user
USER user
WORKDIR /src
ENTRYPOINT [ "paper-age" ]
