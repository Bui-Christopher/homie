#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "$DIR" 

# jq is a dependency
rm -rf tmp.txt

echo "Testing /health"
curl -s -X GET 'http://127.0.0.1:8080/health'

echo "Testing /hpis"
curl -s -X GET 'http://127.0.0.1:8080/hpis?region_name=92841&start_date=2023-1-1&end_date=2024-12-31' | jq . >> tmp.txt

echo "Testing /tyields"
curl -s -X GET 'http://127.0.0.1:8080/tyields?start_date=2023-1-1&end_date=2024-12-31&date_interval=Year' | jq . >> tmp.txt

echo "Testing /zhvis"
curl -s -X GET 'http://127.0.0.1:8080/zhvis?start_date=2023-1-1&end_date=2024-12-31&date_interval=year&home_type=AllHomes&region_type=City&region_name=Irvine&percentile=Middle' | jq . >> tmp.txt

cat tmp.txt
