
# HTTP API

## 1. Purpose

This document defines the first draft of the HTTP API.

The API is divided into:

* public browser API
* authentication routes
* household resources
* inventory resources
* shopping-list resources
* QR action resources
* product barcode resources
* notification resources
* localhost-only scanner API

The public API is versioned under:

```text
/api/v1
```

The scanner API is versioned separately under:

```text
/internal/v1
```

---

## 2. General Conventions

### Content type

JSON requests use:

```http
Content-Type: application/json
```

JSON responses use:

```http
Content-Type: application/json
```

### Authentication

Browser requests use an HTTP-only session cookie.

The scanner endpoint is available only through localhost and validates the configured scanner ID.

### Dates

Dates and timestamps use RFC 3339 UTC strings:

```text
2026-07-13T18:42:19.312Z
```

### IDs

Database entities use integer IDs.

Inventory events and scan requests use UUID strings.

### Error format

All API errors use a common response shape:

```json
{
  "error": {
    "code": "inventory_item_not_found",
    "message": "The requested inventory item does not exist.",
    "details": null
  }
}
```

`code` is intended for programmatic handling.

`message` is intended for display or debugging.

Possible common codes:

```text
unauthenticated
forbidden
validation_failed
not_found
conflict
duplicate_request
database_error
external_service_error
```

---

# 3. Authentication

## `POST /api/v1/auth/login`

Logs in using a local username or email and password.

### Request

```json
{
  "login": "samuel@example.com",
  "password": "user-supplied-password"
}
```

### Response

```http
200 OK
Set-Cookie: __Host-session=...; Path=/; Secure; HttpOnly; SameSite=Lax
```

```json
{
  "user": {
    "id": 1,
    "displayName": "Samuel",
    "email": "samuel@example.com"
  }
}
```

### Errors

```text
401 invalid_credentials
429 too_many_attempts
```

---

## `POST /api/v1/auth/logout`

Destroys the current session.

### Response

```http
204 No Content
```

The response also expires the session cookie.

---

## `GET /api/v1/auth/me`

Returns the currently authenticated user and available households.

### Response

```json
{
  "user": {
    "id": 1,
    "displayName": "Samuel",
    "email": "samuel@example.com"
  },
  "households": [
    {
      "id": 1,
      "name": "Home",
      "role": "owner"
    }
  ]
}
```

### Errors

```text
401 unauthenticated
```

---

## `GET /api/v1/auth/google/start`

Starts the Google OpenID Connect flow.

The backend redirects to Google.

An optional return path can be supplied:

```text
/api/v1/auth/google/start?returnTo=/inventory
```

Only application-local return paths are accepted.

---

## `GET /api/v1/auth/google/callback`

Handles the Google callback.

After validation, the backend:

1. finds or creates the local identity
2. creates the application session
3. redirects to the requested frontend route

---

## `GET /api/v1/auth/apple/start`

Starts Sign in with Apple.

---

## `POST /api/v1/auth/apple/callback`

Processes Apple's callback.

Apple may send callback values using form-encoded POST data.

---

# 4. Households

## `GET /api/v1/households`

Returns households accessible to the current user.

### Response

```json
{
  "households": [
    {
      "id": 1,
      "name": "Home",
      "role": "owner"
    }
  ]
}
```

---

## `POST /api/v1/households`

Creates a household.

### Request

```json
{
  "name": "Home"
}
```

### Response

```http
201 Created
```

```json
{
  "household": {
    "id": 1,
    "name": "Home",
    "role": "owner"
  }
}
```

---

## `GET /api/v1/households/{householdId}/members`

Returns household members.

### Response

```json
{
  "members": [
    {
      "userId": 1,
      "displayName": "Samuel",
      "role": "owner"
    },
    {
      "userId": 2,
      "displayName": "Alex",
      "role": "member"
    }
  ]
}
```

---

# 5. Inventory Categories

## `GET /api/v1/households/{householdId}/categories`

Returns categories.

### Response

```json
{
  "categories": [
    {
      "id": 1,
      "name": "Dairy",
      "sortOrder": 10
    }
  ]
}
```

---

## `POST /api/v1/households/{householdId}/categories`

Creates a category.

