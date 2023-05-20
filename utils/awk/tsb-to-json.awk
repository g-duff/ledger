#! /usr/bin/awk -f
# ./tsb-to-json.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS="," }
{
	amount = $6=="" ? $7 : $6;
	date = $1;
	description = $5;

	print "{";
	printf "\t\"date\": \"%s\",\n",date;
	printf "\t\"description\": \"%s\",\n",description;
	print "\t\"entries\": [";
	printf "\t\t{\"account\": \"%s\", \"amount\": %.2f},\n",account,amount;
	printf "\t\t{\"account\": \"__\", \"amount\": %.2f}\n",(-1*amount);
	print "\t]\n},";
}
