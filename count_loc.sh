#!/usr/bin/env bash

find src -type f | xargs -I '{}' -n1 wc -l '{}' | cut -d\  -f1 | awk '{s+=$1} END {print s}'
