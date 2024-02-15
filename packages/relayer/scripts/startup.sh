#!/bin/bash

cd ../

if cd prover; then
    nohup modal serve modal_server.py &

    if [ $? -ne 0 ]; then
        echo "Error: Failed to start the modal_server"
        exit 1
    fi
else
    echo "Error: Directory ../prover/ does not exist"
    exit 1
fi

cd ../

if cd relayer; then
    if [ "$SETUP" = "true" ]; then
        cargo run --release -- setup

        if [ $? -ne 0 ]; then
            echo "Error: Failed to run cargo run --release -- setup"
            exit 1
        fi
    fi

    cargo run --release >> output.log

    if [ $? -ne 0 ]; then
        echo "Error: Failed to run cargo run --release >> output.log"
        exit 1
    fi
else
    echo "Error: Directory ../relayer/ does not exist"
    exit 1
fi
