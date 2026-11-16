#!/bin/sh
# @name: Rayload Test Script
# @version: 1.0.0
# @description: Safe script plugin for beta7 import testing.
# @permission: network
# @domain: example.com
#
# Rayload plugin protocol: the host keeps this process alive and speaks
# newline-delimited JSON — one request object per line on stdin, one response
# object per line on stdout. The first request is a handshake that must answer
# with "version": 1. Anything that prints once and exits is treated as closed.
while IFS= read -r line; do
  case "$line" in
    *'"method": "download"'* | *'"method":"download"'*)
      printf '%s\n' '{"version":1,"method":"download","downloadUrl":"https://example.com/rayload-test.bin","filename":"rayload-test.bin"}'
      ;;
    *)
      # Echo handshake and echo probes back unchanged; the request already
      # carries "version": 1 by the time it reaches us.
      printf '%s\n' "$line"
      ;;
  esac
done
