#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <user@servername>"
    exit 1
fi

ssh -f -N -R 9000:localhost:9000 "$1"

echo "Reverse SSH tunnel started in background (PID: $!)"
