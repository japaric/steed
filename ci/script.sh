set -ex

main() {
    local examples=(
        create
        format
        hello
        instant
        open
        preadwrite
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

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
