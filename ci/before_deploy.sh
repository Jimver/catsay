#!/usr/bin/env bash

# This script takes care of building your crate and packaging it for release

set -ex

main() {
    if [ $TARGET = 'snap' ]
    then
        return
    fi

    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac


    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin catsay --target $TARGET --release -- -C lto

    if [ $TARGET = 'x86_64-pc-windows-gnu' ]
    then
        cp target/$TARGET/release/catsay.exe $stage/
    else
        cp target/$TARGET/release/catsay $stage/
    fi


    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main