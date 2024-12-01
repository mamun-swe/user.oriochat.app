#!/bin/sh

# Wait until MySQL is ready
echo "Waiting for MySQL to be ready..."
until mysqladmin ping -h ${MYSQL_DATABASE} --silent; do
    echo "Waiting for MySQL..."
    sleep 2
done
echo "MySQL is up and running."

# Initialize database if necessary (keep this part as is)
echo "Initializing database if necessary..."
mysql -h ${MYSQL_DATABASE} -u root -p${MYSQL_PASSWORD} <<EOF
CREATE DATABASE IF NOT EXISTS oriochat-user-db;
USE oriochat-user-db;
DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);
INSERT INTO users (username, name, email, password) 
SELECT 'example', 'example', 'example@gmail.com', '--'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE username='example');
EOF

# Check if data was inserted successfully
echo "Checking if data was inserted..."
if ! mysql -h ${MYSQL_DATABASE} -u root -p${MYSQL_PASSWORD} -e "SELECT * FROM ${MYSQL_DATABASE}.users WHERE username='example'" | grep -q 'example'; then
    echo "Data insertion failed, exiting..."
    exit 1
fi
echo "Data inserted successfully."

# Start the Rust application
echo "Starting Rust application..."
