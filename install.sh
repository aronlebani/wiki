#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: ./install.sh <path>"
fi

if [ -f "./target/debug/query" ]; then
    cp ./target/debug/query $1/q
    echo "Installation successful!"
else
    echo "Error: Could not find executable"
fi

