#! /usr/bin/awk -f
# ./monzo-to-json.awk -v account='account:name' /path/to/account-name.csv

BEGIN { FS="," }
{ if (NR>1)
	{
		date = $2;
		amount = $8;
		description = $5;
		
		print "{";
		printf "\t\"date\": \"%s\",\n",date;
		printf "\t\"description\": \"%s\",\n",description;
		print "\t\"entries\": [";
		printf "\t\t{\"account\": \"%s\", \"amount\": %.2f},\n",account,amount;
		printf "\t\t{\"account\": \"__\", \"amount\": %.2f}\n",(-1*amount);
		print "\t]\n},";
	}
}
