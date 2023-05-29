#! /usr/bin/awk -f
# ./monzo-to-json.awk -v account='account name' /path/to/account-name.csv

BEGIN { FS=","; OFS="," }
{ if (NR>1)
	{
		amount = $8;

		split($2, day_month_year, "/");
		date = day_month_year[3]"-"day_month_year[2]"-"day_month_year[1];

		description = $5;
		gsub(/\"/, "", description);
		
		print date, account, amount, description;
	}
}
