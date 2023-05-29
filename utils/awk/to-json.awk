#! /usr/bin/awk -f
# ./to-json.awk /path/to/normalised.csv

BEGIN { FS="," }
{
	account = $2;
	amount = $3;
	date = $1;
	description = $4;

	printf "{\n"\
		"\t\"date\": \"%s\",\n"\
		"\t\"description\": \"%s\",\n"\
		"\t\"entries\": [\n"\
		"\t\t{\"account\": \"%s\", \"amount\": %.2f},\n"\
		"\t\t{\"account\": \"__\", \"amount\": %.2f}\n"\
		"\t]\n},\n",date,description,account,amount,(-1*amount)
}
