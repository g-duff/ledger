# Use: ./utils/scratch.sh /Directory-of-csv-statements

load_bank_statements() {
	for bank in 'hsbc' 'monzo'; do
		process_bank_statements $bank $1
	done;
}

process_bank_statements() {
	# Use: process_bank_statements bank-name csv-directory
	for filename in $2/*$1*.csv; do
		[ -f $filename ] && ./utils/awk/from-$1.awk \
			-v account=$(filename_to_accountname $filename) \
			$filename
	done;
}

filename_to_accountname() {
	basename $1 | sed -E 's/-|_/:/g; s/\.\///; s/.csv//'
}

(load_bank_statements $1) | sort | ./utils/awk/to-json.awk
