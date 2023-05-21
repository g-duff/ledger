#! /usr/bin/awk -f
# ./from-hsbc.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS="," }
{
	amount = $3$4;
	gsub(/\"/, "", amount);
	gsub(/\r/, "", amount);

	date = $1;
	description = $2;
	gsub(/\"/, "", description);

	printf "%s,%s,%s,%s\n", date, account, amount, description;
}
