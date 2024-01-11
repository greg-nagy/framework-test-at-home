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
    ["openresty"]=2222
    ["rust_axum_tokio-postgres_bb8"]=3000
    ["rust_axum_tokio-postgres_arc"]=3001
    ["rust_axum_tokio-postgres_tech-empower"]=3003
    ["rust_axum_tokio-postgres_prefork"]=3002
)

# 50 concurency, 2 threads, 30s duration, 5 rounds
test_command_with_base_host="rewrk -c 50 -t 2 -d 30s -r 5 -h http://localhost"

# Check if DB_URL is set in the current shell. Rust examples won't work without it
if [ -z "$DB_URL" ]; then
    echo "Error: DB_URL is not set."
    exit 1
fi

for name in "${!names_ports[@]}"; do
    port=${names_ports[$name]}

    if is_port_free $port; then
        echo -e "\n === $name on $port === \n"

        echo -e "--- starting $name on $port --- \n"

        # let's start the app to test
        if [ "$name" != "openresty" ]; then 
            ./bin/"$name" &
        else 
            /usr/local/openresty/bin/openresty
        fi

        sleep 1

        echo -e "--- plain text ---\n"

        $test_command_with_base_host:$port

        if [ "$name" != "openresty" ]; then
            echo -e "--- single query ---\n"

            $test_command_with_base_host:$port/count
        else
            echo -e "--- single query: lua module ---\n"

            $test_command_with_base_host:$port/count

            # restart openresty
            # If the tests are run on both endpoints the 2nd endpoint becomes really slow
            /usr/local/openresty/bin/openresty -s stop
            /usr/local/openresty/bin/openresty

            echo -e "--- single query: postgres module ---\n"

            $test_command_with_base_host:$port/count2
        fi    

        # cleanup
        if [ "$name" != "openresty" ]; then
            killall rust_axum_tokio-postgres
        else 
            /usr/local/openresty/bin/openresty -s stop
        fi
    else
        echo "Port $port is in use, skipping $name"
    fi
done