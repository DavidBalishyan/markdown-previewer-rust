#!/bin/sh

is_installed() {
    if command -v $1 &> /dev/null
    then
        return 0
    else
        return 1
    fi
}

if ! is_installed rustup
then
    echo "Error: rustup is not installed. Please install it and try again."
    exit 1
fi


if ! is_installed rustc
then
    echo "Error: the rust compiler is not installed. Please install it and try again."
    exit 1
fi

if ! is_installed just
then
    echo "Error: just is not installed. Please install it and try again."
    exit 1
fi