### Request

```json
{
  "name": "Dairy",
  "sortOrder": 10
}
```

---

## `PATCH /api/v1/households/{householdId}/categories/{categoryId}`

Updates a category.

### Request

```json
{
  "name": "Dairy and Eggs",
  "sortOrder": 20
}
```

---

# 6. Inventory Items

## `GET /api/v1/households/{householdId}/inventory-items`

Returns the household inventory.

Optional query parameters:

```text
?search=butter
?categoryId=1
?belowThreshold=true
?active=true
```

### Response

```json
{
  "items": [
    {
      "id": 42,
      "name": "Butter",
      "category": {
        "id": 1,
        "name": "Dairy"
      },
      "unit": "package",
      "currentQuantity": 2,
      "shoppingThreshold": 1,
      "defaultRestockQuantity": 2,
      "onShoppingList": false,
      "active": true,
      "updatedAt": "2026-07-13T18:42:19Z"
    }
  ]
}
```

---

## `POST /api/v1/households/{householdId}/inventory-items`

Creates an inventory item.

### Request

```json
{
  "name": "Butter",
  "categoryId": 1,
  "unit": "package",
  "currentQuantity": 2,
  "shoppingThreshold": 1,
  "defaultRestockQuantity": 2,
  "notes": null
}
```

### Response

```http
201 Created
```

```json
{
  "item": {
    "id": 42,
    "name": "Butter",
    "unit": "package",
    "currentQuantity": 2,
    "shoppingThreshold": 1,
    "defaultRestockQuantity": 2,
    "onShoppingList": false
  },
  "qrActions": {
    "decrease": {
      "token": "K71F4HD8M29Q",
      "url": "https://inventory.example.com/q/K71F4HD8M29Q"
    },
    "increase": {
      "token": "RQ7D39M2C8AP",
      "url": "https://inventory.example.com/q/RQ7D39M2C8AP"
    }
  }
}
```

The backend may automatically generate the standard increase and decrease actions.

---

## `GET /api/v1/households/{householdId}/inventory-items/{itemId}`

Returns one item with additional information.

### Response

```json
{
  "item": {
    "id": 42,
    "name": "Butter",
    "category": {
      "id": 1,
      "name": "Dairy"
    },
    "unit": "package",
    "currentQuantity": 2,
    "shoppingThreshold": 1,
    "defaultRestockQuantity": 2,
    "notes": null,
    "onShoppingList": false,
    "active": true
  },
  "qrActions": [
    {
      "id": 100,
      "kind": "decrease",
      "quantityChange": -1,
      "url": "https://inventory.example.com/q/K71F4HD8M29Q",
      "enabled": true
    },
    {
      "id": 101,
      "kind": "increase",
      "quantityChange": 1,
      "url": "https://inventory.example.com/q/RQ7D39M2C8AP",
      "enabled": true
    }
  ]
}
```

---

## `PATCH /api/v1/households/{householdId}/inventory-items/{itemId}`

Updates item metadata.

### Request

```json
{
  "name": "Butter",
  "categoryId": 1,
  "unit": "package",
  "shoppingThreshold": 2,
  "defaultRestockQuantity": 2,
  "notes": "Usually stored in the refrigerator door"
}
```

This endpoint does not directly change `currentQuantity`.

Quantity changes use the inventory adjustment endpoint so that an immutable event is always created.

---

## `DELETE /api/v1/households/{householdId}/inventory-items/{itemId}`

Soft-deactivates the inventory item.

### Response

```http
204 No Content
```

The initial implementation should not physically delete inventory history.

---

# 7. Manual Inventory Adjustments

## `POST /api/v1/households/{householdId}/inventory-items/{itemId}/adjustments`

Creates a manual inventory event.

### Request

```json
{
  "eventId": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
  "quantityChange": -1,
  "reason": "manual_adjustment"
}
```

### Response

```http
201 Created
```

```json
{
  "event": {
    "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
    "inventoryItemId": 42,
    "quantityChange": -1,
    "quantityBefore": 3,
    "quantityAfter": 2,
    "source": "manual",
    "createdAt": "2026-07-13T18:42:19.312Z"
  },
  "item": {
    "id": 42,
    "name": "Butter",
    "currentQuantity": 2,
    "onShoppingList": false
  }
}
```

