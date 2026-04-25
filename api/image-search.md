# Image Search API

**Base URL:** `http://localhost:8080/image`

## Response Format

All endpoints return a standard response wrapper:

```json
{
  "code": 200,
  "message": "success",
  "data": <payload>
}
```

---

## Endpoints

### `GET /image`

Health check endpoint.

**Response (200):**
```json
{
  "code": 200,
  "message": "ok",
  "data": "get image"
}
```

---

### `POST /image/insert`

Insert an image with calorie data into the vector database. Forwards to external image search service.

**Content-Type:** `multipart/form-data`

**Form Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image` | file | Yes | Image file (PNG, JPG, etc.) |
| `cal` | integer | Yes | Calorie value |

**Example (curl):**
```bash
curl -X POST http://localhost:8080/image/insert \
  -F "image=@/path/to/food.jpg" \
  -F "cal=250"
```

**Response (200):**
```json
{
  "code": 200,
  "message": "ok",
  "data": null
}
```

**Response (500):** Forwarding or processing failed.

---

### `POST /image`

Search for similar images by image query. Returns the calorie value of the most similar image.

**Content-Type:** `multipart/form-data`

**Form Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image` | file | Yes | Query image file |

**Example (curl):**
```bash
curl -X POST http://localhost:8080/image \
  -F "image=@/path/to/query.jpg"
```

**Response (200):** Returns the calorie value (integer).
```json
{
  "code": 200,
  "message": "ok",
  "data": 250
}
```

**Response (500):** Forwarding or search failed.