FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY qemu.sh /
RUN apt-get install -y --no-install-recommends \
    gcc-powerpc64le-linux-gnu && \
    bash /qemu.sh 2.7.1 ppc64le

COPY powerpc64le-unknown-linux-steed.json /json

RUN apt-get install -y --no-install-recommends \
    curl && \
    curl -LSfs http://japaric.github.io/trust/install.sh | \
    sh -s -- --git japaric/xargo --tag v0.3.5 --target x86_64-unknown-linux-gnu --to /usr/bin && \
    apt-get purge --auto-remove -y curl

ENV CARGO_TARGET_POWERPC64LE_UNKNOWN_LINUX_STEED_LINKER=powerpc64le-linux-gnu-gcc \
    RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
