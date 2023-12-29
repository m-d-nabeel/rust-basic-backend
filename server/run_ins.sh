#!/bin/bash
clear
docker compose up -d
export DATABASE_URL=postgres://dis-name-02:dis-password-02@localhost/postgres
cargo sqlx prepare
cargo run