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
chmod +x ./init.sh
chmod +x ./down.sh
```

<!-- Database initialization -->

### Database initialization

#### Execute the commands to run and populate the database & sample data

```
./init.sh
```

<!-- Run the application -->

### Build & Run the application

```
cargo clean
cargo build
cargo run
```

<!-- Stop database and all services -->

### Stop database & others services

```
./down.sh
```
