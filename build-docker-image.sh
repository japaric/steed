set -ex

run() {
    local tag=$(cross -V | head -n1 | cut -d' ' -f2)
    docker build \
           -t japaric/${1}:v$tag \
           -f docker/${1}/Dockerfile \
           docker
}

if [ -z $1 ]; then
    for t in `ls docker/`; do
        if [ -d docker/$t ]; then
            run $t
        fi
    done
else
    run $1
fi
