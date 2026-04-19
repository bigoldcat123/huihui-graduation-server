# Meal Record API

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

## GET /meal-record/

Get the user's today's meal records.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 1,
      "user_id": 1,
      "meal_type": "breakfast",
      "source_type": "Inner",
      "total_calories": 500.0,
      "note": null,
      "created_at": "2026-04-19 08:30:00"
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Record ID |
| `user_id` | integer | User ID |
| `meal_type` | string | One of: `breakfast`, `lunch`, `dinner`, `snack` |
| `source_type` | string | One of: `Inner`, `Outer` |
| `total_calories` | float | Total calories for this meal |
| `note` | string | Optional note |
| `created_at` | string | Creation timestamp |

---

## POST /meal-record/

Create a new meal record manually.

**Request Body:**
```json
{
  "meal_type": "lunch",
  "source_type": "Outer",
  "total_calories": 750.0,
  "note": "Rice and chicken"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meal_type` | string | Yes | One of: `breakfast`, `lunch`, `dinner`, `snack` |
| `source_type` | string | Yes | One of: `Inner`, `Outer` |
| `total_calories` | float | Yes | Total calories for this meal |
| `note` | string | No | Optional note |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 2,
    "user_id": 1,
    "meal_type": "lunch",
    "source_type": "Outer",
    "total_calories": 750.0,
    "note": "Rice and chicken",
    "created_at": "2026-04-19 13:00:00"
  }
}
```

---

## POST /meal-record/inner

Create a meal record from an inner food. Calories are fetched from food_attribute table.

**Request Body:**
```json
{
  "food_id": 1,
  "meal_type": "breakfast"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `food_id` | integer | Yes | Food ID to get calories from |
| `meal_type` | string | Yes | One of: `breakfast`, `lunch`, `dinner`, `snack` |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 3,
    "user_id": 1,
    "meal_type": "breakfast",
    "source_type": "Inner",
    "total_calories": 500.0,
    "note": null,
    "created_at": "2026-04-19 08:00:00"
  }
}
```
