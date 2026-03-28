# Code Review - huihui-server

## Overview

This is a Rust HTTP API server using the Faithea framework with a clean layered architecture (handlers → service → source). The server provides food recommendation, topic/forum, restaurant management, and suggestion workflow functionality.

---

## 1. Security Issues (High Priority)

### 1.1 Password Storage - Plaintext Passwords
**Location**: `src/source/user.rs:27-36`

```rust
pub async fn create_user(email: &str, username: &str, password: &str) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as(
        "INSERT INTO _user (email, username, password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(email)
    .bind(username)
    .bind(password)  // Password stored as plaintext!
```

**Problem**: Passwords are stored in plaintext. This is a critical security vulnerability.

**Recommendation**: Use password hashing (argon2, bcrypt, or scrypt):
```rust
use argon2::{hash_password, Algorithm, Params, PasswordHasher, SaltString};
use password_hash::rand_core::OsRng;

fn hash_password(password: &str) -> Result<String, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(argon2::Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}
```

### 1.2 No Password Verification in Login
**Location**: `src/service/auth.rs:84-89`

```rust
pub async fn login(auth: UsernamePasswordAuthentication) -> Result<AuthResult, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;
    // No password verification!
    let current_user: CurrentUser = user.into();
    let token = sign_token(current_user.id)?;
    Ok(AuthResult { token })
}
```

**Problem**: Login only checks if username exists, never verifies the password.

**Recommendation**: Add password verification:
```rust
pub async fn login(auth: UsernamePasswordAuthentication) -> Result<AuthResult, ServiceError> {
    let user = source::user::get_user_by_name(&auth.username).await?;
    verify_password(&auth.password, &user.password)?;
    // ... rest of the logic
}
```

### 1.3 Hardcoded Database Connection String
**Location**: `src/lib.rs:13`

```rust
pub async fn init_db() {
    DB.set(Pool::connect("postgres://admin:root@localhost:5432/huihui").await.expect("URL GG")).expect("msg");
}
```

**Problem**: Database credentials are hardcoded.

**Recommendation**: Use environment variables:
```rust
pub async fn init_db() {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    DB.set(Pool::connect(&db_url).await.expect("Failed to connect")).expect("DB already initialized");
}
```

### 1.4 Hardcoded Root User ID
**Location**: `src/lib.rs:10`

```rust
pub const ROOT_USER_ID: i32 = 6;
```

**Problem**: Root user is identified by hardcoded ID. This is fragile and not configurable.

**Recommendation**: Use a role-based system or configurable admin list:
```rust
pub fn is_root_user(user_id: i32) -> bool {
    env::var("ROOT_USER_IDS")
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .any(|id| id == user_id)
}
```

### 1.5 JWT Secret Default Value
**Location**: `src/service/auth.rs:60-62`

```rust
fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string())
}
```

**Problem**: Default JWT secret in production could lead to token forgery.

**Recommendation**: Fail fast in production:
```rust
fn jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set in production")
}
```

### 1.6 File Upload Security
**Location**: `src/handlers/upload.rs:28-45`

```rust
let ext = file.file_name.as_ref()
    .and_then(|name| Path::new(name).extension())
    // ... no validation of file type or size
```

**Problems**:
- No file type validation (could upload executables)
- No file size limit
- Predictable file names (timestamp-based)

**Recommendation**:
```rust
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "webp"];
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

// Validate extension
if !ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
    return ApiResponse::<Vec<String>>::err("Invalid file type".to_string()).json();
}
```

---

## 2. Error Handling Issues

### 2.1 Debug Error Messages Leaked to Clients
**Location**: `src/model/mod.rs:22-24`

```rust
Err(e) => {
    Self::err(format!("{:?}",e))  // Leaks internal debug info
}
```

**Problem**: Full debug error messages are returned to clients, potentially exposing sensitive implementation details.

**Recommendation**: Return generic error messages, log detailed errors:
```rust
Err(e) => {
    log::error!("Service error: {:?}", e);
    Self::err("An error occurred".to_string())
}
```

### 2.2 Inconsistent Error Types
**Location**: `src/service/error.rs`

**Problem**: `ServiceError` uses `PermissionDenied` for business logic errors like "suggestion already at final stage".

**Recommendation**: Add more specific error variants:
```rust
pub enum ServiceError {
    // ... existing variants
    NotFound(String),
    ValidationError(String),
    ConflictError(String),
}
```

### 2.3 Silent Error Ignoring
**Location**: `src/service/suggestion.rs:92-97`

