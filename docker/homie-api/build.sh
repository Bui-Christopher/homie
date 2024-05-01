#!/bin/sh
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
DOCKERFILE="$DIR/Dockerfile"
VERSION=$(grep -E '^version\s*=' "$REPO_ROOT/homie-api/Cargo.toml" | awk -F'"' '{print $2}')
TAG="homie/homie-api:$VERSION"

if [[ $OSTYPE == 'darwin'* ]]; then
    DOCKERFILE="$DIR/Dockerfile.darwin"
else
    DOCKERFILE="$DIR/Dockerfile"
fi

docker build \
    -f "$DOCKERFILE" "$REPO_ROOT" \
    -t "$TAG" \
