
# Project Structure

The project is organized as a monorepository because all components belong to one application and are deployed together.

```text
shopping-list/

frontend/
backend/
scanner/
deployment/
docs/
```

---

## Frontend

The frontend is a Progressive Web App.

Responsibilities:

* User interface
* Authentication
* Inventory overview
* Shopping list
* QR generation
* Mobile QR scanning
* Settings
* Sharing and printing

The frontend should **not** contain business logic. All inventory changes are performed by the backend.

---

## Backend

The backend contains the complete business logic.

Responsibilities:

* Authentication
* Session management
* Inventory
* Shopping list
* QR actions
* Barcode mapping
* Database access
* Notifications

The backend is the single source of truth.

---

## Scanner

The scanner is a standalone Rust service running on the Raspberry Pi.

Responsibilities:

* Read scanner input
* Generate scan events
* Send inventory requests
* Retry failed requests
* Trigger hardware feedback

Separating the scanner from the backend keeps hardware-specific code isolated and allows multiple scanners to be supported in the future.

---

## Deployment

Contains deployment-specific files.

Examples:

* systemd services
* Caddy configuration
* Docker configuration (optional)
* deployment scripts

---

## Documentation

All project documentation is stored under `docs/`.

Keeping documentation separate from the README allows each topic to be explained in much greater detail while keeping the repository easy to navigate.
