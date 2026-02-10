# huihui-server overview

## What this is
A small Rust HTTP API built with the Faithea framework. It exposes a health-style root route and an auth login endpoint that queries a Postgres database for a user.

## Tech stack
- Rust 2024 edition
- Faithea web framework
- Tokio (current-thread runtime)
- SQLx (Postgres, chrono, rustls TLS)
- Serde + Serde JSON
- env_logger

## Project layout
- `src/main.rs`: Server bootstrap, route mounts, and runtime setup.
- `src/lib.rs`: Global DB pool (`OnceLock`) and initialization helper.
- `src/handlers/`: HTTP handlers (currently auth only).
- `src/service/`: Service layer (auth logic + error mapping).
- `src/source/`: Data access layer (SQLx queries).
- `src/model/`: Request/response types and API response wrapper.
- `db.sql`: DDL for the `_user` table.

## Routes
- `GET /`: Returns the string `hello`.
- `POST /auth/login`: Accepts JSON `{ "username": "...", "password": "..." }` and returns an `ApiResponse<CurrentUser>`.

## Data model
- `_user` table: `id`, `email`, `username`, `password`, `created_at`, `updated_at`.
- `CurrentUser` response: `id`, `email`, `name` (mapped from `username`).

## Request/response shape
- Successful responses are wrapped in:
  - `ApiResponse { code: 200, message: "ok", data: <T> }`
- Errors are wrapped in:
  - `ApiResponse { code: 500, message: "<debug>" }`

## Startup flow
1. `env_logger::init()`
2. `init_db()` creates a global SQLx pool.
3. `HttpServer::builder()` mounts routes and binds `0.0.0.0`.

## Notes / risks
- The database URL is hard-coded in `src/lib.rs`. Consider moving to environment variables for deploys and local dev.
- The login flow currently fetches by username only and does not validate the password.

## How to run
- `cargo run`

## Next logical extensions
- Add password verification + hashing.
- Return proper HTTP status codes and structured error types.
- Add migrations tooling for `db.sql` (e.g., sqlx migrate).
