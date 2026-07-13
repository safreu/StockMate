
# Error Handling

## Purpose

This document defines the error handling strategy for the application.

Every component of the system (frontend, backend and scanner daemon) should use a consistent error model.

The goals are:

* predictable API behaviour
* meaningful error messages
* machine-readable error codes
* easy debugging
* clear distinction between user errors and internal failures

---

# Error Response Format

Every API error returns the following JSON structure:

```json
{
  "error": {
    "code": "inventory_item_not_found",
    "message": "The requested inventory item does not exist.",
    "details": {}
  }
}
```

| Field     | Description                         |
| --------- | ----------------------------------- |
| `code`    | Stable machine-readable identifier. |
| `message` | Human-readable explanation.         |
| `details` | Optional structured information.    |

The frontend should rely on `code` instead of parsing `message`.

---

# HTTP Status Codes

## 200 OK

The request completed successfully.

Examples:

* retrieving inventory
* retrieving shopping list
* executing an already processed scan (idempotent retry)

---

## 201 Created

A new resource or event has been created.

Examples:

* inventory item
* category
* household
* inventory event
* shopping list entry

---

## 204 No Content

The request completed successfully but does not return a body.

Examples:

* logout
* delete category
* deactivate inventory item

---

## 400 Bad Request

The request format is invalid.

Examples:

* invalid JSON
* missing required field
* malformed UUID
* malformed barcode
* malformed QR URL

Example codes:

```text
invalid_json
missing_field
invalid_uuid
invalid_barcode
invalid_qr_code
invalid_query_parameter
```

---

## 401 Unauthorized

Authentication failed.

Examples:

* no session cookie
* expired session
* invalid session

Codes:

```text
unauthenticated
invalid_credentials
session_expired
```

---

## 403 Forbidden

The authenticated user is not allowed to perform the requested action.

Examples:

* accessing another household
* insufficient permissions
* disabled scanner

Codes:

```text
forbidden
insufficient_permissions
household_access_denied
scanner_disabled
```

---

## 404 Not Found

Requested resource does not exist.

Examples:

* unknown inventory item
* unknown household
* unknown QR token
* unknown barcode mapping

Codes:

```text
inventory_item_not_found
household_not_found
category_not_found
qr_action_not_found
shopping_item_not_found
barcode_mapping_not_found
user_not_found
```

---

## 409 Conflict

The request conflicts with the current state.

Examples:

* undo already performed
* duplicate inventory name
* duplicate household invitation

Codes:

```text
event_already_reversed
duplicate_inventory_item
duplicate_category
duplicate_household_member
already_exists
```

---

## 410 Gone

The resource existed previously but is no longer available.

Possible future use:

* disabled QR action
* deleted invitation

Codes:

```text
qr_action_disabled
resource_removed
```

---

## 422 Unprocessable Entity

The request is syntactically correct but violates business rules.

Examples:

* decreasing inventory below zero
* invalid quantity
* attempting to execute an inactive item

Codes:

```text
quantity_already_zero
negative_quantity_not_allowed
invalid_quantity_change
inventory_item_inactive
invalid_barcode_mapping
shopping_item_already_checked
event_not_reversible
```

---

## 429 Too Many Requests

Rate limiting.

Examples:

* repeated login attempts
* excessive public QR requests

Codes:

```text
too_many_requests
login_rate_limited
scanner_rate_limited
```

---

## 500 Internal Server Error

Unexpected server failure.

Examples:

* panic
* database corruption
* serialization failure

Codes:

```text
internal_server_error
database_error
serialization_error
configuration_error
```

---

## 502 Bad Gateway

An external dependency failed.

Examples:

* Open Food Facts unavailable
* push notification provider unavailable

Codes:

```text
external_service_error
product_lookup_failed
push_service_unavailable
```

---

## 503 Service Unavailable

Temporary outage.

Examples:

* backend starting
* maintenance mode
* database unavailable

Codes:

```text
service_unavailable
database_unavailable
maintenance_mode
scanner_unavailable
```

---

# Business Rule Errors

These errors represent valid requests that cannot be completed.

## Inventory

```text
inventory_item_inactive
quantity_already_zero
negative_quantity_not_allowed
invalid_quantity_change
required_quantity_exceeded
```

---

## QR Actions

```text
qr_action_not_found
qr_action_disabled
invalid_qr_code
unknown_qr_format
```

---

## Shopping List

```text
shopping_item_not_found
shopping_item_already_checked
shopping_item_not_checked
shopping_item_not_missing
```

---

## Inventory Events

```text
event_not_found
event_already_reversed
event_not_reversible
duplicate_event_id
duplicate_scan_id
```

---

## Barcode

```text
barcode_not_found
barcode_not_mapped
barcode_mapping_exists
unsupported_barcode
product_lookup_failed
```

---

## Authentication

```text
invalid_credentials
unauthenticated
session_expired
account_disabled
provider_login_failed
```

---

## Authorization

```text
forbidden
household_access_denied
insufficient_permissions
owner_required
```

---

## Validation

```text
invalid_name
invalid_category
invalid_unit
invalid_quantity
invalid_email
invalid_role
invalid_token
validation_failed
```

---

# Scanner Errors

The scanner daemon distinguishes between scanner errors and backend errors.

## Scanner Errors

```text
scanner_not_connected
scanner_disconnected
scanner_read_failed
unknown_scan_format
```

These errors occur before contacting the backend.

---

## Backend Errors

```text
backend_unreachable
request_timeout
invalid_backend_response
duplicate_scan
```

These occur after attempting communication with the backend.

---

# Frontend Error Handling

The frontend should never compare error messages.

Incorrect:

```typescript
if (error.message === "Quantity already zero") {
    ...
}
```

Correct:

```typescript
switch (error.code) {
    case "quantity_already_zero":
        ...
}
```

Messages may be translated.

Error codes must remain stable.

---

# Logging

Every server error should include:

* timestamp
* request ID
* authenticated user (if available)
* household ID
* endpoint
* error code
* stack trace (development only)

Example:

```text
2026-07-13T18:42:19Z

request_id=019bd421...
user=1
household=1
endpoint=POST /api/v1/qr-actions/execute

code=quantity_already_zero
status=422
```

---

# User-Friendly Messages

Internal codes should be translated into friendly UI messages.

| Error Code               | User Message                                |
| ------------------------ | ------------------------------------------- |
| quantity_already_zero    | This item is already empty.                 |
| qr_action_disabled       | This QR code is no longer valid.            |
| inventory_item_not_found | The requested item could not be found.      |
| unauthenticated          | Please sign in to continue.                 |
| household_access_denied  | You don't have access to this household.    |
| backend_unreachable      | The server is currently unavailable.        |
| product_lookup_failed    | Product information could not be retrieved. |

The backend returns only stable error codes and default English messages.

The frontend is responsible for localization.

---

# Error Code Naming Convention

Error codes use:

```text
snake_case
```

Examples:

```text
inventory_item_not_found
quantity_already_zero
invalid_barcode
event_already_reversed
scanner_not_connected
```

Codes should:

* remain stable across API versions
* be descriptive
* never contain spaces
* never depend on the current UI language
