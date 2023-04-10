#!/bin/sh

to_iso_date() {
	sed -E 's/^([0-9]{2})\/([0-9]{2})\/([0-9]{4})/\3-\2-\1/' $1
}

to_json() {
	awk -v account=$1 'BEGIN { FS="," }
	{
		amount = $3$4;
		gsub(/\"/, "", amount);

		print "{"
		printf "\t\"date\": \"%s\",\n",$1
		printf "\t\"description\": \"%s\",\n",$2
		print "\t\"entries\": ["
		printf "\t\t{\"account\": \"%s\", \"amount\": %.2f},\n",account,amount
		printf "\t\t{\"account\": \"__\", \"amount\": %.2f}\n",(-1*amount)
		print "\t]\n},"
	}'
}

to_iso_date $1 | sort | to_json $2
