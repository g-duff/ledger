filename_to_accountname() {
	echo $1 | sed 's/_/:/g; s/\.\///; s/.csv//'
}

for filename in ./*_hsbc_*.csv; do
        ./hsbc-to-json.sh \
		$filename \
		$(filename_to_accountname $filename);
done;

for filename in ./*_monzo_*.csv; do
	./monzo-to-json.sh \
		$filename \
		$(filename_to_accountname $filename);
done;
