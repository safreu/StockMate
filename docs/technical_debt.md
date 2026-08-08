# Technical Debt

## Security

- Add rate limiting to the login endpoint.
- Use a dummy Argon2 verification path for unknown users to reduce timing-based account enumeration.
- Centralize and explicitly configure Argon2 parameters instead of relying on `Argon2::default()`.

## Sessions

- Add cleanup for expired sessions, either periodically or during authentication.

## Persistence

- Review SQLx error classification so connection, TLS, protocol, and network failures consistently map to repository unavailability.
- Standardize application timestamp precision (PostgreSQL TIMESTAMPTZ uses microsecond precision), ideally as part of the future Clock abstraction.

## Testing

- Add application-service tests for repository failure mappings where currently missing.

## Documentation

- Add Rustdoc documentation for public ports and clearly document their behavior and error contracts.


## Axum

- Axum Path<Uuid> extractor rejects malformed uuids, replace it with custom extractor to handle behavior explicitly.