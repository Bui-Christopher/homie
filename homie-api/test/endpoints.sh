#!/bin/bash

echo "Testing /health"
curl -s -X GET 'http://127.0.0.1:8080/health'
echo
echo

echo "Testing /hpis"
curl -s -X GET 'http://127.0.0.1:8080/hpis?region_type=type1&region_id=id1&state_date=date1&end_date=date2&interval=year&annual_change=true&base_2000=false' | jq .
echo
echo

echo "Testing /yields"
curl -s -X GET 'http://127.0.0.1:8080/yields?state_date=date1&end_date=date2&interval=month' | jq .
echo
echo

echo "Testing /zhvis"
curl -s -X GET 'http://127.0.0.1:8080/zhvis?state_date=date1&end_date=date2&interval=day&region_type=type1&region_id=id1&percentile=percentile1&home_type=type2' | jq .
echo
echo
