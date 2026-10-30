# Security rules

Security is a product feature, not a later hardening task.

- Default to secure configuration.
- Do not claim that Rust makes an application automatically secure. Rust memory safety reduces classes of memory-safety defects; application-level authorization, injection, CSRF, XSS, SSRF, secrets, and business logic vulnerabilities remain possible.
- Do not write custom cryptography. Use maintained, reviewed crates selected through an approved security review.
- Use parameterized database operations. Never concatenate untrusted input into SQL.
- Validate and authorize every state-changing operation on the server.
- Enforce authorization at the service/use-case boundary, not only in UI or route visibility.
- Use secure cookie attributes (`HttpOnly`, `Secure` in production, appropriate `SameSite`) when cookie sessions are approved.
- Implement CSRF protection for cookie-authenticated browser requests.
- Set explicit CORS allowlists; never use wildcard origins with credentials.
- Hash passwords with a modern password hashing algorithm through a maintained crate; never encrypt or store plaintext passwords.
- Store secrets only in environment/secret managers. Redact them from logs and error responses.
- Set request body, upload, timeout, pagination, and concurrency limits.
- Protect file uploads with content validation, size limits, safe filenames, and storage isolation.
- Prevent path traversal and SSRF in any URL/file fetching features.
- Use TLS in production deployment guidance.
- Apply least privilege to DB users and service credentials.
- Add dependency auditing and vulnerability scanning to CI.
- Security-sensitive architecture requires owner approval before implementation.
- Add negative tests for unauthorized access, invalid input, CSRF, CORS, and data leakage where relevant.
