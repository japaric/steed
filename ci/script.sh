set -ex

main() {
    local examples=(
        create
        format
        hello
        instant
        open
        stderr
        system-time
        vec
        zero
    )

    for example in ${examples[@]}; do
        cross run --target $TARGET --example $example
    done

    for example in ${examples[@]}; do
        # NOTE linker errors
        if [ $TARGET = armv7-unknown-linux-gnueabihf ] && [ $example = open ]; then
            continue
        fi
        if [ $TARGET = armv7-unknown-linux-gnueabihf ] && [ $example = vec ]; then
            continue
        fi

        if [ $TARGET = i686-unknown-linux-gnu ] && [ $example = vec ]; then
            continue
        fi

        if [ $TARGET = i686-unknown-linux-gnu ] && [ $example = instant ]; then
            continue
        fi

        if [ $TARGET = armv7-unknown-linux-gnueabihf ] && [ $example = instant ]; then
            continue
        fi

        if [ $TARGET = x86_64-unknown-linux-gnu ] && [ $example = vec ]; then
            continue
        fi

        cross run --target $TARGET --example $example --release
    done
}

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
