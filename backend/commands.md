# StockMate Manual API Commands

These commands assume the backend is running at `http://127.0.0.1:3000`.

```bash
export BASE_URL="http://127.0.0.1:3000"
```

## Health check

```bash
curl -i "$BASE_URL/api/v1/health"
```

Expected status: `200 OK`.

## Register a user

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "samuel@example.com",
    "display_name": "Samuel",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `201 Created`.

Example response:

```json
{
  "id": "<user-uuid>"
}
```

## Register the same email again

Run the registration command again:

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "samuel@example.com",
    "display_name": "Samuel",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `409 Conflict`.

## Register with an invalid email

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "invalid-email",
    "display_name": "Samuel",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `400 Bad Request`.

## Register with an empty display name

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "another@example.com",
    "display_name": "",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `400 Bad Request`.

## Register with a whitespace-only display name

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "whitespace@example.com",
    "display_name": "   ",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `400 Bad Request`.

## Log in

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "samuel@example.com",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `200 OK`.

The response should include a `Set-Cookie` header containing the session cookie.

## Log in and save the session cookie

```bash
curl -i \
  -c cookies.txt \
  -X POST "$BASE_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "samuel@example.com",
    "password": "SuperSecretPassword123!"
  }'
```

The cookie is stored in `cookies.txt` and can later be sent with:

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/<protected-route>"
```

Replace `<protected-route>` after adding the next authenticated endpoint.

## Log in with a wrong password

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "samuel@example.com",
    "password": "wrong-password"
  }'
```

Expected status: `401 Unauthorized`.

## Log in with an unknown email

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "unknown@example.com",
    "password": "SuperSecretPassword123!"
  }'
```

Expected status: `401 Unauthorized`.

## Inspect the stored cookie

```bash
cat cookies.txt
```

The raw session token should only appear in the cookie file and HTTP response. The database should contain only its hash.