The event ID acts as the idempotency key.

---

# 8. QR Actions

## `GET /q/{token}`

Loads the frontend QR action route.

This route does not modify inventory.

The React application automatically calls the execute endpoint once it has loaded and verified the session.

---

## `POST /api/v1/qr-actions/{token}/execute`

Executes a reusable QR action.

### Request

```json
{
  "eventId": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
  "source": "pwa_scanner"
}
```

Allowed public sources:

```text
pwa_scanner
public_qr_page
```

The server determines the user from the session. The client cannot supply an arbitrary user ID.

### Response

```json
{
  "event": {
    "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
    "inventoryItemId": 42,
    "quantityChange": -1,
    "quantityBefore": 2,
    "quantityAfter": 1,
    "source": "pwa_scanner",
    "createdAt": "2026-07-13T18:42:19.312Z"
  },
  "item": {
    "id": 42,
    "name": "Butter",
    "currentQuantity": 1,
    "shoppingThreshold": 1,
    "onShoppingList": true
  },
  "shoppingListChange": {
    "type": "added",
    "entryId": 51
  }
}
```

When the same `eventId` is submitted again, the backend returns the original successful response without applying another quantity change.

A new event ID with the same QR token represents another valid scan.

---

## `POST /api/v1/households/{householdId}/inventory-items/{itemId}/qr-actions`

Creates an additional QR action.

### Request

```json
{
  "quantityChange": -2
}
```

### Response

```http
201 Created
```

```json
{
  "qrAction": {
    "id": 105,
    "quantityChange": -2,
    "token": "Y6P7H89FQ2LA",
    "url": "https://inventory.example.com/q/Y6P7H89FQ2LA",
    "enabled": true
  }
}
```

---

## `PATCH /api/v1/households/{householdId}/qr-actions/{actionId}`

Enables or disables an action.

### Request

```json
{
  "enabled": false
}
```

Disabling an action invalidates already printed QR codes using its token.

---

## `POST /api/v1/households/{householdId}/qr-actions/{actionId}/rotate`

Revokes the existing token and generates a new one.

Use this if a QR code is lost, copied, or should no longer be usable.

---

# 9. Inventory History

## `GET /api/v1/households/{householdId}/inventory-events`

Returns inventory activity.

Optional query parameters:

```text
?itemId=42
?source=usb_scanner
?limit=50
?cursor=...
```

### Response

```json
{
  "events": [
    {
      "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
      "item": {
        "id": 42,
        "name": "Butter"
      },
      "quantityChange": -1,
      "quantityBefore": 2,
      "quantityAfter": 1,
      "source": "usb_scanner",
      "sourceIdentifier": "kitchen-scanner",
      "reversed": false,
      "createdAt": "2026-07-13T18:42:19.312Z"
    }
  ],
  "nextCursor": null
}
```

---

## `POST /api/v1/households/{householdId}/inventory-events/{eventId}/undo`

Creates a compensating inventory event.

### Request

```json
{
  "undoEventId": "019bd423-1ed1-76f0-bfd0-4bd6c0e7658e"
}
```

### Response

```json
{
  "originalEvent": {
    "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
    "reversed": true
  },
  "undoEvent": {
    "id": "019bd423-1ed1-76f0-bfd0-4bd6c0e7658e",
    "quantityChange": 1,
    "quantityBefore": 1,
    "quantityAfter": 2,
    "reversedEventId": "019bd421-6e50-7e21-bcb3-88be20e92bf5"
  },
  "item": {
    "id": 42,
    "name": "Butter",
    "currentQuantity": 2,
    "onShoppingList": false
  }
}
```

### Errors

```text
409 event_already_reversed
409 event_not_reversible
```

---

# 10. Shopping List

## `GET /api/v1/households/{householdId}/shopping-list`

Returns active shopping-list entries.

### Response

```json
{
  "entries": [
    {
      "id": 51,
      "item": {
        "id": 42,
        "name": "Butter",
        "unit": "package",
        "category": {
          "id": 1,
          "name": "Dairy"
        }
      },
      "requestedQuantity": 2,
      "currentQuantity": 1,
      "shoppingThreshold": 1,
      "source": "threshold",
      "createdAt": "2026-07-13T18:42:19Z"
    }
  ]
}
```

