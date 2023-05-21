#! /usr/bin/awk -f
# ./tsb-to-json.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS="," }
{
	amount = $6=="" ? $7 : $6;
	date = $1;
	description = $5;

	printf "%s,%s,%s,%s\n", date, account, amount, description;
}
