# Batch Explorer

## How to Run

The easiest way to get everything up is to run
`docker compose up frontend -d`

## For development

1. Make sure you have the following installed:

* Node.js (≥ v18) – Install via nvm.
* Rust (≥ 1.74) – Install using [the Rust installer](https://rustup.rs).
* PostgreSQL (via Homebrew or system package manager)

```sh
brew install postgresql@15
brew services start postgresql@15
```

* SQLx CLI (for running migrations):

```sh
cargo install sqlx-cli --no-default-features --features postgres
```

2. Create the database with

```sh
$ psql postgres << EOF
CREATE USER postgres WITH PASSWORD 'password';
ALTER USER postgres WITH SUPERUSER;
CREATE DATABASE batch_explorer_db OWNER postgres;
EOF
```

3. Apply migrations with

```sh
cd backend
export DATABASE_URL=postgres://postgres:password@localhost:5432/batch_explorer_db
cargo run --bin migration
```

4. Run the backend with

```sh
export APP_DATABASE_URL=postgres://postgres:password@localhost:5432/batch_explorer_db
export STRATA_FULLNODE=https://rpc.testnet-staging.stratabtc.org
export APP_FETCH_INTERVAL=5

cargo run --bin batch-explorer
```

5. Run the frontend with

```sh
cd frontend
npm install
npm run dev -- --host
```
