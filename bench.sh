#!/bin/bash

# Function to check if a port is free
is_port_free() {
    local port=$1
    if lsof -i :$port > /dev/null; then
        return 1 # port is in use
    else
        return 0 # port is free
    fi
}

# Define an associative array for names and corresponding ports
declare -A names_ports=(
    ["rust_axum_tokio-postgres_bb8"]=3000
    ["rust_axum_tokio-postgres"]=3001
    ["rust_axum_tokio-postgres_prefork"]=3002
    ["rust_axum_tokio-postgres_tech-empower"]=3003
)

# Iterate over each name and port
for name in "${!names_ports[@]}"; do
    port=${names_ports[$name]}

    echo "Checking $name on port $port"

    if is_port_free $port; then
        echo "Starting $name on port $port"

        # Start the app in background mode on the specified port
        PORT=$port ./bin/"$name" &

        # Store the pid
        pid=$!

        # Run the benchmark on the specified port
        rewrk -c 50 -t 2 -d 30s -h http://localhost:$port

        # Kill the app 
        kill -9 "$pid"
    else
        echo "Port $port is in use, skipping $name"
    fi
done

echo -e "\n\n\n === All processes completed. === \n\n\n"
