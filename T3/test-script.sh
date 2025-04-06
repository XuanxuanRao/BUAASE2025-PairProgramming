#!/bin/bash

cd /d/course/SE/BUAASE2025-PairProgramming/T3 || exit 1

my=0
other=0

for ((i=1; i<=100; i++))
do
    output=$(npm run submit-test 2>&1)
    result=$(echo "$output" | tail -n 1)

    if [[ "$result" =~ ^-?[0-9]+$ ]]; then
        if [[ "$result" -eq 1 ]]; then
            my=$((my + 1))
        elif [[ "$result" -eq -1 ]]; then
            other=$((other + 1))
        fi
    else
        echo "Run $i: Invalid result '$result' (not a number)"
    fi

    echo "Run $i: Result = $result, My = $my, Other = $other"
done

echo ""
echo "✅ Final Result Summary:"
echo "My: $my"
echo "Other: $other"

