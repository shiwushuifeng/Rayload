#!/bin/sh
# @name: Rayload Test Script
# @version: 1.0.0
# @description: Safe script plugin for beta7 import testing.
# @permission: network
# @domain: example.com
#
# Rayload script plugins are long-running processes speaking newline-delimited
# JSON: one request object per line on stdin, one response object per line on
# stdout. The host injects `version` into every request, so echoing the line back
# answers both the handshake and the echo probe. Exiting after one line makes the
# host report "plugin closed stdout without answering".
while IFS= read -r line; do
    printf '%s\n' "$line"
done
