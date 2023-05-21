#! /usr/bin/awk -f
# ./to-json.awk /path/to/normalised.csv

BEGIN { FS="," }
{
	date = $1;
	account = $2;
	amount = $3;
	description = $4;

	print "{";
	printf "\t\"date\": \"%s\",\n",date;
	printf "\t\"description\": \"%s\",\n",description;
	print "\t\"entries\": [";
	printf "\t\t{\"account\": \"%s\", \"amount\": %.2f},\n",account,amount;
	printf "\t\t{\"account\": \"__\", \"amount\": %.2f}\n",(-1*amount);
	print "\t]\n},";
}
