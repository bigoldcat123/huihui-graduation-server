# Project Conventions

These conventions are derived from this codebase and generalized so they can be reused across backend projects.

## 1. Architecture and Folder Layout

- Use clear layers with single responsibilities:
- `handlers/`: HTTP transport only (parse request, call service, return response).
- `service/`: business logic, authorization rules, orchestration.
- `source/` (or `repository/`): database and persistence queries.
- `model/`: input/output DTOs and raw persistence models.
- Keep bootstrapping in one place (`main` for runtime/wiring, `lib` for shared globals/utilities).
- Do not call DB code directly from handlers.

## 2. API Contract

- Use one consistent response envelope for all endpoints:
- success: `{ code, message, data }`
- failure: `{ code, message, data: null }`
- Keep request DTOs and response DTOs separate from raw DB models.
- Standardize pagination params (`page`, `page_size`) and clamp limits centrally.
- Keep route naming consistent (resource-based paths and stable verbs).

## 3. Error Handling

- Define a central service error enum and map lower-level errors into it.
- Convert errors at layer boundaries:
- source -> service error
- service result -> API response
- Avoid leaking internal debug output to public clients in production.
- Prefer explicit permission/auth errors over generic "not found" behavior.

## 4. Auth and Security

- Extract current user/root user using framework request extractors/middleware.
- Keep auth checks in service layer (or guard layer), not inside SQL queries.
- Never hardcode secrets or DB URLs in code.
- Read secrets from environment variables with safe startup validation.
- Store passwords hashed (argon2/bcrypt), never plaintext.
- Validate login credentials (username/email + password verification), not user existence alone.

## 5. Data Access and SQL

- Keep SQL close to source/repository functions with typed result structs.
- Use parameterized queries only (`$1`, `$2`, ...), never string interpolation.
- Use `fetch_optional` when absence is valid, `fetch_one` when required.
- Keep SQL deterministic (`ORDER BY`) for list endpoints.
- Encapsulate multi-step write flows in transactions when consistency matters.
- For schema evolution, use migration tooling instead of ad-hoc SQL edits.

## 6. Configuration and Initialization

- Initialize logging, config, and DB before starting the server.
- Fail fast on missing required configuration.
- Keep constants/config in one module and document defaults.
- Avoid global mutable state unless initialization is guaranteed once.

## 7. Logging and Observability

- Use structured logs for request start/end, key business actions, and failures.
- Never log credentials, tokens, or sensitive payload fields.
- Include request identifiers (or user/context ids when safe) for traceability.

## 8. Code Style

- Keep handlers thin and predictable.
- Keep functions focused; extract helpers for reusable transforms/calculations.
- Prefer explicit types at public boundaries; infer internally where clear.
- Use consistent naming conventions for files, modules, and route handlers.

## 9. Testing Standards

- Add tests at three levels:
- unit tests for service logic and helpers
- integration tests for source layer against a test DB
- API tests for handler routing and response contracts
- Cover auth edge cases, pagination bounds, and empty-result scenarios.
- Add regression tests for every bug fix.

## 10. Delivery Checklist (Before Merge)

- `cargo fmt` and `cargo clippy` clean.
- tests pass locally/CI.
- no hardcoded secrets or environment-specific URLs.
- new endpoints documented (request, response, auth requirements).
- DB changes have migrations and rollback considerations.
- error messages are safe for production clients.
