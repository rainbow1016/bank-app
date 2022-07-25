up: 
    docker compose up -d

down: 
    docker compose down --remove-orphans

build: 
    docker compose build

b-s SERVICE: 
    docker compose build {{ SERVICE }}

lint: 
    cargo clippy -- -D clippy::all

test: 
    cargo nextest run -v

fmt:
    cargo ftm --all -- --check

pre-commit: lint fmt

pre-push: nextest
