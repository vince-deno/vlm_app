#!/bin/bash

# Function to display help message
function display_help() {
    echo "Usage: $0 [command]"
    echo
    echo "Commands:"
    echo "  build    Build the Go module"
    echo "  run      Run the Go module"
    echo "  help     Display this help message"
    echo
}

# Check if a command is provided
if [ -z "$1" ]; then
    echo "Error: No command provided."
    display_help
    exit 1
fi

# Execute the provided command
case "$1" in
    build)
        echo "Building the Go module..."
        go build
        ;;
    run)
        echo "Running the Go module..."
        go run .
        ;;
    help)
        display_help
        ;;
    *)
        echo "Error: Unknown command '$1'."
        display_help
        exit 1
        ;;
esac