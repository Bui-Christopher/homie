#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 

# Source the CSV file paths
source .env

# Clean up
docker-compose down --volumes

# Set up the Postgres Database
docker-compose up -d
echo "Waiting on Postgres to start..."
tput sc
time=1
until docker exec postgres_db pg_isready -U postgres &>/dev/null; do
    tput rc; tput el; echo " ($time)"
    sleep 1
    time=$(expr $time + 1)
done
tput rc; tput el; echo "(done)"; tput cud1

DB_NAME=homie

# Initialize Tables Migration
pwd
docker exec -i postgres_db psql -U postgres -d "$DB_NAME" < migrations/20240424014039_init_tables.sql

# Write datasets to Postgres
cd ..
# TODO: Run a container instead
cargo run --bin homie-data

NAME=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT current_database();")
echo "Checking database stats for: $NAME"

# Database Size
DB_SIZE=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT pg_size_pretty(pg_database_size(current_database()));")
echo "Database size: ${DB_SIZE//[^0-9]/}"

# Count "tyields"
TYIELDS_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM tyields;")
echo "Number of rows in tyields table: $TYIELDS_COUNT"

# Count "hpis"
HPIS_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM hpis;")
echo "Number of rows in hpis table: $HPIS_COUNT"

# Count "zhvi_metadata"
ZHVI_METADATA_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM zhvi_metadata;")
echo "Number of rows in zhvi_metadata table: $ZHVI_METADATA_COUNT"

# Count "zhvi_prices"
ZHVI_PRICES_COUNT=$(docker exec postgres_db psql -U postgres -d "$DB_NAME" -tAc "SELECT count(*) FROM zhvi_prices;")
echo "Number of rows in zhvi_prices table: $ZHVI_PRICES_COUNT"

# TODO: Count Regions

unset TEN_YEAR_YIELD_PATH
unset THREE_ZIP_HPIS_PATH
unset FIVE_ZIP_HPIS_PATH
unset COUNTY_HPIS_PATH
unset MID_ZIP_ALL_HOMES_PATH
unset MID_CITY_ALL_HOMES_PATH
unset MID_COUNTY_ALL_HOMES_PATH
unset DATABASE_URL
