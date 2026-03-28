# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
# Run the server
cargo run

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Build release
cargo build --release
```

## Database Setup

- PostgreSQL database with connection string: `postgres://admin:root@localhost:5432/huihui`
- Schema defined in `db.sql` - run this against your database to create tables
- Uses SQLx with compile-time checked queries (no migrations tooling yet)

## Architecture Overview

This is a Rust HTTP API server using the **Faithea** framework with a strict layered architecture:

```
src/
├── main.rs          # Server bootstrap, route mounting
├── lib.rs           # Global DB pool (OnceLock), ROOT_USER_ID constant
├── handlers/        # HTTP layer only - parse request, call service, return response
├── service/         # Business logic, auth checks, orchestration between sources
├── source/          # Data access layer - SQLx queries, raw DB models
└── model/           # Request/response DTOs, ApiResponse wrapper
```

### Layer Responsibilities

1. **handlers/**: Parse incoming requests, extract auth tokens, call service functions, wrap results in `ApiResponse`. Keep thin - no business logic.

2. **service/**: Business logic lives here. Orchestrates calls to multiple source functions, handles auth/permissions, transforms data for responses.

3. **source/**: Database queries only. Returns typed structs matching DB rows. Each table typically has its own module.

4. **model/**:
   - `input.rs`: Request DTOs (e.g., `CreateFoodInput`)
   - `output.rs`: Response DTOs (e.g., `FoodWithTags`)
   - `raw.rs`: Database row structs (e.g., `FoodRow`)
   - `mod.rs`: `ApiResponse<T>` wrapper with `code`, `message`, `data` fields

### Request Flow

```
HTTP Request → Handler → Service → Source → Database
                ↓
            ApiResponse::from(Result) → JSON response
```

### Authentication

- JWT-based auth using `jsonwebtoken` crate
- `CurrentUserId` extractor validates Bearer token from `Authorization` header
- `CurrentRootUserId` additionally checks if user ID equals `ROOT_USER_ID` (currently hardcoded to 6)
- JWT secret from `JWT_SECRET` env var, defaults to "dev-secret-change-me"

### API Response Format

All endpoints return `ApiResponse<T>`:
```json
// Success
{ "code": 200, "message": "ok", "data": {...} }

// Error
{ "code": 500, "message": "error details", "data": null }
```

### Route Structure

Routes are mounted at prefixes in `main.rs`:
- `/auth/*` - login, register, me
- `/food/*` - recommendations, likes, CRUD
- `/topic/*` - forum posts, comments
- `/tag/*` - food tags
- `/restaurant/*` - restaurants CRUD
- `/suggestion/*` - user suggestions workflow
- `/static/*` - static file serving

### Key Domain Concepts

- **Foods** have tags, belong to restaurants, can be liked by users
- **Operations** track user reactions (like/skip/dislike) with weights
- **Topics** are forum posts with likes and nested comments
- **Suggestions** have a workflow status (PENDING → APPROVED/REJECTED → PROCESSING → FINISHED)

### Error Handling

`ServiceError` enum wraps underlying errors:
- `SqlError(sqlx::Error)`
- `JwtError(jsonwebtoken::errors::Error)`
- `JsonError(serde_json::Error)`
- `PermissionDenied(String)`

Services return `Result<T, ServiceError>`, handlers convert via `ApiResponse::from()`.

### Conventions

See `conventions.md` for detailed coding standards. Key points:
- Use parameterized queries (`$1`, `$2`) - never string interpolation
- Keep handlers thin, logic in service layer
- Separate request/response DTOs from DB models
- `ROOT_USER_ID` is the admin user (id=6)