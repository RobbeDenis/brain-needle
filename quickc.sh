#!/bin/bash

BF_QUERY=$1

# Colors
export NC="\e[0m"
export RED="\e[31m"
export GREEN="\e[32m"
export UNDL="\e[4;37m"
export BOLD="\e[1;97m"

# Clean output/quickc/
if [ "$BF_QUERY" == "--clean" ]; then
    rm output/quickc/*.asm
    rm output/quickc/*.o
    rm output/quickc/*.exe
    exit 0
fi

# Query
if [ -z "$BF_QUERY" ]; then
    # Find matching files for .b*
    FOUND_FILES=$(find bf -type f -name "*.b*")
else
    # Find matching files for query
    FOUND_FILES=$(find bf -type f -name "*$BF_QUERY*")
fi

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
        echo -e "${RED}Error${NC}: No files matched '$NARROW_QUERY'"
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
        echo -e "Aborted"
        exit 0
    fi
done

COMPILER_SRC="."
COMPILER_EXE="$COMPILER_SRC/target/release/brain-needle.exe"

# Do interpreted if no extra arguments where specified else compile
if [[ -z "${@:2}" ]]; then
    echo -e "\nInterpreting with brain-needle ${GREEN}$BF_FILE${NC}"
    echo -e "--------"
    ./scripts/profile_verbose.sh "$COMPILER_EXE" "$BF_FILE"
else
    OUTPUT_FILE="output/quickc/${BF_FILE##*/}"
    OUTPUT_FILE="${OUTPUT_FILE%.*}"
    echo -e "\nCompiling with brain-needle ${GREEN}$BF_FILE${NC}"
    echo -e "--------"
    ./scripts/profile_verbose.sh "$COMPILER_EXE" "$BF_FILE" -d file -o "$OUTPUT_FILE.asm" "${@:2}"

    # Compile with nasm
    echo -e "Compiling with nasm"
    nasm -f elf64 "$OUTPUT_FILE.asm" -o "$OUTPUT_FILE.o"
    echo -e "Linking..."
    ld "$OUTPUT_FILE.o" -o "$OUTPUT_FILE.exe"
    echo -e "Running ${GREEN}$OUTPUT_FILE.exe${NC}"
    echo -e "--------"
    ./scripts/profile_verbose.sh "./$OUTPUT_FILE.exe"
fi