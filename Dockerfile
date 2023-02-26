FROM rust:alpine3.17 as base
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

RUN cd /src && cargo build --release --features=compression

FROM alpine:3.17 as tool

RUN apk update && apk add libgcc

COPY --from=base /src/target/release/paper-age /usr/local/bin

RUN adduser -D user
USER user
WORKDIR /src
ENTRYPOINT [ "paper-age" ]
