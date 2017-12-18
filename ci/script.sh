set -ex

main() {
    sh build-docker-image.sh $TARGET

    if [ $TRAVIS_BRANCH = master ]; then
        return
    fi

    local examples=(
        _llseek
        args
        chdir
        create
        dup
        env
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
        thread
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

    # blow the old Xargo directory to avoid like "found possibly newer version of crate"
    rm -rf $HOME/.xargo

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
