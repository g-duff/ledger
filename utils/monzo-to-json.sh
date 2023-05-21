#!/bin/sh
./sed/to-iso-date.sed $1 | ./awk/monzo-to-json.awk account=$2
