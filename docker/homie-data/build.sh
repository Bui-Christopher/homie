#!/bin/sh
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
VERSION=$(grep -E '^version\s*=' "$REPO_ROOT/homie-data/Cargo.toml" | awk -F'"' '{print $2}')
TAG="homie/homie-data:$VERSION"

if [[ $OSTYPE == 'darwin'* ]]; then
    DOCKERFILE="$DIR/Dockerfile.darwin"
else
    DOCKERFILE="$DIR/Dockerfile"
fi

docker build \
    -f "$DOCKERFILE" "$REPO_ROOT" \
    -t "$TAG" \
    --no-cache
