#!/bin/sh
./sed/to-iso-date.sed $1 | sort | ./awk/hsbc-to-json.awk -v account=$2
