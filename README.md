# pushkind-files

`pushkind-files` is the file management service that powers Pushkind hubs. It
delivers a server-rendered file browser backed by per-hub storage, integrates
with the Pushkind authentication flow, and exposes upload and folder management
capabilities for members who hold the appropriate service role.

## Features

- **Per-hub storage isolation** – Each authenticated hub member works inside a dedicated directory under `./upload/{hub_id}`, guaranteeing users can only browse their own hub's files.
- **Server-rendered file browser** – Folder contents are listed with directory-first sorting, inline image detection, and flash messaging for quick feedback.
- **Secure uploads** – Multipart uploads accept files up to 10 MB, normalise file names, and reject attempts at path traversal before persisting to disk.
- **Folder management** – Users can create nested folders after form validation, keeping the structure tidy without leaving the interface.
- **Pushkind auth integration** – Access is gated by the `"files"` service role using `pushkind-common` helpers, preserving the shared login and authorization experience.

## Architecture at a Glance

The codebase follows a clean, layered structure so that business logic can be
exercised and tested without going through the web framework:

- **Domain (`src/domain`)** – Type-safe models for clients, client events, and
  managers. Builder-style helpers make it easy to construct new payloads while
  capturing timestamps, normalising contact data (phone numbers stored in E164
  format), and sanitising inputs early.
- **Services (`src/services`)** – Application use-cases that orchestrate domain
  logic, repository traits, and Pushkind authentication helpers. Services return
  `ServiceResult<T>` and map infrastructure errors into well-defined service
  errors.
- **DTOs (`src/dto`)** – Data transfer objects for rendering templates and API
  responses. Services convert domain types to DTOs before handing data to routes,
  keeping handlers thin and domain models focused.
- **Forms (`src/forms`)** – `serde`/`validator` powered structs that handle
  request payload validation, CSV parsing, and transformation into domain types.
- **Routes (`src/routes`)** – Actix Web handlers that wire HTTP requests into the
  service layer and render Tera templates or redirect with flash messages.
- **Templates (`templates/`)** – Server-rendered UI built with Tera and
  Bootstrap 5

## Technology Stack

- Rust 2024 edition
- [Actix Web](https://actix.rs/) with identity, session, and flash message
  middleware
- `actix-files` for serving uploaded assets and static resources
- `actix-multipart` for handling file uploads with size limits
- [Tera](https://tera.netlify.app/) templates styled with Bootstrap 5.3
- [`pushkind-common`](https://github.com/pushkindt/pushkind-common) shared crate
  for authentication guards, configuration, and reusable helpers
- Supporting crates: `validator`, `serde`, `uuid`, `env_logger`, and `dotenvy`

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://www.rust-lang.org/tools/install))
- `cargo` available on your `PATH`

### Configuration

Settings are layered via the [`config`](https://crates.io/crates/config) crate in the following order (later entries override earlier ones):

1. `config/default.yaml` (checked in)
2. `config/{APP_ENV}.yaml` where `APP_ENV` defaults to `local`
3. Environment variables prefixed with `APP_` (loaded automatically from a `.env` file via `dotenvy`)

Key settings you may want to override:

| Environment variable | Description | Default |
| --- | --- | --- |
| `APP_SECRET` | 64-byte secret used to sign cookies and flash messages | _required_ |
| `APP_ADDRESS` | Interface to bind | `127.0.0.1` |
| `APP_PORT` | HTTP port | `80` (override to `8080` in local.yaml) |
| `APP_DOMAIN` | Cookie domain (without protocol) | _required_ |
| `APP_TEMPLATES_DIR` | Glob pattern for templates consumed by Tera | `templates/**/*` |
| `APP_AUTH_SERVICE_URL` | URL of the Pushkind authentication service | _required_ |
| `APP_UPLOAD_PATH` | Path to the upload folder | `./upload/` |

Switch to the production profile with `APP_ENV=prod` or provide your own
`config/{env}.yaml`. Environment variables always win over YAML values, so a
local `.env` file containing `APP_SECRET=<64-byte key>` (generate with
`openssl rand -base64 64`) and any overrides will take effect without changing
the checked-in config files.

### Uploads Directory

Uploaded files live under the `./upload/{hub_id}` tree on disk. Ensure the
process has permission to create and write to the `upload` directory before
starting the server. Staging or production deployments should mount persistent
storage at that path so files survive restarts.

## Running the Application

Start the HTTP server with:

```bash
cargo run
```

The server listens on `http://127.0.0.1:8080` by default, serves uploaded files
from `/upload`, and renders the file browser template for authorized users. All
routes are protected by the Pushkind authentication middleware and check that
the signed-in member has the `"files"` service role.

## Quality Gates

The project treats formatting, linting, and tests as required gates before
opening a pull request. Use the following commands locally:

```bash
cargo fmt --all -- --check
cargo clippy --all-features --tests -- -Dwarnings
cargo test --all-features --verbose
cargo build --all-features --verbose
```

Alternatively, the `make check` target will format the codebase, run clippy, and
execute the test suite in one step.

## Testing

Unit tests cover the sanitisation helpers in `src/lib.rs` and any new logic
should expand that suite to guard against path traversal or invalid input. Add
Actix handler tests when behaviour diverges from the happy paths exercised
manually.

## Project Principles

- **Domain-driven**: keep business rules in the domain and service layers and
  translate to/from external representations at the boundaries.
- **Explicit errors**: use `thiserror` to define granular error types and convert
  them into `ServiceError`/`RepositoryError` variants instead of relying on
  `anyhow`.
- **No panics in production paths**: avoid `unwrap`/`expect` in request handlers,
  services, and repositories—propagate errors instead.
- **Path safety**: normalise directories and file names, reject traversal
  attempts, and use the utilities in `src/lib.rs` whenever possible.
- **Security aware**: sanitize user-facing strings, validate form inputs with
  `validator`, and rely on `pushkind-common::routes::ensure_role` to enforce the
  `"files"` service role.
- **Testable**: introduce small, pure functions for new logic so they can be
  covered by unit tests without spinning up Actix.

Following these guidelines will help new functionality slot seamlessly into the
existing architecture and keep the service reliable in production.
