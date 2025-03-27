# Batch Explorer

## How to Run 
The easiest way to get everything up is to run 
- `docker compose up frontend -d`

## For development

Make sure you have the following installed:

- Node.js (≥ v18) – Install via nvm
- Rust (≥ 1.74) – https://rustup.rs
- PostgreSQL (via Homebrew or system package manager)
```
brew install postgresql@15
brew services start postgresql@15
```

Create the database with
```
$ psql postgres

-- Inside psql shell:
CREATE USER postgres WITH PASSWORD 'password';
ALTER USER postgres WITH SUPERUSER;
CREATE DATABASE batch_explorer_db OWNER postgres;
\q
```

- SQLx CLI (for running migrations):
```
cargo install sqlx-cli --no-default-features --features postgres
```

- Apply migrations with
```
cd backend

export DATABASE_URL=postgres://postgres:password@localhost:5432/batch_explorer_db

cargo run --bin migration
```

- Run the backend with 

```
export APP_DATABASE_URL=postgres://postgres:password@localhost:5432/batch_explorer_db
export STRATA_FULLNODE=https://rpc.testnet-staging.stratabtc.org
export APP_FETCH_INTERVAL=5

cargo run --bin batch-explorer
```

- run the frontend with 
```
cd frontend

npm install
# Set environment variables
export VITE_API_BASE_URL=http://localhost:3000
export VITE_BLOCKSCOUT_BASE_URL=https://explorer.testnet-staging.stratabtc.org/
export VITE_MEMPOOL_BASE_URL=https://bitcoin.testnet-staging.stratabtc.org/
export VITE_REFRESH_INTERVAL=10000

npm run dev -- --host
```











