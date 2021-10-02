#!/bin/bash

EXE="./target/debug/query";
DEST="$HOME/bin";

if [ ! -f "$EXE" ]; then
    echo "Error: Could not find executable"
    exit 1
fi

if [ ! -d "$DEST" ]; then
    mkdir -p "$DEST"
fi

cargo build && cp "$EXE" "$DEST/q"
echo "Installation successful!"
