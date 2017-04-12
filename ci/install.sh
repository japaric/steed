set -ex

main() {
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag v0.1.10 \
           --target x86_64-unknown-linux-musl
}

main
