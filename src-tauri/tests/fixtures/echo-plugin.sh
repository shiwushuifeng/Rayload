#!/bin/sh
# Rayload test plugin: echoes each JSON request line back unchanged.
while IFS= read -r line; do
    printf '%s\n' "$line"
done
