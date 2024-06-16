#!/bin/bash
clear
docker compose up -d

# Function to check if PostgreSQL is ready
function wait_for_postgres() {
  echo "Waiting for PostgreSQL to be ready..."
  until docker exec dis-postgres-02 pg_isready -U dis-name-02 -d postgres
  do
    sleep 2
  done
  echo "PostgreSQL is ready."
}

# Wait for PostgreSQL to be ready before proceeding
wait_for_postgres

export DATABASE_URL=postgres://dis-name-02:dis-password-02@localhost:5432/postgres

echo "Checking for existing tables..."
if ! docker exec dis-postgres-02 psql -U dis-name-02 -d postgres -c "\dt" | grep -q "member"; then
  echo "First run detected. Initializing database schema..."
  docker exec -i dis-postgres-02 psql -U dis-name-02 -d postgres < db_init_query/query_init.sql
else
  echo "Database schema already initialized. Skipping initial setup."
fi

cargo sqlx prepare

# Run the Rust application
echo "Starting the Rust application..."
cargo run
