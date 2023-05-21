filename_to_accountname() {
	basename $1 | sed 's/_/:/g; s/\.\///; s/.csv//'
}

hsbc() {
	for filename in $1/*_hsbc_*.csv; do
		[ -f $filename ] && ./utils/awk/from-hsbc.awk \
			-v account=$(filename_to_accountname $filename) \
			$filename
	done;
}

monzo() {
	for filename in $1/*_monzo_*.csv; do
		[ -f $filename ] && ./utils/awk/from-monzo.awk \
			-v account=$(filename_to_accountname $filename) \
			$filename
	done;
}

(hsbc $1; monzo $1) | sort | ./utils/awk/to-json.awk
