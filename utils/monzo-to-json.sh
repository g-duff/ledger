#!/bin/sh

to_iso_date() {
	sed -E 's/([0-9]{2})\/([0-9]{2})\/([0-9]{4})/\3-\2-\1/' $1
}

to_json() {
	awk -v account=$1 'BEGIN { FS="," }
	{ if (NR>1)
		{
			print "{"
			printf "\t\"date\": \"%s\",\n",$2
			printf "\t\"description\": \"%s\",\n",$5
			print "\t\"entries\": ["
			printf "\t\t{\"account\": \"%s\", \"amount\": %.2f},\n",account,$8
			printf "\t\t{\"account\": \"__%s\", \"amount\": %.2f}\n",tolower($7),(-1*$8)
			print "\t]\n},"
		}
	}'
}

to_iso_date $1 | to_json $2
