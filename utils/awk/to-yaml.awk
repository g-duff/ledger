#! /usr/bin/awk -f
# ./to-json.awk -v min_date='<iso 8601 date>' /path/to/normalised.csv

BEGIN { FS="," }
{
	account = $2;
	amount = $3;
	date = $1;
	description = $4;

	if (date >= min_date) {
		printf "- date: '%s'\n"\
		       "  description: %s\n"\
		       "  entries:\n"\
		       "  - account: %s\n"\
		       "    amount: %.2f\n"\
		       "  - account: '__'\n"\
		       "    amount: %.2f\n",
		       date,description,account,amount,(-1*amount)
	}
}
