#!/usr/bin/env bash

set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:3000}"

COOKIE_FILE="$(mktemp)"
RESPONSE_FILE="$(mktemp)"

cleanup() {
    rm -f "$COOKIE_FILE" "$RESPONSE_FILE"
}

trap cleanup EXIT

# Unique users make the script safe to run repeatedly
RUN_ID="$(date +%s)"

OWNER_EMAIL="smoke-owner-${RUN_ID}@example.com"
MEMBER_EMAIL="smoke-member-${RUN_ID}@example.com"
PASSWORD="SuperSecretPassword123!"

assert_status() {
    local actual="$1"
    local expected="$2"
    local description="$3"

    if [[ "$actual" != "$expected" ]]; then
        echo "FAIL: $description"
        echo "  expected: $expected"
        echo "  actual:   $actual"

        if [[ -s "$RESPONSE_FILE" ]]; then
            echo "  response:"
            cat "$RESPONSE_FILE"
            echo
        fi

        exit 1
    fi

    echo "PASS: $description"
}

echo
echo "Running StockMate API smoke tests"
echo "Base URL: $BASE_URL"
echo

# ---------------------------------------------------------------------------
# Health
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        "$BASE_URL/api/v1/health"
)"

assert_status "$STATUS" "200" "health check"

# ---------------------------------------------------------------------------
# Register owner
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -X POST "$BASE_URL/api/v1/auth/register" \
        -H "Content-Type: application/json" \
        -d "{
            \"email\": \"$OWNER_EMAIL\",
            \"display_name\": \"Smoke Owner\",
            \"password\": \"$PASSWORD\"
        }"
)"

assert_status "$STATUS" "201" "register owner"

OWNER_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ -z "$OWNER_ID" || "$OWNER_ID" == "null" ]]; then
    echo "FAIL: owner registration did not return an id"
    exit 1
fi

# ---------------------------------------------------------------------------
# Register member
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -X POST "$BASE_URL/api/v1/auth/register" \
        -H "Content-Type: application/json" \
        -d "{
            \"email\": \"$MEMBER_EMAIL\",
            \"display_name\": \"Smoke Member\",
            \"password\": \"$PASSWORD\"
        }"
)"

assert_status "$STATUS" "201" "register member"

MEMBER_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ -z "$MEMBER_ID" || "$MEMBER_ID" == "null" ]]; then
    echo "FAIL: member registration did not return an id"
    exit 1
fi

# ---------------------------------------------------------------------------
# Login owner
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -c "$COOKIE_FILE" \
        -X POST "$BASE_URL/api/v1/auth/login" \
        -H "Content-Type: application/json" \
        -d "{
            \"email\": \"$OWNER_EMAIL\",
            \"password\": \"$PASSWORD\"
        }"
)"

assert_status "$STATUS" "200" "login owner"

# ---------------------------------------------------------------------------
# Protected endpoint without authentication
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        "$BASE_URL/api/v1/households"
)"

assert_status "$STATUS" "401" "reject unauthenticated household request"

# ---------------------------------------------------------------------------
# Create shared household
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X POST "$BASE_URL/api/v1/households" \
        -H "Content-Type: application/json" \
        -d '{
            "name": "Smoke Test Household",
            "kind": "shared"
        }'
)"

assert_status "$STATUS" "201" "create shared household"

HOUSEHOLD_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ -z "$HOUSEHOLD_ID" || "$HOUSEHOLD_ID" == "null" ]]; then
    echo "FAIL: household creation did not return an id"
    exit 1
fi

# ---------------------------------------------------------------------------
# Rename household
# ---------------------------------------------------------------------------

RENAMED_HOUSEHOLD_NAME="Renamed Smoke Test Household"

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X PATCH \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID" \
        -H "Content-Type: application/json" \
        -d "{
            \"name\": \"$RENAMED_HOUSEHOLD_NAME\"
        }"
)"

assert_status "$STATUS" "204" "rename household"

# ---------------------------------------------------------------------------
# Verify renamed household
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID"
)"

assert_status "$STATUS" "200" "get renamed household"

RETURNED_NAME="$(jq -r '.name' "$RESPONSE_FILE")"

if [[ "$RETURNED_NAME" != "$RENAMED_HOUSEHOLD_NAME" ]]; then
    echo "FAIL: household rename was not persisted"
    echo "  expected: $RENAMED_HOUSEHOLD_NAME"
    echo "  actual:   $RETURNED_NAME"
    exit 1
fi

echo "PASS: household rename was persisted"

# ---------------------------------------------------------------------------
# List households
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        "$BASE_URL/api/v1/households"
)"

assert_status "$STATUS" "200" "list households"

