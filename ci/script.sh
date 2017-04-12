set -ex

main() {
    sh build-docker-image.sh $TARGET

    if [ $TRAVIS_BRANCH = master ]; then
        return
    fi

    local examples=(
        _llseek
        args
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
              --features naive_ralloc \
              --example $example
    done

    for example in ${examples[@]}; do
        cross run \
              --target $TARGET \
              --no-default-features \
              --features naive_ralloc \
              --example $example --release
    done

    cat >>Xargo.toml <<'EOF'

[dependencies.std]
default-features = false
features = ["naive_ralloc"]
path = "/project"
stage = 2

[dependencies.test]
path = "/project/test"
stage = 3
EOF

    cross test \
          --target $TARGET \
          --no-default-features \
          --features naive_ralloc

    set +x
    pushd target/$TARGET/release/examples
    size ${examples[@]}
    popd
}

main
