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

# Initialize Tables Migration
sqlx migrate run

# Write datasets to Postgres
cd ..
pwd
cargo run --bin homie-data

unset TEN_YEAR_YIELD_PATH
unset THREE_ZIP_HPIS_PATH
unset FIVE_ZIP_HPIS_PATH
unset COUNTY_HPIS_PATH
unset MID_ZIP_ALL_HOMES_PATH
unset MID_CITY_ALL_HOMES_PATH
unset MID_COUNTY_ALL_HOMES_PATH
unset DATABASE_URL
