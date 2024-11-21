#!/usr/bin/env bash
set -e

# Declare Containerization Tool and Images
SCRIPT_DIR="$( cd "$( dirname "$0" )" && pwd )"
cd "$SCRIPT_DIR"

if command -v podman &>/dev/null; then
    CONTAINER_TOOL="podman"
elif command -v docker &>/dev/null; then
    CONTAINER_TOOL="docker"
else
    echo "Neither Podman or Docker found, exiting..."
    exit 1
fi
CONTAINER_REGISTRY="ghcr.io/Bui-Christopher/homie/"

API_TAG="latest"
API_IMAGE="homie-api:$API_TAG"

DATA_TAG="latest"
DATA_IMAGE="homie-data:$DATA_TAG"

POSTGRES_TAG="16.2-alpine3.19"
POSTGRES_IMAGE="postgres:$POSTGRES_TAG"

# Pull Images
echo "Pulling images..."
"$CONTAINER_TOOL" pull "$CONTAINER_REGISTRY/$API_IMAGE"
"$CONTAINER_TOOL" pull "$CONTAINER_REGISTRY/$DATA_IMAGE"
"$CONTAINER_TOOL" pull "$CONTAINER_REGISTRY/$POSTGRES_IMAGE"

# Declare and Clean Up Previous Environment
POSTGRES="postgres_db"
POSTGRES_DATA="postgres_data"
DATA="homie-data"
API="homie-api"
NETWORK="homie_network"

echo "Cleaning up previous containers and volumes..."
"$CONTAINER_TOOL" stop "$POSTGRES" || true
"$CONTAINER_TOOL" rm "$POSTGRES" || true
"$CONTAINER_TOOL" volume rm "$POSTGRES_DATA" || true

"$CONTAINER_TOOL" stop "$DATA" || true
"$CONTAINER_TOOL" rm "$DATA" || true

"$CONTAINER_TOOL" stop "$API" || true
"$CONTAINER_TOOL" rm "$API" || true

"$CONTAINER_TOOL" network rm "$NETWORK" || true

# Start the Network, Database, and Services
echo "Creating network..."
"$CONTAINER_TOOL" network create "$NETWORK"

# Initialize Database
echo "Starting PostgreSQL container..."
DB_NAME="homie"
"$CONTAINER_TOOL" run -d \
  --name "$POSTGRES" \
  --network "$NETWORK" \
  -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB="$DB_NAME" \
  -v "$POSTGRES_DATA":/var/lib/postgresql/data \
  $POSTGRES_IMAGE \

until $CONTAINER_TOOL exec "$POSTGRES" pg_isready -U postgres > /dev/null 2>&1; do
  echo "Waiting for PostgreSQL to become ready..."
  sleep 3
done

# Import the schema
echo "Importing initial database schema..."
if ! $CONTAINER_TOOL exec -i "$POSTGRES" psql -U postgres -d "$DB_NAME" < datasets/migrations/20240424014039_init_tables.sql; then
  echo "Error: Failed to import the schema"
  exit 1
fi

# Import Local Datasets
echo "Starting data import..."
"$CONTAINER_TOOL" run -d \
    --name $DATA \
    --network $NETWORK \
    --env-file .container.env \
    -v "$SCRIPT_DIR"/datasets:/datasets \
    "$DATA_IMAGE" \
    &> /dev/null

# Start Endpoints
echo "Starting API endpoints..."
"$CONTAINER_TOOL" run -d \
    --name $API \
    --network $NETWORK \
    --env-file .container.env \
    -p 8080:8080 \
    "$API_IMAGE" \
    &> /dev/null

echo "All services have been started successfully."
