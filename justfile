alias b:= build
alias d:= down

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
    cargo fmt --all -- --check

ci-install:
    cargo install cargo-nextest

pre-commit: lint fmt

pre-push: test
