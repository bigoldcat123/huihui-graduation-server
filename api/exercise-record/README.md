# Exercise Record API

## Base URL
```
http://localhost:8899
```

## Authentication
All endpoints require a Bearer token in the `Authorization` header:
```
Authorization: Bearer <token>
```

---

## GET /exercise-record/

Get the user's today's exercise records.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 1,
      "user_id": 1,
      "exercise_type_id": 1,
      "exercise_name_snapshot": "Running",
      "met_value_snapshot": 9.8,
      "duration_minutes": 30,
      "body_weight_kg": 70.0,
      "calories_burned": 343.0,
      "occurred_at": "2026-04-19 07:00:00",
      "created_at": "2026-04-19 07:30:00"
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Record ID |
| `user_id` | integer | User ID |
| `exercise_type_id` | integer | Exercise type ID |
| `exercise_name_snapshot` | string | Exercise name (snapshot) |
| `met_value_snapshot` | float | MET value at time of record |
| `duration_minutes` | integer | Duration in minutes |
| `body_weight_kg` | float | Body weight in kg |
| `calories_burned` | float | Calculated calories burned |
| `occurred_at` | string | When exercise occurred |
| `created_at` | string | Record creation time |

---

## GET /exercise-record/all

Get the user's all exercise records.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 2,
      "user_id": 1,
      "exercise_type_id": 2,
      "exercise_name_snapshot": "Walking",
      "met_value_snapshot": 3.8,
      "duration_minutes": 45,
      "body_weight_kg": 70.0,
      "calories_burned": 199.5,
      "occurred_at": "2026-04-18 18:00:00",
      "created_at": "2026-04-18 18:30:00"
    }
  ]
}
```

---

## POST /exercise-record/

Create a new exercise record. Calories are calculated automatically using the formula:

```
calories_burned = met_value * body_weight_kg * (duration_minutes / 60)
```

**Request Body:**
```json
{
  "exercise_type_id": 1,
  "duration_minutes": 30,
  "body_weight_kg": 70.0,
  "occurred_at": "2026-04-19 07:00:00"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `exercise_type_id` | integer | Yes | Exercise type ID |
| `duration_minutes` | integer | Yes | Exercise duration in minutes |
| `body_weight_kg` | float | Yes | Body weight in kg |
| `occurred_at` | string | Yes | When exercise occurred (YYYY-MM-DD HH:MM:SS) |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 3,
    "user_id": 1,
    "exercise_type_id": 1,
    "exercise_name_snapshot": "Running",
    "met_value_snapshot": 9.8,
    "duration_minutes": 30,
    "body_weight_kg": 70.0,
    "calories_burned": 343.0,
    "occurred_at": "2026-04-19 07:00:00",
    "created_at": "2026-04-19 07:30:00"
  }
}
```
