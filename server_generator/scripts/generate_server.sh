#!/bin/sh
set -e

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
source "$SCRIPT_DIR/common.sh"

echo -e "Creating/Updating Server Stubs and Models\nwith specification: $specification\nwith templates: $ROOT_DIR/$TEMPLATES_PATH"
docker run --rm -v "$ROOT_DIR:/local" -u "$UID:$UID" $image generate \
    -i "$specification" \
    -t "/local/$TEMPLATES_PATH" \
    -g rust-server \
    -o "/local/$SERVER_STUB_PATH" \
    --api-package digivine \
    --package-name digivine-server \
    --skip-validate-spec

echo "Cleaning up server lib"
cd "$ROOT_DIR/$SERVER_STUB_PATH"
cargo clippy --fix -Z unstable-options --allow-dirty --allow-staged
cargo fmt

