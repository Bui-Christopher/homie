#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 

source .env
cd ..

cargo run > tmp.txt

unset TEN_YEAR_YIELD_PATH
unset THREE_ZIP_HPIS_PATH
unset FIVE_ZIP_HPIS_PATH
unset COUNTY_HPIS_PATH
unset MID_ZIP_ALL_HOMES_PATH
unset MID_CITY_ALL_HOMES_PATH
unset MID_COUNTY_ALL_HOMES_PATH
