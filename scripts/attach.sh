#!/bin/bash

directory="/usr/local/bin/"

# Check if the directory exists
if [ ! -d "$directory" ]; then
    echo "Directory '$directory' does not exist. Creating it..."
    sudo mkdir -p "$directory"
fi

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Error: Two arguments are required - script file path and script name"
    echo "Usage: ./attach_script_to_bin.sh <script_file_path> <script_name>"
    exit 1
fi

script_path="$1"

script_name="$2"

# Check if the specified script file exists
if [ ! -f "$script_path" ]; then
    echo "Script '$script_path' does not exist. Please provide the correct path to the script."
    exit 1
fi

# Create a symbolic link to the script in the bin directory
sudo ln -s "$script_path" "$directory$script_name"

# Check if the link creation was successful
if [ "$?" -eq 0 ]; then
    echo "Script '$script_name' has been attached to '$directory' successfully."
else
    echo "Failed to attach the script '$script_name' to '$directory'."
fi
