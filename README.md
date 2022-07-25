# Bank Application

This application is built to demostrate a microservice architecture

## Design

- Frontend
  - Todo
- Backend
  - Rust
  - tide, serde, (tonic, prost, tokio for creating grpc client)
- Microservices
  - Rust
  - tokio, tonic, prost, sqlx
- Database
  - postgresql

## Remaining work

- [ ] Frontend
  - [ ] Determine frontend tooling
- [ ] Backend
  - [x] Determine backend framework
  - [ ] Design api routes for frontend
  - [ ] Establish connections to microservices
  - [x] Dockerized
- [ ] Database
  - [ ] Data model
  - [x] Figure out how to seed data
  - [x] Dockerized
  - [x] Others services can connect
- [ ] Users service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Checkings service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Savings service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Credits service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Cards service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Transactions service
  - [ ] Protobufs
  - [ ] Server
  - [ ] Implement functions
- [ ] Other
  - [ ] Build and run all services from docker compose
  - [ ] Unit tests / integration tests
  - [ ] Logging
  - [ ] Full architecture diagram
  - [ ] Results and learnings
  - [ ] Create scaffolding for creating other grpc services
  - [x] Networking for multiple containers to communicate
  - [ ] Implement CI
  - [ ] Learn/setup git hooks

## How to build and run

To do

## Requirements

docker, docker compose, rust toolchain, just
