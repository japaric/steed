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
        format
        hello
        instant
        open
        preadwrite
        stat
        stderr
        system-time
        vec
        zero
    )

    for example in ${examples[@]}; do
        cross run --target $TARGET --example $example
    done

    for example in ${examples[@]}; do
        cross run --target $TARGET --example $example --release
    done
}

main
