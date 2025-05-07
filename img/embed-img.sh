#!/bin/bash

if [ $# -ne 3 ]; then
    echo "Usage: $0 <input_file> -o <output_path>"
    exit 1
fi

INPUT_FILE="$1"

if [ "$2" != "-o" ]; then
    echo "Error: Expected '-o' as second argument"
    exit 1
fi

OUTPUT_PATH="$3"

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' not found"
    echo "No such file"
    exit 1
fi


set -e

echo "// EMBEDED IMG FILE GENERATED embed-img.sh" > "$OUTPUT_PATH"
xxd -i "$INPUT_FILE" >> "$OUTPUT_PATH"

echo "Completed embededing binary file to $OUTPUT_PATH"
