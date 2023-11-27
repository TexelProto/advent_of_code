#!/bin/bash

# Check if directory and file names are provided
if [ -z "$1" ]; then
    echo "Year name not provided."
    exit 1
fi
if [ -z "$2" ]; then
    echo "File name not provided."
    exit 1
fi

# Set the directory and file names
if [ ! -d "$1" ]; then
    echo "Directory $1 does not exist."
    exit 1
fi

cp "_daytemplate.rs" "$1/src/$2.rs"
echo "Created ./$1/src/$2.rs"

touch "inputs/$1/$2.txt"
echo "Created ./inputs/$1/$2.txt"
