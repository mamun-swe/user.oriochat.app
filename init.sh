#!/bin/bash

# Exit script on any error
set -e

# Start the MySQL container using docker-compose
echo "Starting Docker Compose..."
docker-compose -f docker-compose.db.yml up -d

# Define the MySQL container service name
MYSQL_SERVICE_NAME="rust_mysql_db"
MYSQL_CONTAINER=$(docker-compose -f docker-compose.db.yml ps -q "$MYSQL_SERVICE_NAME")
MYSQL_USER="root"
MYSQL_PASSWORD="root@root"

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
DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);
INSERT INTO users (username, name, email, password) VALUES ('example', 'example', 'example@gmail.com', '--');
"

# Execute SQL commands inside the MySQL container
docker exec -i "$MYSQL_CONTAINER" mysql -u"$MYSQL_USER" -p"$MYSQL_PASSWORD" -e "$SQL_COMMANDS"

echo "Database populated successfully!"
