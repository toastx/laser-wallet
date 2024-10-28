DB_DOCKER_CONTAINER=postgres-instance

install:

	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add cargo-watch

	cargo install sqlx-cli

build:
	cargo build

run:
	cargo watch -s "shuttle run"

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres


create_postgres_database:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root actix

start_docker_db:
	docker start ${DB_DOCKER_CONTAINER}

create_migrations:
	sqlx migrate add -r init

migrate_up:
	sqlx migrate run

migrate_down:
	sqlx migrate revert



