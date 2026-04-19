# User Calorie Goal API

## Base URL
```
http://localhost:8080
```

## Authentication
All endpoints require a Bearer token in the `Authorization` header:
```
Authorization: Bearer <token>
```

---

## GET /user/calorie-goal/

Get the user's current active daily calorie goal.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 1,
    "user_id": 1,
    "daily_calorie_goal": 2000.0,
    "effective_from": "2026-04-01",
    "effective_to": null
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Goal record ID |
| `user_id` | integer | User ID |
| `daily_calorie_goal` | float | Daily calorie target |
| `effective_from` | string | Start date (YYYY-MM-DD) |
| `effective_to` | string | End date (YYYY-MM-DD), null if currently active |

---

## POST /user/calorie-goal/

Create or update a daily calorie goal.

- If `effective_from` is **today**: update the current goal directly
- If `effective_from` is **future**: close current goal (set `effective_to` to one day before new goal's start), then create new goal

**Request Body:**
```json
{
  "daily_calorie_goal": 2200.0,
  "effective_from": "2026-05-01"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `daily_calorie_goal` | float | Yes | Daily calorie target |
| `effective_from` | string | Yes | Start date (YYYY-MM-DD) |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 2,
    "user_id": 1,
    "daily_calorie_goal": 2200.0,
    "effective_from": "2026-05-01",
    "effective_to": null
  }
}
```