```rust
let _ = source::todo_log::create_todo_log(
    ipt.suggestion_id,
    next,
    &format!("move suggestion {} from {} to {}", ipt.suggestion_id, current, next),
)
.await?;
```

**Problem**: Using `let _ = ... ?` pattern discards the result but still propagates errors, which is confusing.

**Recommendation**: Either handle the result or use `.ok()` to explicitly ignore:
```rust
if let Err(e) = source::todo_log::create_todo_log(...).await {
    log::warn!("Failed to create todo log: {:?}", e);
}
```

---

## 3. Code Quality Issues

### 3.1 Duplicate SQL Query Logic
**Location**: `src/source/topic.rs` - Multiple similar queries

The queries for `list_topics_by_page`, `list_comments_by_topic_id`, and `list_topics_by_user_id` share almost identical SELECT clauses and JOINs.

**Recommendation**: Extract common query parts using SQL CTEs or create a helper function:
```rust
const TOPIC_SELECT: &str = r#"
    SELECT
        t.id, t.user_id, t.title, t.content, t.images, t.create_at,
        u.username AS user_name, u.email AS user_email, u.profile AS user_profile,
        COALESCE(c.comment_count, 0)::BIGINT AS comment_count,
        COALESCE(l.like_count, 0)::BIGINT AS like_count,
        EXISTS(SELECT 1 FROM topic_like tl2 WHERE tl2.topic_id = t.id AND tl2.user_id = $1) AS liked
"#;
```

### 3.2 Unused Parameter
**Location**: `src/handlers/auth.rs:26`

```rust
async fn me(user_id:FromRequest<CurrentUserId>,_token:FromRequest<CurrentUserId>) {
```

**Problem**: `_token` is never used.

**Recommendation**: Remove the unused parameter.

### 3.3 Debug Print Statements
**Location**: `src/handlers/food.rs:25`

```rust
println!("{:?}",ipt);
```

**Problem**: Debug print left in production code.

**Recommendation**: Remove or replace with proper logging:
```rust
log::debug!("Recommendation reaction: {:?}", ipt);
```

### 3.4 Magic Numbers
**Location**: `src/service/topic.rs:7`, `src/service/food.rs:9-11`

```rust
const PAGE_SIZE: i64 = 10;
// ...
if tags.len() > 4 {
    tags.truncate(4);
}
```

**Recommendation**: Define constants with descriptive names:
```rust
const DEFAULT_PAGE_SIZE: i64 = 10;
const MAX_RECOMMENDATION_TAGS: usize = 4;
```

### 3.5 N+1 Query Problem
**Location**: `src/service/suggestion.rs:153-209`

```rust
for item in suggestions {
    let food = match item.food_id {
        Some(food_id) => {
            let food = source::food::get_food_by_id(food_id).await?;  // N+1
            let restaurant = source::restaurant::get_restaurant_by_id(food.restaurant_id).await?;  // N+1
            let tags = source::tag::list_food_tags(food.id).await?;  // N+1
```

**Problem**: For each suggestion, multiple database queries are made sequentially.

**Recommendation**: Batch load related data or use JOINs:
```rust
// Collect all food_ids first
let food_ids: Vec<i32> = suggestions.iter()
    .filter_map(|s| s.food_id)
    .collect();
let foods = source::food::list_food_in_ids(&food_ids).await?;
let foods_map: HashMap<i32, FoodRow> = foods.into_iter().map(|f| (f.id, f)).collect();
```

---

## 4. Performance Issues

### 4.1 Sequential Database Calls
**Location**: `src/service/food.rs:48-64`

```rust
pub async fn list_liked_foods(user_id: i32) -> Result<Vec<FoodWithRestaurant>, ServiceError> {
    let foods = source::food::list_user_liked_foods(user_id).await?;
    for food in foods {
        let restaurant = source::restaurant::get_restaurant_by_id(food.restaurant_id).await?;
```

**Problem**: Sequential queries for each food item.

**Recommendation**: Use a single JOIN query or batch fetch restaurants.

### 4.2 Random Ordering Without Index
**Location**: `src/source/food.rs:61-75`

```sql
ORDER BY RANDOM()
```

**Problem**: `ORDER BY RANDOM()` is slow for large tables as it requires scanning all rows.

**Recommendation**: For large tables, consider:
1. Using `TABLESAMPLE` for random sampling
2. Pre-computing random IDs
3. Adding a random column with index

### 4.3 Unbounded Page Size in Some Endpoints
**Location**: `src/service/topic.rs:9-12`

