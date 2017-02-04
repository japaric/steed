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
        cross run --target $TARGET --no-default-features --features naive_ralloc --example $example
    done

    for example in ${examples[@]}; do
        cross run --target $TARGET --no-default-features --features naive_ralloc --example $example --release
    done

    set +x
    pushd target/$TARGET/release/examples
    size ${examples[@]}
    popd
}

main
