#!/bin/sh

# Wait until MySQL is ready
echo "Waiting for MySQL to be ready..."
while ! mysqladmin ping -h oriochatusermysqldb --silent; do
    sleep 2
done
echo "MySQL is up and running."

# Run the SQL commands to initialize the database (only if it's not initialized yet)
echo "Initializing database if necessary..."
mysql -h oriochatusermysqldb -u root -p${MYSQL_PASSWORD} <<EOF
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
INSERT INTO users (username, name, email, password) 
SELECT 'example', 'example', 'example@gmail.com', '--'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE username='example');
EOF

# Check if the data was inserted successfully
echo "Checking if data was inserted..."
if ! mysql -h oriochatusermysqldb -u root -p${MYSQL_PASSWORD} -e "SELECT * FROM oriochat_user_db.users WHERE username='example'" | grep -q 'example'; then
    echo "Data insertion failed, exiting..."
    exit 1
fi
echo "Data inserted successfully."

# Start the Rust application
echo "Starting Rust application..."
exec cargo run
