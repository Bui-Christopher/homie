#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 
start_time=$(date +%s)

# Clean up old containers
echo -n "Preparing local deployment... "
tput sc
docker stop postgres_db &> /dev/null
docker rm postgres_db &> /dev/null
docker volume rm database_data &> /dev/null

docker stop homie_data &> /dev/null
docker rm homie_data &> /dev/null

docker stop homie_api &> /dev/null
docker rm homie_api &> /dev/null

# Clean up old network bridge
docker network rm homie_network &> /dev/null
tput rc; tput el; echo "(done)"

# Create a user-defined bridge network
echo -n "Creating Docker network... "
tput sc
docker network create homie_network &> /dev/null
tput rc; tput el; echo "(done)"

# Prepare the PostgreSQL Instance
echo -n "Waiting on PostgreSQL to start... "
docker run -d \
  --name postgres_db \
  --network homie_network \
  -p 5433:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=homie \
  -v database_data:/var/lib/postgresql/data \
  postgres:16.2-alpine3.19 \
  &> /dev/null

tput sc
time=1
cols=$(tput cols)
until docker exec postgres_db pg_isready -U postgres &> /dev/null; do
    tput rc; tput el
    time_position=$((cols - ${#time}))
    printf "(%s)" "$time"
    sleep 1
    time=$(expr $time + 1)
done
tput rc; tput el; echo "(done)"

# Initialize Tables
DB_NAME=homie
docker exec -i postgres_db psql -U postgres -d "$DB_NAME" < migrations/20240424014039_init_tables.sql > /dev/null 2>&1

# Write datasets to Postgres
echo -n "Running homie-data and importing datasets... "
docker run -d \
  --name homie_data \
  --network homie_network \
  --env-file .docker.env \
  -v "$DIR"/datasets:/datasets \
  homie/homie-data:0.1.0 \
  &> /dev/null
sleep 2

tput sc
time=1
cols=$(tput cols)
until [ "$(docker inspect -f '{{.State.Running}}' homie_data)" = "false" ]; do
    tput rc; tput el
    time_position=$((cols - ${#time}))
    printf "(%s)" "$time"
    sleep 1
    time=$(expr $time + 1)
done
tput rc; tput el; echo "(done)"

# Ensure datasets were properly imported
echo -n "Checking database stats... "
tput sc

# Gather DB Status
# Database Name
NAME=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT current_database();")
# Database Size
DB_SIZE=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT pg_database_size(current_database()) / (1024 * 1024);")
# Count "hpis"
HPIS_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM hpis;")
# Count "tyields"
TYIELDS_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM tyields;")
# TODO: Count Regions
# REGIONS_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM regions;")
# Count "zhvi_metadata"
ZHVI_METADATA_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM zhvi_metadata;")
# Count "zhvi_prices"
ZHVI_PRICES_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM zhvi_prices;")

# Print DB Status
tput rc; tput el; echo "(done)"
echo -e "\tDatabase name: $NAME"
echo -e "\tDatabase size: ${DB_SIZE} MB"
echo -e "\tNumber of rows in hpis table: $HPIS_COUNT"
# echo -e "\tNumber of rows in regions table: $REGIONS_COUNT"
echo -e "\tNumber of rows in tyields table: $TYIELDS_COUNT"
echo -e "\tNumber of rows in zhvi_metadata table: $ZHVI_METADATA_COUNT"
echo -e "\tNumber of rows in zhvi_prices table: $ZHVI_PRICES_COUNT"

echo -n "Running homie-api to serve datasets... "
docker run -d \
  --name homie_api \
  --network homie_network \
  --env-file .docker.env \
  -p 8080:8080 \
  homie/homie-api:0.1.0 \
  # &> /dev/null

echo "Curling and checking its health:"
curl -s -X GET 'http://127.0.0.1:8080/health'

end_time=$(date +%s)
execution_time=$((end_time - start_time))
echo -e "\nTotal execution time: $execution_time seconds"
