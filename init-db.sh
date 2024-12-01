#!/bin/bash

# Exit script on any error
set -e

# Start the MySQL container using docker-compose
echo "Starting Docker Compose For Database..."
docker-compose -f docker-compose.db.yml up -d

# Define the MySQL container service name
MYSQL_SERVICE_NAME="rust_db"
MYSQL_CONTAINER=$(docker-compose -f docker-compose.db.yml ps -q "$MYSQL_SERVICE_NAME")
MYSQL_USER="root"
MYSQL_PASSWORD="rootx"

# Wait for MySQL container to be ready
echo "Waiting for MySQL to be ready..."
until docker exec "$MYSQL_CONTAINER" mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "SELECT 1" &>/dev/null; do
    echo "Waiting for MySQL to initialize..."
    sleep 2
done
echo "MySQL is ready!"

# SQL commands to initialize the database and populate it
echo "Inserting values into the database..."

SQL_COMMANDS="
CREATE DATABASE IF NOT EXISTS oriochat_user_db;
USE oriochat_user_db;
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);
INSERT INTO users (username, name, email, password)
SELECT 'example', 'example', 'example@gmail.com', '--'
WHERE NOT EXISTS (
    SELECT 1 FROM users WHERE email = 'example@gmail.com'
);
"

# Execute SQL commands inside the MySQL container
docker exec -i "$MYSQL_CONTAINER" mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "$SQL_COMMANDS"

echo "Database populated successfully!"

# Retrieve data from the users table and display it
echo "Retrieving data from the users table..."

SELECT_QUERY="SELECT id, username, name, email FROM oriochat_user_db.users;"

# Execute SELECT query inside the MySQL container and capture the output
USER_DATA=$(docker exec -i "$MYSQL_CONTAINER" mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "$SELECT_QUERY")

# Echo the retrieved user data
echo "User Data Retrieved:"
echo "$USER_DATA"

echo "Data retrieval complete!"
