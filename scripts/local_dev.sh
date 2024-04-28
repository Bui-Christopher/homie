#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 
start_time=$(date +%s)

# Clean up old containers
echo -n "Preparing local deployment... "
docker-compose down --volumes > /dev/null 2>&1
tput sc
tput rc; tput el; echo "(done)"

# Prepare the PostgreSQL Instance
echo -n "Waiting on PostgreSQL to start... "
docker-compose up -d > /dev/null 2>&1
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
# Source the CSV file paths that are imported into DB
source .env

cd ..
echo -n "Running homie-data and importing datasets... "
tput sc
# TODO: Run a docker container instead
cargo run --bin homie-data  > /dev/null 2>&1 &
cargo_pid=$!
time=1
cols=$(tput cols)
while ps -p $cargo_pid > /dev/null; do
    tput rc; tput el
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

unset TEN_YEAR_YIELD_PATH
unset THREE_ZIP_HPIS_PATH
unset FIVE_ZIP_HPIS_PATH
unset COUNTY_HPIS_PATH
unset MID_ZIP_ALL_HOMES_PATH
unset MID_CITY_ALL_HOMES_PATH
unset MID_COUNTY_ALL_HOMES_PATH
unset DATABASE_URL

end_time=$(date +%s)
execution_time=$((end_time - start_time))
echo -e "\nTotal execution time: $execution_time seconds"
