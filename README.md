# User profile microservice (RUST, Actix-web, MySQL, gRPC)

<!-- System requirements -->

## System requirements

- Docker
- docker-compose
- git

<!-- Clone repository -->

## Clone Repository

```
git clone https://github.com/mamun-swe/user.oriochat.app
```

<!-- Set permissions -->

## Setup permissions

```
chmod +x ./init-db.sh
chmod +x ./run.sh
chmod +x ./exit.sh
```

<!-- Database initialization -->

### First step: Database initialization

#### Why ?

When you run ```./init-db.sh```, it will execute the ```docker-compose.db.yml``` file to start and set up a MySQL database container. Once the database is set up, the script will create a database, connect to it via shell commands, create a table, and insert some sample data. After that, it will retrieve the inserted data and display it in the terminal to confirm that the database is working correctly.

To ensure everything runs smoothly, please follow the instructions provided below.

#### Execute the commands to run and populate the database & sample data


```
./init-db.sh
```

<!-- Run the application -->

### Second step: Build & Run the application
#### Why ?
When you run ```./run.sh```, the script will first build the application using ```docker-compose build```. After the build is complete, it will execute ```docker-compose up -d``` to start the containers, register them on the database network, and connect the application to the database. The application will then run on the specified PORT.

To ensure everything runs smoothly, please follow the instructions below.

```
./run.sh
```

<!-- Browser application -->
### Third step: Rust microservice & gRPC
```
Rust PORT: http://127.0.0.1:5000/
gRPC PORT: http://[::1]:50051
```

<!-- REST API Endpoints -->
### Available REST API Endpoints
  - Authentication
      - [POST] (Login) http://127.0.0.1:5000/login
      - [POST] (Register) http://127.0.0.1:5000/register
  - Profile
      - [GET] (Profile information) http://127.0.0.1:5000/profile
      - [PUT] (Update profile information) http://127.0.0.1:5000/profile

<!-- Stop database and all services -->

### Exit from application & database

```
./exit.sh
```