---

## `POST /api/v1/households/{householdId}/shopping-list`

Adds an item manually.

### Request

```json
{
  "inventoryItemId": 42,
  "requestedQuantity": 2
}
```

### Response

```http
201 Created
```

If an active entry already exists, the API may update its requested quantity or return a conflict. The exact version 1 behavior should be documented before implementation.

Recommended behavior: return the existing entry and update its quantity.

---

## `PATCH /api/v1/households/{householdId}/shopping-list/{entryId}`

Updates the requested quantity.

### Request

```json
{
  "requestedQuantity": 3
}
```

---

## `POST /api/v1/households/{householdId}/shopping-list/{entryId}/complete`

Marks a shopping-list entry as completed.

This endpoint does not automatically increase the inventory unless a restock quantity is supplied.

### Request without restock

```json
{}
```

### Request with restock

```json
{
  "restockQuantity": 2,
  "eventId": "019bd430-d861-7ef0-a084-621cf1cfcafd"
}
```

When `restockQuantity` is present, completing the entry and increasing inventory happen in one transaction.

---

# 11. Product Barcodes

## `GET /api/v1/households/{householdId}/products/barcodes/{barcode}`

Looks up a local mapping.

### Known barcode response

```json
{
  "status": "mapped",
  "product": {
    "barcode": "4001234567890",
    "productName": "Example Brand Butter",
    "brand": "Example Brand",
    "packageQuantity": "250 g"
  },
  "inventoryItem": {
    "id": 42,
    "name": "Butter"
  }
}
```

### Unknown barcode response

```json
{
  "status": "unknown",
  "externalProduct": null
}
```

---

## `POST /api/v1/households/{householdId}/products/barcodes/{barcode}/lookup`

Attempts an external product lookup.

The backend queries a configured external product service and caches the result.

### Response

```json
{
  "status": "found",
  "externalProduct": {
    "barcode": "4001234567890",
    "productName": "Example Brand Butter",
    "brand": "Example Brand",
    "packageQuantity": "250 g",
    "imageUrl": "https://example.invalid/product.jpg",
    "source": "open_food_facts"
  }
}
```

A product returned by the external service is not automatically mapped to an inventory item.

---

## `PUT /api/v1/households/{householdId}/products/barcodes/{barcode}/mapping`

Creates or replaces a mapping.

### Request

```json
{
  "inventoryItemId": 42,
  "productName": "Example Brand Butter",
  "brand": "Example Brand",
  "packageQuantity": "250 g",
  "dataSource": "open_food_facts"
}
```

---

## `POST /api/v1/households/{householdId}/products/barcodes/{barcode}/purchase`

Records a purchase and optionally increases stock.

### Request

```json
{
  "quantity": 2,
  "priceCents": 438,
  "currency": "EUR",
  "storeName": "Local Supermarket",
  "eventId": "019bd439-90d4-7f50-b299-1c74fa0de31e"
}
```

The mapped inventory item is increased by `quantity`.

The purchase record, inventory event, inventory update, and shopping-list update occur in one transaction.

---

# 12. QR Card Generation

## `GET /api/v1/households/{householdId}/inventory-items/{itemId}/qr-card`

Returns the data needed for a frontend-generated QR card.

### Response

```json
{
  "item": {
    "id": 42,
    "name": "Butter",
    "unit": "package"
  },
  "actions": {
    "decrease": {
      "label": "Remove one",
      "quantityChange": -1,
      "url": "https://inventory.example.com/q/K71F4HD8M29Q"
    },
    "increase": {
      "label": "Add one",
      "quantityChange": 1,
      "url": "https://inventory.example.com/q/RQ7D39M2C8AP"
    }
  }
}
```

The frontend may render this as PNG, HTML, or canvas content.

---

## `GET /api/v1/households/{householdId}/inventory-items/{itemId}/qr-card.pdf`

Optional endpoint for server-generated printable PDFs.

### Response

```http
200 OK
Content-Type: application/pdf
Content-Disposition: inline; filename="butter-qr-card.pdf"
```

