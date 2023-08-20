#!/bin/bash

# variables
MAP_DIR="../maps/"
SOL_DIR="./solutions/"
PROGRAM="../bsq"

# script
echo
for MAP_FILE in "$MAP_DIR"*; do
	# variables
	BASE_NAME=$(basename "$MAP_FILE")
	NUM=${BASE_NAME#map_}
	SOL_FILE=sol_$NUM

	OUTPUT=$($PROGRAM $MAP_FILE)
	EXPECTED=$(cat $SOL_DIR$SOL_FILE)

	echo -e "\033[1;33m$BASE_NAME:\033[0m"
	if [ "$OUTPUT" == "$EXPECTED" ]; then
		echo "Output   - $OUTPUT"
		echo "Solution - $EXPECTED"
		echo "✅"
		echo
	else
		echo "Output   - $OUTPUT"
		echo "Solution - $EXPECTED"
		echo "❌"
		echo
	fi
done
echo