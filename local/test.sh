#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR"

# jq is a dependency
command -v jq >/dev/null 2>&1 || { echo >&2 "jq is required but it's not installed. Aborting."; exit 1; }

rm -rf tmp.txt

echo "Testing /health" >> tmp.txt
curl -s -X GET 'http://127.0.0.1:8080/health'
echo >> tmp.txt

echo "Testing /hpis" >> tmp.txt
curl -s -X GET 'http://127.0.0.1:8080/hpis?region_name=92841&start_date=2023-1-1&end_date=2024-12-31' | jq . >> tmp.txt
echo >> tmp.txt

echo "Testing /regions" >> tmp.txt
curl -X POST -d 'cities=GARDEN GROVE' http://127.0.0.1:8080/regions | jq . >> tmp.txt
# curl -X POST -d 'zipcodes=92841' http://127.0.0.1:8080/regions | jq . >> tmp.txt
# curl -X POST -d 'cities=IRVINE&zipcodes=92841' http://127.0.0.1:8080/regions | jq . >> tmp.txt
# curl -X POST -d '' http://127.0.0.1:8080/regions | jq . >> tmp.txt
echo >> tmp.txt

echo "Testing /tyields" >> tmp.txt
curl -s -X GET 'http://127.0.0.1:8080/tyields?start_date=2023-1-1&end_date=2024-12-31&date_interval=Year' | jq . >> tmp.txt
echo >> tmp.txt

echo "Testing /zhvis" >> tmp.txt
curl -s -X GET 'http://127.0.0.1:8080/zhvis?start_date=2023-1-1&end_date=2024-12-31&date_interval=month&home_type=AllHomes&region_type=City&region_name=Irvine&percentile=Middle' | jq . >> tmp.txt
echo >> tmp.txt

echo "Output saved to homie/local/tmp.txt"
