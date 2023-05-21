#! /usr/bin/awk -f
# ./monzo-to-json.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS="," }
{ if (NR>1)
	{
		date = $2;
		amount = $8;
		description = $5;
		gsub(/\"/, "", description);
		
		printf "%s,%s,%s,%s\n", date, account, amount, description;
	}
}
