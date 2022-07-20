up: docker compose up -d

down: docker compose down --remove-orphans

build: docker compose build

b-s SERVICE: docker compose build {{ SERVICE }}
