#!/bin/sh

image="openapitools/openapi-generator-cli:v5.4.0"
specification="https://raw.githubusercontent.com/DigiVine-DE/API-specification/main/bundled/openapi.yaml"

echo "Pulling $image"
docker pull $image

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
ROOT_DIR=$SCRIPT_DIR/../..

TEMPLATES_PATH=server_generator/templates
SERVER_STUB_PATH=libs/digivine-api-server
