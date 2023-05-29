#! /usr/bin/awk -f
# ./tsb-to-json.awk -v account='account name' /path/to/account-name.csv

BEGIN { FS=","; OFS="," }
{ if (NR>1)
	{
		amount = $6=="" ? $7 : $6;
		date = $1;
		description = $5;

		print date, account, amount, description;
	}
}
