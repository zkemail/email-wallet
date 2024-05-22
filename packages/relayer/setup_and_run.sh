#!/bin/bash

# Attempt to run the setup command
cargo run setup

# Check if the last command's exit code is not zero and matches the specific error
if [ $? -ne 0 ]; then
    echo "Setup failed or encountered a known error, proceeding to run the main application..."
    cargo run
fi

# Always run cargo run after attempting setup, regardless of the outcome
cargo run