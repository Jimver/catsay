#!/usr/bin/env bash

# This script takes care of testing your crate

set -ex

main() {
    if [ $TARGET = snap ]
    then
        docker run -v $(pwd):$(pwd) -w $(pwd) snapcore/snapcraft sh -c "apt update && snapcraft"
        return
    fi

    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release

    cross run --target $TARGET test
    cross run --target $TARGET --release test_release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi