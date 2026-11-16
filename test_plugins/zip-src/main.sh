#!/bin/sh
# Rayload script plugin.
#
# The host keeps this process alive and speaks newline-delimited JSON: one
# request object per line on stdin, one response object per line on stdout.
# Every request already carries "version": 1, so echoing it back answers the
# handshake and the echo probe. A one-shot script that prints and exits makes the
# host fail with "plugin closed stdout without answering".
while IFS= read -r line; do
    printf '%s\n' "$line"
done
