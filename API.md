# Food Attribute API

## GET /food/attribute/{food_id}

Get food nutritional attributes.

**Response:**
```json
{
  "code": 200,
  "message": "ok",
  "data": {
    "food_id": 1,
    "calories": 500.00,
    "protein": 30.00,
    "fat": 20.00,
    "carbohydrates": 50.00,
    "fiber": 5.00,
    "sugar": 10.00,
    "sodium": 800.00,
    "serving_size": "200g",
    "is_vegetarian": false,
    "is_vegan": false,
    "is_gluten_free": true,
    "allergens": "nuts",
    "ingredients": "chicken, rice, vegetables"
  }
}
```
