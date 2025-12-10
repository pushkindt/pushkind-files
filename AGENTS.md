# AGENTS.md

Guidance for AI changes so new code matches the existing architecture.

## Project Context
Rust 2024 Actix Web app using Diesel (SQLite), Tera, and `pushkind-common`. Layers: domain models, repository traits/impls, services, dto, Actix routes, forms, templates. Keep business logic in services; handlers/repos stay thin.

## Development Commands
```bash
cargo build --all-features --verbose
cargo test --all-features --verbose
cargo clippy --all-features --tests -- -Dwarnings
cargo fmt --all -- --check
```

## Coding Standards
- Domain types in `src/domain`; Diesel models in `src/models`; DTOs in `src/dto`; conversions via `From`/`Into`.
- Domain fields are strongly typed (e.g., `ManagerEmail`, `HubId`); construct at boundaries after sanitization; domain structs do no validation/normalization.
- Repos/services return `RepositoryResult<T>`/`ServiceResult<T>` with `thiserror` enums owned by the crate.
- Services convert domain -> DTO for routes; services return DTOs or domain/`()`. Routes handle flash/redirects.
- Validate and sanitize early (forms) using `validator` and `ammonia`; trim/normalize before building domain types.
- Push branching/validation/orchestration into services; prefer DI over globals; document public APIs/breaking changes; avoid `unwrap`/`expect` in production paths.

## HTTP and Templates
- Routes in `src/routes` only extract input, call a service, map to HTTP.
- Use `RedirectSuccess` for service-triggered redirects with flash.
- Render Tera contexts with sanitized data; reuse components in `templates/`.
- Enforce auth via `pushkind_common::routes::check_role` and `SERVICE_ACCESS_ROLE`.

## Testing
- Add tests for new service/form logic; use Diesel migrations and helpers for DB work.
- Use `src/repository/mock.rs` to isolate service tests from Diesel.
- Ensure new functionality is covered before PRs.
