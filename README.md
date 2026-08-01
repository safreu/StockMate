
# StockMate

---
[![codecov](https://codecov.io/github/safreu/StockMate/graph/badge.svg?token=JKDRX41DDP)](https://codecov.io/github/safreu/StockMate)
---
> A smart household inventory system that automates inventory tracking using QR codes.

StockMate is a self-hosted household inventory system designed to make inventory management almost effortless. Instead of manually updating a shopping list whenever something runs out, every inventory item has one or more printed QR codes. Scanning these codes with a dedicated scanner connected to a Raspberry Pi immediately updates the inventory. When an item's stock falls below a configurable threshold, it is automatically added to the shopping list.

The primary interaction happens through the dedicated scanner. The Progressive Web App (PWA) is mainly used for administration, viewing the inventory, checking the shopping list, generating QR codes, and serving as a fallback scanner if the dedicated hardware is unavailable.

## Features

* Shared household inventory
* Automatic shopping list generation
* Dedicated QR scanner connected to Raspberry Pi
* Mobile PWA as fallback
* User authentication
* QR code generation and printing
* Purchase history
* Barcode support for mapping products
* Undo functionality through immutable inventory events
* Push notifications

## Project Goals

* Minimize user interaction during everyday use.
* Keep inventory changes fast and reliable.
* Be completely self-hostable.
* Work offline within the local network whenever possible.
* Remain extensible for future hardware integrations.

## Technology Stack

### Frontend

* React
* TypeScript
* Vite
* Progressive Web App

### Backend

* Rust
* Axum
* SQLx
* Postgres

### Hardware

* Raspberry Pi
* USB QR Scanner
* Optional display, LEDs and buzzer

## Documentation

* `docs/roadmap.md`
* `docs/project-structure.md`
* `docs/features.md`
* `docs/architecture.md`