```rust
pub async fn list(page: Option<i64>, user_id: i32) -> Result<Vec<TopicListItem>, ServiceError> {
    let page = page.unwrap_or(1);
    let raw_topics = source::topic::list_topics_by_page(page, PAGE_SIZE, user_id).await?;
```

**Good**: PAGE_SIZE is constant.

**Problem**: Other endpoints don't clamp page_size properly:
```rust
// src/service/food.rs:66-68
let page_size = page_size.unwrap_or(10).max(1).min(100);  // Good
// But other places don't have this validation
```

**Recommendation**: Standardize pagination validation across all endpoints.

---

## 5. Database & SQL Issues

### 5.1 SELECT * Usage
**Location**: `src/source/user.rs:4, 12, 20`

```sql
SELECT * FROM _user WHERE id = $1
```

**Problem**: `SELECT *` can break when schema changes and returns unnecessary columns (like password hash).

**Recommendation**: Explicitly list columns:
```sql
SELECT id, email, username, profile, created_at, updated_at FROM _user WHERE id = $1
```

### 5.2 Missing Indexes
**Location**: `db.sql`

The schema might benefit from additional indexes:
- `operation.user_id` (frequently queried)
- `topic.create_at` (for ordering)
- `suggestion.status` (for filtering)

### 5.3 Inconsistent Date Formatting
**Location**: `db.sql:56`

```sql
create_at timestamp with time zone NOT NULL DEFAULT '2026-02-13 05:24:35.438949'::timestamp without time zone,
```

**Problem**: Default timestamp is a hardcoded value, not `CURRENT_TIMESTAMP`.

**Recommendation**: Fix the default:
```sql
create_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
```

---

## 6. Architecture Suggestions

### 6.1 Consider Using Transactions
**Location**: `src/service/food.rs:132-146`

```rust
pub async fn update_food(ipt: UpdateFoodInput) -> Result<FoodWithTags, ServiceError> {
    let food = source::food::update_food(...).await?;
    source::food::clear_food_tags(food.id).await?;
    for tag_id in ipt.tag_ids {
        source::food::add_food_tag(food.id, tag_id).await?;
    }
```

**Problem**: Multiple operations without transaction - partial failure could leave inconsistent state.

**Recommendation**: Wrap in a transaction:
```rust
pub async fn update_food(ipt: UpdateFoodInput) -> Result<FoodWithTags, ServiceError> {
    let mut tx = db().begin().await?;
    // ... operations using &mut *tx
    tx.commit().await?;
}
```

### 6.2 Add Request/Response Validation
**Location**: `src/model/input.rs`

**Problem**: No validation on input fields (empty strings, negative IDs, etc.)

**Recommendation**: Add validation in service layer or use a validation crate:
```rust
pub fn validate(&self) -> Result<(), ValidationError> {
    if self.username.is_empty() || self.username.len() > 50 {
        return Err(ValidationError::InvalidUsername);
    }
    // ...
}
```

### 6.3 Add Health Check Endpoint
**Recommendation**: Add a `/health` endpoint that checks database connectivity:
```rust
#[get("/health")]
async fn health() -> impl IntoResponse {
    match sqlx::query("SELECT 1").fetch_one(db()).await {
        Ok(_) => Json(json!({"status": "healthy"})),
        Err(_) => Json(json!({"status": "unhealthy"})),
    }
}
```

---

## 7. Missing Features

### 7.1 No Rate Limiting
Consider adding rate limiting for authentication endpoints to prevent brute force attacks.

### 7.2 No Request Logging Middleware
Add middleware to log all requests with timing information for observability.

### 7.3 No CORS Configuration
If the API is called from browsers, CORS configuration may be needed.

### 7.4 No API Versioning
Consider versioning the API (e.g., `/v1/food/recommendation`) for future compatibility.

---

## 8. Testing

### 8.1 No Tests Present
The codebase has no unit or integration tests.

**Recommendation**: Add tests following the conventions in `conventions.md`:
- Unit tests for service logic
- Integration tests for source layer
- API tests for handlers

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cal_user_profile() {
        // Test the recommendation algorithm
    }
}
```

---

## Summary of Priority Fixes

| Priority | Issue | Impact |
|----------|-------|--------|
| 🔴 Critical | Plaintext passwords | Security breach |
| 🔴 Critical | No password verification | Anyone can login |
| 🟠 High | Hardcoded DB credentials | Deployment risk |
| 🟠 High | Debug errors leaked | Information disclosure |
| 🟡 Medium | N+1 queries | Performance degradation |
| 🟡 Medium | File upload validation | Security risk |
| 🟢 Low | Magic numbers | Code maintainability |
| 🟢 Low | Missing tests | Quality assurance |