#!/bin/bash

# Check if year number is provided
if [ -z "$1" ]; then
    echo "Year number not provided."
    exit 1
fi

# Check if directory "aoc_year" exists
if [ ! -d "aoc_$1" ]; then
    echo "Directory 'aoc_$1' does not exist."
    exit 1
fi

mkdir "aoc_$1/docs"

# Loop from 01 to 25 for the second argument
for ((i=1; i<=25; i++)); do
    echo "Downloading $i"
    PADDED_NUM=$(printf "%02d" $i)
    OUTPUT="aoc_$1/docs/day${PADDED_NUM}.md"
    python download_desc.py $1 $PADDED_NUM $2 > "$OUTPUT"
done
