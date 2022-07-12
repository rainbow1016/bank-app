FROM postgres:latest as db
WORKDIR /app
COPY ./database/scripts/init.sh /docker-entrypoint-initdb.d/
COPY ./database/scripts/seed.sql ./scripts/seed.sql
