
# Smart Inventory & Shopping List

## Project Vision

The goal of this project is to create a **household inventory management system** that minimizes the effort required to maintain a shopping list.

Instead of manually opening an app and adding products, the primary interaction is performed through **printed QR codes** and a **dedicated scanner** connected to a Raspberry Pi.

The smartphone application is intentionally a **secondary interface**, used for:

* viewing the inventory
* viewing the shopping list
* configuring items
* generating and printing QR codes
* correcting mistakes (undo)
* fallback scanning if the dedicated scanner is unavailable

---

# Core Idea

Every inventory item has two QR codes:

* Increase (+1)
* Decrease (-1)

Example:

```
Butter

+------------------------------+
|        BUTTER                |
|                              |
|  REMOVE (-1)   ADD (+1)      |
|    [ QR ]        [ QR ]      |
+------------------------------+
```

Normal workflow:

```
Take butter

↓

Scan REMOVE QR

↓

Inventory:
3 → 2

↓

If threshold reached:

Automatically added to shopping list
```

Buying groceries:

```
Come home

↓

Scan ADD QR

↓

Inventory:
0 → 1

↓

Removed from shopping list
```

No confirmation dialogs should be required.

---

# Architecture

```
                    +-------------------+
                    | React PWA         |
                    | (Fallback UI)     |
                    +---------+---------+
                              |
                              |
                        HTTPS API
                              |
+--------------------+        |
| USB QR Scanner     |        |
| Raspberry Pi       |        |
+---------+----------+        |
          |                   |
          |                   |
      Scanner daemon          |
          |                   |
          +--------+----------+
                   |
                   v
          Rust Backend (Axum)
                   |
            Inventory Service
                   |
              SQLite Database
```

---

# Repository Structure

```
shopping-list/

├── frontend/
│   ├── React
│   ├── TypeScript
│   ├── Vite
│   └── PWA
│
├── backend/
│   ├── Axum
│   ├── Authentication
│   ├── Inventory API
│   ├── Shopping List
│   └── Database
│
├── scanner/
│   ├── Rust daemon
│   ├── Reads USB scanner
│   └── Sends scan events
│
├── deployment/
│
└── README.md
```

---

# Technology Stack

## Frontend

* React
* TypeScript
* Vite
* PWA
* ZXing (fallback camera scanner)

## Backend

* Rust
* Axum
* SQLx
* SQLite
* tower-sessions

## Scanner

Rust daemon

Reads USB HID scanner

Communicates with backend through localhost.

---

# Authentication

The browser authenticates through secure HTTP-only session cookies.

Possible login providers:

* Google
* Apple
* Local login

Authentication should be independent from inventory logic.

The scanner itself does **not** authenticate as a user.

---

# QR Codes

Every inventory action has a unique action token.

Example:

```
https://inventory.example.com/q/A7EF8C91
```

The QR identifies

* inventory item
* action (+1/-1)

The QR **does not** represent a unique scan.

Every scan becomes a new inventory event.

---

# Scanner

Primary input device.

Connected through USB.

Expected behavior:

```
Scan QR

↓

Scanner decodes QR internally

↓

Outputs text

↓

Presses ENTER
```

The scanner daemon:

* waits for a complete scan
* generates a unique scan/event ID
* calls the inventory service

---

# Smartphone Fallback

Two fallback methods:

## Inside the PWA

```
Open app

↓

Scan QR

↓

ZXing decodes

↓

POST inventory action
```

Preferred smartphone workflow.

## Camera App

Scanning the QR URL should also work by opening the web application.

This is considered a secondary fallback.

---

# Inventory Model

Never only store:

```
Butter = 3
```

Instead store immutable inventory events.

Example:

```
Butter -1

Butter -1

Butter +2
```

Current inventory is a cached value.

---

# Undo

Undo does **not** delete history.

Instead it creates a compensating event.

Example:

```
Butter -1

↓

Undo

↓

Butter +1
```

History always remains complete.

---

# Shopping List

Every item has:

* current quantity
* threshold

Example:

```
Butter

Current:
2

Threshold:
1
```

When

```
current <= threshold
```

the item automatically appears in the shopping list.

Buying the item increases inventory and removes it from the shopping list.

---

# Barcode Support

Barcodes are optional.

Purpose:

Buying products.

Workflow:

```
Scan supermarket barcode

↓

Lookup product

↓

Known?

↓

Map to inventory item

↓

Increase inventory
```

Barcodes represent products.

Inventory represents generic household items.

Example:

```
Kerrygold Butter

↓

Butter

Aldi Butter

↓

Butter

Meggle Butter

↓

Butter
```

The mapping is stored locally after the first scan.

Potential data source:

Open Food Facts.

---

# QR Generation

Each inventory item can generate printable QR cards.

Example:

```
Butter

[ REMOVE QR ]
[ ADD QR ]
```

The application supports:

* Preview
* Download
* Print
* Share

Sharing uses the native operating system share sheet.

Possible destinations:

* AirDrop
* Mail
* WhatsApp
* Messages
* Files
* Print

---

# Scanner Feedback

The scanner station should provide immediate feedback.

Ideas:

* Green LED
* Red LED
* Buzzer
* Small display

Example display:

```
✓ Butter

Stock:
3
```

---

# Future Features

## Shared households

Multiple users.

## Purchase history

Track previous purchases.

## Price tracking

Store

* shop
* date
* price

for every purchase.

## Recipes

Automatically determine missing ingredients.

## Expiration dates

Optional.

## Statistics

Examples:

* most consumed products
* average purchase interval
* average monthly spending

## NFC tags

Alternative to QR codes.

---

# Development Roadmap

## Phase 1

* Rust backend
* SQLite
* React frontend
* Login
* Inventory CRUD
* Shopping list
* QR generation

---

## Phase 2

* Scanner daemon
* USB scanner support
* Inventory events
* Undo
* Threshold logic

---

## Phase 3

* PWA
* Camera scanner
* Native sharing
* Printing
* PDF generation

---

## Phase 4

* Barcode support
* Open Food Facts integration
* Product mapping
* Purchase history

---

## Phase 5

* Statistics
* Household sharing
* Notifications
* Polish
