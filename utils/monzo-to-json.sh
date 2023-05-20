#!/bin/sh

to_iso_date() {
	sed -E 's/([0-9]{2})\/([0-9]{2})\/([0-9]{4})/\3-\2-\1/' $1
}

to_iso_date $1 | $PWD/awk/monzo-to-json.awk account=$2
