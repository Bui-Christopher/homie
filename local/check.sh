#!/bin/bash

DB_NAME=homie
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
