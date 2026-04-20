# Exercise Type API

## Base URL
```
http://localhost:8899
```

---

## GET /exercise-type/

Get all exercise types.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 1,
      "name": "Running",
      "met_value": 9.8
    },
    {
      "id": 2,
      "name": "Walking",
      "met_value": 3.8
    }
  ]
}
```

---

## GET /exercise-type/{id}

Get an exercise type by ID.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 1,
    "name": "Running",
    "met_value": 9.8
  }
}
```

---

## POST /exercise-type/

Create a new exercise type.

**Request Body:**
```json
{
  "name": "Swimming",
  "met_value": 7.0
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Exercise name |
| `met_value` | float | Yes | MET value |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 3,
    "name": "Swimming",
    "met_value": 7.0
  }
}
```

---

## POST /exercise-type/update

Update an exercise type.

**Request Body:**
```json
{
  "id": 3,
  "name": "Swimming (freestyle)",
  "met_value": 8.0
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | integer | Yes | Exercise type ID |
| `name` | string | Yes | Updated exercise name |
| `met_value` | float | Yes | Updated MET value |

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "id": 3,
    "name": "Swimming (freestyle)",
    "met_value": 8.0
  }
}
```

---

## POST /exercise-type/delete/{id}

Delete an exercise type.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": null
}
```
