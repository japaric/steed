set -ex

main() {
    local tag=v0.3.6
    local dependencies=(
        ca-certificates
        curl
    )

    apt-get update
    local purge_list=()
    for dep in ${dependencies[@]}; do
        if ! dpkg -L $dep; then
            apt-get install --no-install-recommends -y $dep
            purge_list+=( $dep )
        fi
    done

    curl -LSfs http://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --git japaric/xargo \
           --tag $tag \
           --target x86_64-unknown-linux-gnu \
           --to /usr/bin

    # Clean up
    apt-get purge --auto-remove -y ${purge_list[@]}

    rm $0
}

main "${@}"
