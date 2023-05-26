# Use: ./utils/scratch.sh /Directory-of-csv-statements

filename_to_accountname() {
	basename $1 | sed -E 's/-|_/:/g; s/\.\///; s/.csv//'
}

process_bank_statements() {
	# Use: process_bank_statements bank-name csv-directory
	for filename in $2/*$1*.csv; do
		[ -f $filename ] && ./utils/awk/from-$1.awk \
			-v account=$(filename_to_accountname $filename) \
			$filename
	done;
}

alias process_hsbc='process_bank_statements hsbc'
alias process_monzo='process_bank_statements monzo'

(process_hsbc $1; process_monzo $1) | sort | ./utils/awk/to-json.awk
