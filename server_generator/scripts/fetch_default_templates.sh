#!/bin/sh

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
source "$SCRIPT_DIR/common.sh"

echo "fetch default templates"
docker run --rm -v "$ROOT_DIR:/local" $image author template -g rust-server  -o "/local/$TEMPLATES_PATH"


