#!/bin/bash

EXE="./target/debug/md-query";
DEST="/usr/local/bin";

if [ ! -f "$EXE" ]; then
    echo "Error: Could not find executable"
    exit 1
fi

if [ ! -d "$DEST" ]; then
    echo "Error: Destination directory does not exist"
    exit 1
fi

cargo build && cp "$EXE" "$DEST/wiki"
echo "Installation successful!"