if ! jq -e --arg id "$HOUSEHOLD_ID" \
    '.[] | select(.id == $id)' \
    "$RESPONSE_FILE" >/dev/null; then
    echo "FAIL: created household was not returned by list households"
    exit 1
fi

echo "PASS: created household appears in household list"

# ---------------------------------------------------------------------------
# Get household
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID"
)"

assert_status "$STATUS" "200" "get household"

RETURNED_HOUSEHOLD_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ "$RETURNED_HOUSEHOLD_ID" != "$HOUSEHOLD_ID" ]]; then
    echo "FAIL: get household returned unexpected household"
    exit 1
fi

echo "PASS: get household returned correct household"

# ---------------------------------------------------------------------------
# Add member
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X POST "$BASE_URL/api/v1/households/$HOUSEHOLD_ID/members" \
        -H "Content-Type: application/json" \
        -d "{
            \"email\": \"$MEMBER_EMAIL\"
        }"
)"

assert_status "$STATUS" "204" "add household member"

# ---------------------------------------------------------------------------
# Adding same member again must fail
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X POST "$BASE_URL/api/v1/households/$HOUSEHOLD_ID/members" \
        -H "Content-Type: application/json" \
        -d "{
            \"email\": \"$MEMBER_EMAIL\"
        }"
)"

assert_status "$STATUS" "409" "reject duplicate household member"

# ---------------------------------------------------------------------------
# List members
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID/members"
)"

assert_status "$STATUS" "200" "list household members"

MEMBER_COUNT="$(jq 'length' "$RESPONSE_FILE")"

if [[ "$MEMBER_COUNT" -ne 2 ]]; then
    echo "FAIL: expected 2 household members, got $MEMBER_COUNT"
    exit 1
fi

echo "PASS: household contains two members"

if ! jq -e --arg id "$OWNER_ID" \
    '.[] | select(.user_id == $id and .role == "owner")' \
    "$RESPONSE_FILE" >/dev/null; then
    echo "FAIL: owner missing from household member list"
    exit 1
fi

echo "PASS: owner appears in member list"

if ! jq -e --arg id "$MEMBER_ID" \
    '.[] | select(.user_id == $id and .role == "member")' \
    "$RESPONSE_FILE" >/dev/null; then
    echo "FAIL: member missing from household member list"
    exit 1
fi

echo "PASS: member appears in member list"

# ---------------------------------------------------------------------------
# Remove member
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X DELETE \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID/members/$MEMBER_ID"
)"

assert_status "$STATUS" "204" "remove household member"

# ---------------------------------------------------------------------------
# Verify removal
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        "$BASE_URL/api/v1/households/$HOUSEHOLD_ID/members"
)"

assert_status "$STATUS" "200" "list members after removal"

if jq -e --arg id "$MEMBER_ID" \
    '.[] | select(.user_id == $id)' \
    "$RESPONSE_FILE" >/dev/null; then
    echo "FAIL: removed member still appears in household"
    exit 1
fi

echo "PASS: removed member no longer appears in household"

# ---------------------------------------------------------------------------
# Create category
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X POST \
        "$BASE_URL/api/v1/inventory/$HOUSEHOLD_ID/categories" \
        -H "Content-Type: application/json" \
        -d '{
            "name": "Smoke Test Food"
        }'
)"

assert_status "$STATUS" "201" "create category"

CATEGORY_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ -z "$CATEGORY_ID" || "$CATEGORY_ID" == "null" ]]; then
    echo "FAIL: category creation did not return an id"
    exit 1
fi

echo "PASS: category creation returned an id"

# ---------------------------------------------------------------------------
# Create inventory item
# ---------------------------------------------------------------------------

STATUS="$(
    curl -sS \
        -o "$RESPONSE_FILE" \
        -w "%{http_code}" \
        -b "$COOKIE_FILE" \
        -X POST \
        "$BASE_URL/api/v1/inventory/$HOUSEHOLD_ID" \
        -d "{
            \"category_id\": \"$CATEGORY_ID\",
            \"name\": \"Smoke Test Milk\",
            \"current_stock\": 2,
            \"reorder_threshold\": 1,
            \"priority\": \"high\"
        }"      -H "Content-Type: application/json" \
)"

assert_status "$STATUS" "201" "create inventory item"

INVENTORY_ITEM_ID="$(jq -r '.id' "$RESPONSE_FILE")"

if [[ -z "$INVENTORY_ITEM_ID" || "$INVENTORY_ITEM_ID" == "null" ]]; then
    echo "FAIL: inventory item creation did not return an id"
    exit 1
fi

echo "PASS: inventory item creation returned an id"

echo
echo "================================="
echo "All StockMate smoke tests passed."
echo "================================="
echo