#!/bin/sh

# This script will output models context in "server_generator/debug/out" while generating server stub.    
# Json files stored in "server_generator/debug/models_context" are processed based on the output of this script.

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
source "$SCRIPT_DIR/common.sh"

# The command below also generates server stub files
# But they will be placed in "/tmp/templates" in docker as we don't need them. 
echo -e "Generating models context\nwith specification: $specification\nwith templates: $ROOT_DIR/$TEMPLATES_PATH"
docker run --rm -v "$ROOT_DIR:/local" $image generate \
    -i "$specification" \
    -t "/local/$TEMPLATES_PATH" \
    -g rust-server \
    -o "/tmp/templates" \
    --api-package digivine \
    --package-name digivine-server \
    --global-property debugModels \
    --skip-validate-spec > $ROOT_DIR/server_generator/debug/out


