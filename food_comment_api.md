# Food Comment API

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

## GET /food/{food_id}/comments

Get all comments for a food.

**Path Parameters:**
- `food_id` (integer): The food ID

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 1,
      "food_id": 1,
      "user_id": 1,
      "content": "Delicious!",
      "create_time": "2026-04-15 10:30:00",
      "thumb_count": 5,
      "thumbed": true
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Comment ID |
| `food_id` | integer | Food ID |
| `user_id` | integer | Author's user ID |
| `content` | string | Comment text |
| `create_time` | string | Creation timestamp |
| `thumb_count` | integer | Number of thumbs |
| `thumbed` | boolean | Whether current user thumbed this comment |

---

## POST /food/{food_id}/comments

Create a new comment for a food.

**Path Parameters:**
- `food_id` (integer): The food ID

**Request Body:**
```json
{
  "content": "This food is amazing!"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | string | Yes | Comment text |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 2,
    "food_id": 1,
    "user_id": 1,
    "content": "This food is amazing!",
    "create_time": "2026-04-15 11:00:00",
    "thumb_count": 0,
    "thumbed": false
  }
}
```

---

## POST /food/comments/{comment_id}/thumb

Toggle thumb on a comment. If user has not thumbed, adds thumb. If already thumbed, removes thumb.

**Path Parameters:**
- `comment_id` (integer): The comment ID

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": true
}
```

| Value | Meaning |
|-------|---------|
| `true` | Thumb was added |
| `false` | Thumb was removed |
