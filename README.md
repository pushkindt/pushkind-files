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

- **Core utilities (`src/lib.rs`)** – Shared helpers for upload paths, file name
  sanitisation, and image classification used across routes and forms.
- **Forms (`src/forms`)** – Structures for multipart uploads and folder creation
  with `validator`-backed input checks.
- **Routes (`src/routes`)** – Actix Web handlers that guard access, list hub
  directories, save uploads, and manage flash messaging before rendering Tera
  templates.
- **Templates (`templates/`)** – Server-rendered UI built with Tera and
  Bootstrap 5, showing flash messages and file system metadata pulled from the
  routes.
- **Server bootstrap (`src/main.rs`)** – Builds the Actix application, applies
  authentication middleware from `pushkind-common`, and mounts static file
  serving for uploads and assets.

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

### Environment

The service reads configuration from environment variables. The most important
ones are:

| Variable | Description | Default |
| --- | --- | --- |
| `AUTH_SERVICE_URL` | Base URL of the Pushkind authentication service | _required_ |
| `SECRET_KEY` | 32-byte secret for signing cookies | generated at runtime |
| `PORT` | HTTP port | `8080` |
| `ADDRESS` | Interface to bind | `127.0.0.1` |
| `DOMAIN` | Cookie domain (without protocol) | `localhost` |

Create a `.env` file if you want these values loaded automatically via
[`dotenvy`](https://crates.io/crates/dotenvy).

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

- **Path safety**: normalise directories and file names, reject traversal
  attempts, and use the utilities in `src/lib.rs` whenever possible.
- **Explicit errors**: bubble up filesystem errors and map them into HTTP
  responses that keep users informed without exposing sensitive paths.
- **No panics in production paths**: avoid `unwrap`/`expect` in request handlers
  and prefer graceful error handling with logging.
- **Security aware**: sanitize user-facing strings, validate form inputs with
  `validator`, and rely on `pushkind-common::routes::ensure_role` to enforce the
  `"files"` service role.
- **Testable**: introduce small, pure functions for new logic so they can be
  covered by unit tests without spinning up Actix.

Following these guidelines will help new functionality slot seamlessly into the
existing architecture and keep the service reliable in production.
