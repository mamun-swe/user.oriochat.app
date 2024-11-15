#!/usr/bin/env bash

# Get the host and port from the command line arguments
host="$1"
port="$2"
shift 2
cmd="$@"

echo "Waiting for $host:$port to be available..."

# Continuously check if the MySQL service is available
until nc -z "$host" "$port"; do
  echo "Waiting for $host:$port..."
  sleep 3
done

# When MySQL is ready, show success message and execute the command
echo "$host:$port is available! Running the application now..."
exec $cmd
