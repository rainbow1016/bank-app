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

nextest: 
    cargo nextest run -v

fmt:
    cargo ftm --all -- --check
