set -ex

main() {
    sh build-docker-image.sh $TARGET

    if [ $TRAVIS_BRANCH = master ]; then
        return
    fi

    local examples=(
        _llseek
        create
        dup
        format
        hashmap
        hello
        instant
        ls
        open
        preadwrite
        stat
        stderr
        system-time
        tcp_listen_connect
        vec
        zero
    )

    for example in ${examples[@]}; do
        cross run \
              --target $TARGET \
              --no-default-features \
              --features "compiler-builtins naive_ralloc" \
              --example $example
    done

    for example in ${examples[@]}; do
        cross run \
              --target $TARGET \
              --no-default-features \
              --features "compiler-builtins naive_ralloc" \
              --example $example --release
    done

    cat >>Xargo.toml <<'EOF'

[dependencies.std]
default-features = false
features = ["compiler-builtins", "naive_ralloc"]
path = "/project"
stage = 1

[dependencies.test]
path = "/project/test"
stage = 2
EOF

    cross test \
          --target $TARGET \
          --no-default-features \
          --features "naive_ralloc"

    set +x
    pushd target/$TARGET/release/examples
    size ${examples[@]}
    popd
}

main
