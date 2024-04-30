#!/bin/sh
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
DOCKERFILE="$DIR/Dockerfile"
VERSION=$(grep -E '^version\s*=' "$REPO_ROOT/homie-data/Cargo.toml" | awk -F'"' '{print $2}')
TAG="homie/homie-data:$VERSION"

docker build \
    -f "$DOCKERFILE" "$REPO_ROOT" \
    -t "$TAG" \
