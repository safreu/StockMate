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

## Create a personal household

This route requires authentication. Log in and save the session cookie first.

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Household",
    "kind": "personal"
  }'
```

Expected status: `201 Created`.

Example response:

```json
{
  "id": "<household-uuid>"
}
```

## Create a shared household

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Shared Household",
    "kind": "shared"
  }'
```

Expected status: `201 Created`.

Example response:

```json
{
  "id": "<household-uuid>"
}
```

## Create a second personal household

After successfully creating a personal household, run:

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Another Personal Household",
    "kind": "personal"
  }'
```

Expected status: `409 Conflict`.

## Create a household with an invalid name

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "   ",
    "kind": "shared"
  }'
```

Expected status: `400 Bad Request`.

## Create a household with an invalid kind

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Household",
    "kind": "invalid"
  }'
```

Expected status: `400 Bad Request`.

## Create a household without authentication

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/households" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Household",
    "kind": "shared"
  }'
```

Expected status: `401 Unauthorized`.

## List households

Returns all households the authenticated user belongs to.

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households"
```

Expected status: `200 OK`.

Example response:

```json
[
  {
    "id": "<household-uuid>",
    "name": "My Household",
    "kind": "personal"
  },
  {
    "id": "<household-uuid>",
    "name": "Shared Household",
    "kind": "shared"
  }
]
```

## List households without authentication

```bash
curl -i \
  "$BASE_URL/api/v1/households"
```

Expected status: `401 Unauthorized`.

## Get a household by ID

Returns a household if the authenticated user is a member.

Replace `<household-uuid>` with the ID returned when creating or listing households.

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/<household-uuid>"
```

Expected status: `200 OK`.

Example response:

```json
{
  "id": "<household-uuid>",
  "name": "My Household",
  "kind": "personal"
}
```

## Get an unknown household

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/00000000-0000-0000-0000-000000000000"
```

Expected status: `404 Not Found`.

## Get a household without membership

Use the ID of a household belonging to another user.

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/<other-household-uuid>"
```

Expected status: `403 Forbidden`.

## Get a household without authentication

```bash
curl -i \
  "$BASE_URL/api/v1/households/<household-uuid>"
```

Expected status: `401 Unauthorized`.

## Add a member to a shared household

The authenticated user must be an owner of the household.

Replace `<household-uuid>` with the ID of a shared household.

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households/<household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "member@example.com"
  }'
```

Expected status: `204 No Content`.

## Add an unknown user to a household

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households/<household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "unknown@example.com"
  }'
```

Expected status: `404 Not Found`.

## Add an existing member again

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households/<household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "member@example.com"
  }'
```

Expected status: `409 Conflict`.

## Add a member to a personal household

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households/<personal-household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "member@example.com"
  }'
```

Expected status: `409 Conflict`.

## Add a member without owner permission

Use a session belonging to a household member who is not an owner.

```bash
curl -i \
  -b cookies.txt \
  -X POST "$BASE_URL/api/v1/households/<household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "another@example.com"
  }'
```

Expected status: `403 Forbidden`.

## Add a member without authentication

```bash
curl -i \
  -X POST "$BASE_URL/api/v1/households/<household-uuid>/members" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "member@example.com"
  }'
```

Expected status: `401 Unauthorized`.

## List household members

Returns all members of a household. The authenticated user must belong to the household.

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/<household-uuid>/members"
```

Expected status: `200 OK`.

Example response:

```json
[
  {
    "user_id": "<user-uuid>",
    "display_name": "Samuel",
    "role": "owner"
  },
  {
    "user_id": "<user-uuid>",
    "display_name": "Another User",
    "role": "member"
  }
]
```

## List members of an unknown household

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/00000000-0000-0000-0000-000000000000/members"
```

Expected status: `404 Not Found`.

## List household members without membership

Use the ID of a household the authenticated user does not belong to.

```bash
curl -i \
  -b cookies.txt \
  "$BASE_URL/api/v1/households/<other-household-uuid>/members"
```

Expected status: `403 Forbidden`.

## List household members without authentication

```bash
curl -i \
  "$BASE_URL/api/v1/households/<household-uuid>/members"
```

Expected status: `401 Unauthorized`.

## Remove a household member

The authenticated user must either be the member being removed or an owner of the household.

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/<member-user-uuid>"
```

Expected status: `204 No Content`.

## Leave a household

A normal household member can remove themselves.

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/<your-user-uuid>"
```

Expected status: `204 No Content`.

## Remove a member without permission

Use a session belonging to a normal member and try to remove another member.

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/<other-member-user-uuid>"
```

Expected status: `403 Forbidden`.

## Remove an unknown member

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/00000000-0000-0000-0000-000000000000"
```

Expected status: `404 Not Found`.

## Remove the household owner

```bash
curl -i \
  -b cookies.txt \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/<owner-user-uuid>"
```

Expected status: `409 Conflict`.

## Remove a household member without authentication

```bash
curl -i \
  -X DELETE \
  "$BASE_URL/api/v1/households/<household-uuid>/members/<member-user-uuid>"
```

Expected status: `401 Unauthorized`.