---

# 13. Push Notifications

## `POST /api/v1/push-subscriptions`

Registers a browser push subscription.

### Request

```json
{
  "endpoint": "https://push-service.example/...",
  "keys": {
    "p256dh": "...",
    "auth": "..."
  }
}
```

### Response

```http
201 Created
```

---

## `DELETE /api/v1/push-subscriptions`

Removes a subscription.

### Request

```json
{
  "endpoint": "https://push-service.example/..."
}
```

---

## `POST /api/v1/push-subscriptions/test`

Sends a test notification to the current user's subscriptions.

This route should be available only in settings or development mode.

---

# 14. Internal Scanner API

The internal scanner API must listen on localhost and must not be exposed by Caddy.

## `POST /internal/v1/scans`

Processes a physical scanner event.

### Request

```json
{
  "scanId": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
  "scannerId": "kitchen-scanner",
  "rawCode": "https://inventory.example.com/q/K71F4HD8M29Q",
  "scannedAt": "2026-07-13T18:42:19.312Z"
}
```

The backend:

1. validates the scanner
2. parses the QR action token
3. checks whether the scan ID already exists
4. applies the inventory action
5. updates the shopping list
6. stores the response
7. returns the resulting state

### Response

```json
{
  "status": "succeeded",
  "event": {
    "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
    "quantityChange": -1,
    "quantityBefore": 2,
    "quantityAfter": 1
  },
  "item": {
    "id": 42,
    "name": "Butter",
    "currentQuantity": 1
  },
  "shoppingListChange": {
    "type": "added"
  },
  "feedback": {
    "kind": "success",
    "displayText": "Butter: 1",
    "soundPattern": "success"
  }
}
```

### Already processed response

A repeated `scanId` returns the original result:

```json
{
  "status": "already_processed",
  "event": {
    "id": "019bd421-6e50-7e21-bcb3-88be20e92bf5",
    "quantityBefore": 2,
    "quantityAfter": 1
  }
}
```

It must not decrement the quantity again.

### Rejected response

```http
422 Unprocessable Entity
```

```json
{
  "error": {
    "code": "quantity_already_zero",
    "message": "Butter is already at zero.",
    "details": {
      "itemId": 42,
      "currentQuantity": 0
    }
  },
  "feedback": {
    "kind": "warning",
    "displayText": "Butter already empty",
    "soundPattern": "warning"
  }
}
```

---

# 15. Health Endpoints

## `GET /health/live`

Confirms that the backend process is running.

### Response

```json
{
  "status": "ok"
}
```

---

## `GET /health/ready`

Confirms that required dependencies, especially SQLite, are available.

### Response

```json
{
  "status": "ready",
  "database": "ok"
}
```

These endpoints should not expose sensitive configuration.

---

# 16. HTTP Status Codes

Recommended status usage:

```text
200 OK
    Successful read or idempotent repeated operation

201 Created
    Resource or event created

204 No Content
    Successful operation without response body

400 Bad Request
    Invalid JSON or malformed parameters

401 Unauthorized
    Missing or invalid login session

403 Forbidden
    User does not belong to the household

404 Not Found
    Resource or QR action does not exist

409 Conflict
    State conflict, such as an already reversed event

422 Unprocessable Entity
    Valid request format but invalid business operation

429 Too Many Requests
    Login or public endpoint rate limit

500 Internal Server Error
    Unexpected server failure

502 Bad Gateway
    External product or push service failed
```

---

# 17. API Versioning

Breaking API changes create a new major path:

```text
/api/v1
/api/v2
```

Adding optional response fields is not considered breaking.

Renaming or removing fields is breaking.

The internal scanner API is also versioned so the scanner daemon and backend can be upgraded independently.

---

# 18. Open Decisions

The following API decisions remain open:

* whether shopping-list completion always restocks inventory
* whether manually created shopping entries may be auto-completed
* whether one user may belong to multiple active households in version 1
* whether API pagination uses cursors or page numbers everywhere
* whether QR PDFs are generated in the frontend or backend
* whether scanner authentication needs an additional shared secret despite localhost binding
* whether barcode purchases default to the item's configured restock quantity
