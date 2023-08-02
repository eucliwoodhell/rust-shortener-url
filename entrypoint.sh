#!/bin/bash
set -e

cd /app
echo en /app
if [[ $# -eq 0 ]]; then
    exec ./target/debug/rust-shortener-url
else
    exec $@
fi
