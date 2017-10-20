FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY xargo.sh /
RUN bash /xargo.sh

COPY lld.sh /
RUN bash /lld.sh

COPY qemu.sh /
RUN bash /qemu.sh 2.10.1 arm

COPY armv7-unknown-linux-steedeabihf.json /json

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_STEEDEABIHF_RUNNER=qemu-arm \
    RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
