# Huihui Server API

## Base
- Base URL: `http://<host>:<port>`
- Content-Type: `application/json`

## Response Wrapper
All JSON endpoints return:

```json
{
  "code": 200,
  "message": "ok",
  "data": {}
}
```

On error:

```json
{
  "code": 500,
  "message": "<error>",
  "data": null
}
```

## Endpoints

### GET /
- Method: `GET`
- Path: `/`
- Body: none
- Response: plain text `"hello"`

### POST /auth/login
- Method: `POST`
- Path: `/auth/login`
- Body:
```json
{
  "username": "string",
  "password": "string"
}
```
- Success `data`:
```json
{
  "token": "<jwt>"
}
```

### POST /auth/register
- Method: `POST`
- Path: `/auth/register`
- Body:
```json
{
  "email": "user@example.com",
  "username": "string",
  "password": "string"
}
```
- Success `data`:
```json
{
  "token": "<jwt>"
}
```

### GET /auth/me
- Method: `GET`
- Path: `/auth/me`
- Header: `Authorization: Bearer <jwt>`
- Body: none
- Success `data`:
```json
{
  "id": 1,
  "email": "user@example.com",
  "name": "username"
}
```

### POST /food/consecutiveSuggest
- Method: `POST`
- Path: `/food/consecutiveSuggest`
- Body (`SuggestionInput`):
```json
{
  "food_ids": [1, 2, 3, 4],
  "selected_food_ids": [1, 2]
}
```
- Success `data` (`Vec<FoodRow>`):
```json
[
  {
    "id": 10,
    "restaurant_id": 2,
    "name": "Spicy Chicken",
    "description": "...",
    "image": "https://..."
  }
]
```

## Notes
- Mounted route groups: `/auth/*`, `/food/*`, and `/`.
- JWT expiry is 24 hours.
