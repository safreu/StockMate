
# Roadmap

This roadmap describes the planned development of the project. The goal is to always have a usable application after every milestone.

---

# Milestone 1 – Project Foundation

Goal:
Create the basic infrastructure.

Tasks:

* Create Git repository
* Create Cargo workspace
* Create React frontend
* Configure TypeScript
* Configure formatting and linting
* Configure CI pipeline
* Create SQLite database
* Configure SQL migrations
* Setup deployment on Raspberry Pi

Deliverable:

A running frontend and backend communicating through a REST API.

---

# Milestone 2 – Authentication

Goal:
Users should be able to securely access the application.

Tasks:

* Session management
* Login page
* User management
* Household model
* Optional Google login
* Optional Apple login

Deliverable:

Authenticated users can access the application and remain logged in.

---

# Milestone 3 – Inventory

Goal:
Implement the core inventory system.

Tasks:

* Inventory CRUD
* Categories
* Threshold configuration
* Inventory overview
* Search
* Quantity adjustments
* Inventory history

Deliverable:

Users can manage their household inventory.

---

# Milestone 4 – Shopping List

Goal:
Automatically generate shopping lists.

Tasks:

* Shopping list table
* Threshold logic
* Automatic additions
* Automatic removal after restocking
* Manual edits

Deliverable:

Inventory automatically creates and updates the shopping list.

---

# Milestone 5 – QR System

Goal:
Replace manual inventory updates with QR scanning.

Tasks:

* Generate QR codes
* Printable QR cards
* Share QR codes
* Print support
* QR management

Deliverable:

Every inventory item has printable increase and decrease QR codes.

---

# Milestone 6 – Scanner Integration

Goal:
Connect the dedicated scanner.

Tasks:

* Scanner daemon
* USB HID support
* Event generation
* Local API communication
* Retry handling
* Feedback through LEDs/buzzer/display

Deliverable:

The scanner updates inventory without requiring the smartphone.

---

# Milestone 7 – Mobile Fallback

Goal:
Allow inventory updates without the dedicated scanner.

Tasks:

* Camera scanning
* ZXing integration
* Native share support
* Offline caching
* PWA improvements

Deliverable:

Users can fully operate the system using only their smartphone.

---

# Milestone 8 – Barcode Support

Goal:
Simplify adding purchased products.

Tasks:

* Barcode scanner
* Product mapping
* Open Food Facts integration
* Purchase history
* Price history

Deliverable:

Purchased products can automatically increase inventory.

---

# Milestone 9 – Polish

Goal:
Improve usability.

Tasks:

* Statistics
* Push notifications
* Better search
* Settings
* UI improvements
* Performance improvements

Deliverable:

Version 1.0.
