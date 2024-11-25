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
