FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY qemu.sh /
RUN bash /qemu.sh 2.8.0 arm

RUN apt-get install -y --no-install-recommends \
    gcc-arm-linux-gnueabihf

COPY arm-unknown-linux-steedeabihf.json /json

RUN apt-get install -y --no-install-recommends \
    curl && \
    curl -LSfs http://japaric.github.io/trust/install.sh | \
    sh -s -- --git japaric/xargo --tag v0.3.5 --target x86_64-unknown-linux-gnu --to /usr/bin && \
    apt-get purge --auto-remove -y curl

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_STEEDEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
