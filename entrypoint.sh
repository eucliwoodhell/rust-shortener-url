#!/bin/bash
set -e

cd /app
echo en /app
if [[ $# -eq 0 ]]; then
    exec ./sai-queue-parser
else
    exec $@
fi
