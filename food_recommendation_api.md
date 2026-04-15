# Food Recommendation API

## Base URL
```
http://localhost:8080
```

## Authentication
Requires a Bearer token in the `Authorization` header:
```
Authorization: Bearer <token>
```

---

## GET /food/recommendation

Get personalized food recommendations based on user's liked foods. Falls back to random foods if `is_random` is set.

**Query Parameters:**
- `is_random` (optional, string): Set to any value (e.g., `true`) to get random foods instead of personalized recommendations

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": [
    {
      "id": 1,
      "restaurant_id": 1,
      "name": "Kung Pao Chicken",
      "description": "Spicy stir-fried chicken with peanuts",
      "image": "https://example.com/kungpao.jpg",
      "price": 25.00,
      "restaurant": {
        "id": 1,
        "name": "Sichuan Restaurant",
        "description": "Authentic Sichuan cuisine",
        "location": "123 Food Street",
        "image": "https://example.com/restaurant.jpg"
      }
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Food ID |
| `restaurant_id` | integer | Restaurant ID |
| `name` | string | Food name |
| `description` | string | Food description |
| `image` | string | Food image URL |
| `price` | number | Price |
| `restaurant.id` | integer | Restaurant ID |
| `restaurant.name` | string | Restaurant name |
| `restaurant.description` | string | Restaurant description |
| `restaurant.location` | string | Restaurant address |
| `restaurant.image` | string | Restaurant image URL |
