#!/bin/sh

source clear-env.sh
cargo run

if [ $? -eq 0 ]; then
    echo "Expected const_env_tests run to fail because env vars are not set, but it did not"; 
    exit 1;
fi

source set-env.sh
cargo run

if [ $? -ne 0 ]; then
    echo "Expected const_env_tests run to succeed because env vars are set, but it failed"; 
    exit 1;
fi

source clear-env.sh
echo "run-tracking-test.sh completed successfully"