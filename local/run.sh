#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 

# Function to clean up and exit
cleanup_and_exit() {
    echo -e "\nCleaning up and exiting..."
    kill %1 >/dev/null 2>&1  # Kill background process
    exit 1
}

# Trap SIGINT (Ctrl + C) signal
# Stops all hanging, counting tasks
trap 'cleanup_and_exit' SIGINT
# Define ANSI color codes
NC='\033[0m'       # No color
GREEN='\033[0;32m' # Green color
RED='\033[0;31m'    # RED color
YELLOW='\033[1;33m' # Yellow color

echo -e "Running local homie environment\n"
start_time=$(date +%s)

# Clean up old containers and network
echo -n "Cleaning up previous environment... "
tput sc
time=1
cols=$(tput cols)
while true; do
    time_position=$((cols - ${#time}))
    tput rc; tput el;
    printf "${YELLOW}(%s)${NC}" "$time"
    sleep 1
    time=$(expr $time + 1)
done &
loop_pid=$!

docker stop postgres_db &> /dev/null
docker rm postgres_db &> /dev/null
docker volume rm database_data &> /dev/null

docker stop homie_data &> /dev/null
docker rm homie_data &> /dev/null

docker stop homie_api &> /dev/null
docker rm homie_api &> /dev/null

docker network rm homie_network &> /dev/null
kill -TERM $loop_pid >/dev/null 2>&1
wait $loop_pid 2>/dev/null
tput rc; tput el; echo -e "(${GREEN}done${NC})"

# Check if homie images exist
echo "Checking if homie images exist... "
col=$(printf "Checking if homie images exist... " | wc -c)
row=0
if ! docker images | grep -q "homie/homie-api"; then
    row=$(expr $row + 1)
    tput sc
    tput cuu $row; tput cuf $col; tput el; printf "(${YELLOW}not found${NC})"
    tput rc
    echo -en "\thomie-api image building... "
    # Save cursor position
    tput sc
    time=0
    cols=$(tput cols)
    while true; do
        time_position=$((cols - ${#time}))
        # Restore cursor position
        tput rc; tput el;
        printf "${YELLOW}(%s)${NC}" "$time"
        sleep 1
        time=$(expr $time + 1)
    done &
    loop_pid=$!
    ./../docker/homie-api/build.sh &> /dev/null
    kill -TERM $loop_pid >/dev/null 2>&1
    wait $loop_pid 2>/dev/null
    # Restore cursor position
    tput rc; tput el; echo -e "(${GREEN}done${NC})"
fi

if ! docker images | grep -q "homie/homie-data"; then
    row=$(expr $row + 1)
    tput sc
    tput cuu $row; tput cuf $col; tput el; printf "(${YELLOW}not found${NC})"
    tput rc
    echo -en "\thomie-data image building... "
    # Save cursor position
    tput sc
    time=1
    cols=$(tput cols)
    while true; do
        time_position=$((cols - ${#time}))
        # Restore cursor position
        tput rc; tput el;
        printf "${YELLOW}(%s)${NC}" "$time"
        sleep 1
        time=$(expr $time + 1)
    done &
    loop_pid=$!
    ./../docker/homie-data/build.sh &> /dev/null
    kill -TERM $loop_pid >/dev/null 2>&1
    wait $loop_pid 2>/dev/null
    # Restore cursor position
    tput rc; tput el; echo -e "(${GREEN}done${NC})"
fi
tput sc
tput cuu $(expr $row + 1); tput cuf $col; tput el; printf "(${GREEN}done${NC})"
tput rc

# Create a user-defined bridge network
echo -n "Creating Docker network... "
tput sc
docker network create homie_network &> /dev/null
tput rc; tput el; echo -e "(${GREEN}done${NC})"

# Prepare the PostgreSQL Instance
echo -n "Waiting on PostgreSQL to start... "
tput sc
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

time=0
cols=$(tput cols)
until docker exec postgres_db pg_isready -U postgres &> /dev/null; do
    tput rc; tput el
    time_position=$((cols - ${#time}))
    printf "${YELLOW}(%s)${NC}" "$time"
    sleep 2
    time=$(expr $time + 1)
done
tput rc; tput el; echo -e "(${GREEN}done${NC})"

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
    printf "${YELLOW}(%s)${NC}" "$time"
    sleep 1
    time=$(expr $time + 1)
done
tput rc; tput el; echo -e "(${GREEN}done${NC})"

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
tput rc; tput el; echo -e "(${GREEN}done${NC})"
echo -e "\tDatabase name: $NAME"
echo -e "\tDatabase size: ${DB_SIZE} MB"
echo -e "\tNumber of rows in hpis table: $HPIS_COUNT"
# echo -e "\tNumber of rows in regions table: $REGIONS_COUNT"
echo -e "\tNumber of rows in tyields table: $TYIELDS_COUNT"
echo -e "\tNumber of rows in zhvi_metadata table: $ZHVI_METADATA_COUNT"
echo -e "\tNumber of rows in zhvi_prices table: $ZHVI_PRICES_COUNT"

echo -n "Running homie-api to serve datasets... "
tput sc
docker run -d \
    --name homie_api \
    --network homie_network \
    --env-file .docker.env \
    -p 8080:8080 \
    homie/homie-api:0.1.0 \
    &> /dev/null

sleep 2
response=$(curl -s -X GET 'http://localhost:8080/health')
expected_response="Service is running."
if [ $? -eq 0 ] && [ "$response" = "$expected_response" ]; then
    tput rc; tput el; echo -e "(${GREEN}success${NC})"
else
    tput rc; tput el; echo -e "(${RED}failed${NC})"
fi
end_time=$(date +%s)
execution_time=$((end_time - start_time))
echo -e "\nTotal execution time: $execution_time seconds"
