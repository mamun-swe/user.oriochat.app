docker-compose down
docker-compose -f docker-compose.db.yml down
docker rm rust_app
docker rmi rust_app