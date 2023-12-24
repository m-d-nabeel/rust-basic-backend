#!/bin/bash
clear
docker start dis-postgres-01
export DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/postgres
cargo sqlx prepare
cargo run