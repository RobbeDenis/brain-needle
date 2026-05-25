#!/bin/bash

BF_QUERY=$1

# Colors
export NC="\e[0m"
export RED="\e[31m"
export GREEN="\e[32m"
export UNDL="\e[4;37m"
export BOLD="\e[1;97m"

# Usage
if [ -z "$BF_QUERY" ]; then
    echo -e "$Usage: $0 <file query in ./bf>"
    exit 1
fi

# Find matching files
FOUND_FILES=$(find bf -type f -name "*$BF_QUERY*")

if [ -z "$FOUND_FILES" ]; then
    echo -e "${RED}Error${NC}: No files matching '$BF_QUERY' found in ./bf"
    exit 1
fi

# Narrow found files
while [ $(echo "$FOUND_FILES" | wc -l) -gt 1 ]; do
    echo -e "${UNDL}${BOLD}Multiple files found:${NC}"
    echo "$FOUND_FILES"

    echo -e "${BOLD}Narrow search:${NC}"
    read -r NARROW_QUERY

    FOUND_FILES=$(echo "$FOUND_FILES" | grep "$NARROW_QUERY" || true)

    # Check if narrow was valid
    if [ -z "$FOUND_FILES" ]; then
        echo -e "${RED}Error${NC}: No files matched '$BF_QUERY'"
        FOUND_FILES=$(find bf -type f -name "*$BF_QUERY*")
    fi
done

BF_FILE=$FOUND_FILES

# Confirm compile
while true; do
    echo -e -n "${BOLD}Compile: $BF_FILE [Y/n]: ${NC}"
    read -r CONFIRM

    if [[ -z "$CONFIRM" || "$CONFIRM" == "y" || "$CONFIRM" == "yes" ]]; then
        break;
    elif [[ "$CONFIRM" == "n" || "$CONFIRM" == "no" ]]; then
        echo -e "Aborted."
        exit 0
    fi
done

# Start compiling the file
COMPILER_SRC="."
COMPILER_EXE="$COMPILER_SRC/target/release/brain-needle.exe"

echo -e "Compiling: ${GREEN}$BF_FILE${NC}"

./scripts/profile_verbose.sh "$COMPILER_EXE" "$BF_FILE"