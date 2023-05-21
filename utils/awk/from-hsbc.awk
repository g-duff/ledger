#! /usr/bin/awk -f
# ./from-hsbc.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS=","; OFS=","}
{
	amount = $3$4;
	gsub(/\"/, "", amount);
	gsub(/\r/, "", amount);

	split($1, day_month_year, "/");
	date = day_month_year[3]"-"day_month_year[2]"-"day_month_year[1];

	description = $2;
	gsub(/\"/, "", description);

	print date, account, amount, description;
}
