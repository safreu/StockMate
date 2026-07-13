
# Features

This document describes the planned functionality of the application.

---

# Household Inventory

The application maintains the current stock of all household items.

Every item stores:

* Name
* Category
* Current quantity
* Threshold
* Unit
* Optional notes

Example:

```text
Butter

Current: 2
Threshold: 1
Unit: package
```

---

# Automatic Shopping List

Whenever an item's quantity reaches or falls below its configured threshold, it is automatically added to the shopping list.

Example:

```text
Milk

Current:
1

Threshold:
1

↓

Added automatically
```

Once the inventory rises above the threshold again, the shopping list entry is completed or removed automatically.

This removes the need to manually maintain shopping lists.

---

# QR Based Inventory Updates

Every inventory item has two QR codes.

Increase:

```text
Butter +1
```

Decrease:

```text
Butter -1
```

The dedicated scanner or the mobile application scans these QR codes and immediately performs the corresponding inventory action.

No confirmation dialog is required during normal operation.

---

# Dedicated Scanner

The dedicated Raspberry Pi scanner is the primary way of interacting with the inventory.

Advantages:

* Very fast
* Always available
* No smartphone required
* Multiple scans possible within seconds
* Immediate feedback through LEDs, display or buzzer

---

# Mobile Fallback

The smartphone application serves as a fallback whenever the dedicated scanner is unavailable.

Supported functionality:

* Inventory overview
* Shopping list
* Camera scanning
* QR generation
* Printing
* Sharing
* Settings

The mobile scanner uses exactly the same QR codes as the dedicated scanner.

---

# Barcode Support

Barcodes represent products, not inventory items.

Example:

```text
Kerrygold Butter

↓

Butter
```

```text
Aldi Butter

↓

Butter
```

Multiple products can therefore belong to the same inventory item.

The first time a barcode is scanned, the user chooses the corresponding inventory item. Future scans use this mapping automatically.

---

# Inventory History

Every inventory change is stored as an immutable event.

Instead of simply changing the quantity from 3 to 2, the system records:

```text
Butter

-1

2026-07-13
```

Benefits:

* Complete audit trail
* Statistics
* Purchase history
* Undo functionality

---

# Undo

Undo never modifies existing history.

Instead, a compensating event is created.

Example:

```text
Butter -1

↓

Undo

↓

Butter +1
```

This guarantees that the event history always remains complete and trustworthy.

---

# QR Sharing and Printing

Users can generate printable QR cards for every inventory item.

Supported actions:

* Preview
* Download
* Print
* Share

Sharing uses the operating system's native share functionality, allowing QR cards to be sent via AirDrop, Mail, Messages, WhatsApp or printed directly.

---

# Notifications

The system can notify household members when:

* Items are added to the shopping list
* Inventory reaches critical levels
* Purchases are completed

Push notifications are intended to work on both Android and iOS through the PWA.

---

# Future Features

Potential future extensions include:

* Expiration dates
* Recipes
* NFC tags
* Voice assistants
* Weight sensors
* Multiple scanners
* Consumption statistics
* Price history
* Smart shopping recommendations
