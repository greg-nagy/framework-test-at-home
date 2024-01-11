#!/bin/bash

mkdir -p ./bin

# Define the array of names
names=(
    "rust_axum_tokio-postgres"
    "rust_axum_tokio-postgres_bb8"
    "rust_axum_tokio-postgres_prefork"
    "rust_axum_tokio-postgres_tech-empower"
)

# Iterate over each name
for name in "${names[@]}"; do
    # Change to the directory
    cd "$name" || exit

    # Build the project
    RUSTFLAGS="-C target-cpu=native" cargo build --release

    # Copy the binary to the parent directory
    cp "./target/release/$name" ../bin

    # Go back to the original directory
    cd ..

    echo "Processed $name"
done

echo "All processes completed."
