# pushkind-files Specification

## Purpose
`pushkind-files` is the Pushkind hub file management service. It provides
authenticated hub members with a hub-scoped file browser, upload workflows, and
folder creation backed by on-disk storage.

## Goals
- Provide per-hub file storage isolation.
- Keep authorization, path validation, and storage rules in Rust service code.
- Preserve a simple server-routed UI model for the main files page.
- Support embedding the file browser in other Pushkind surfaces.
- Expose predictable DTOs for UI rendering and future frontend migration work.

## Non-Goals
- Cross-hub file sharing.
- Object storage backends or multi-storage support.
- Client-side authorization or path validation.
- File deletion, rename, move, or tagging workflows.
- SPA navigation.

## Architecture Overview
- **Routes (`src/routes`)**: Actix handlers extract inputs, call services, and
  return HTTP responses.
- **Services (`src/services`)**: Own file-system use-cases, authorization,
  sanitized path handling, and DTO creation.
- **Domain (`src/domain`)**: Strongly typed path and storage primitives such as
  `UploadRoot`, `HubStorage`, `RelativePath`, and `FileName`.
- **Forms (`src/forms`)**: Request payload structs for multipart uploads and
  folder creation validation.
- **DTOs (`src/dto`)**: Serializable shapes for UI rendering.
- **Static assets (`assets/`)**: Browser-side JavaScript and images served by
  the application.

## Runtime Components
- Actix Web server configured in `src/lib.rs::run`.
- Cookie-backed sessions and identity.
- Shared Pushkind authentication and redirect helpers from `pushkind-common`.
- On-disk storage rooted at `ServerConfig.upload_path`.
- React/Vite frontend build emitted into `assets/dist/`.

## HTTP Routes
All authenticated routes are mounted under the root scope and use
`RedirectUnauthorized`.

| Method | Path | Description |
| --- | --- | --- |
| GET | `/` | Serve the main React file browser page for the authenticated hub member. |
| GET | `/files/browser` | Serve the embeddable React browser document for an optional `path`. |
| POST | `/files/upload` | Upload a single file into the current hub directory. |
| POST | `/folder/create` | Create a folder under the current hub directory. |
| POST | `/logout` | Logout via shared `pushkind_common` route. |
| GET | `/na` | Local not-assigned page for authenticated users without the service role. |
| GET | `/upload/*` | Serve uploaded files from the configured upload root. |
| GET | `/assets/*` | Serve static assets from the repository `assets/` directory. |

## Authentication and Authorization
- All application routes except static assets rely on shared Pushkind
  authentication middleware.
- File access requires the fixed `SERVICE_ACCESS_ROLE` value `"files"`.
- Authorization checks happen in `FileService::authorize`.
- Unauthorized authenticated users are redirected to `/na` for HTML routes.
- Upload and folder creation return `401` when the authenticated user lacks the
  required role.

## Core Flows

### Directory Listing
1. Authenticate the user and require the `"files"` role.
2. Sanitize the optional `path` query into a `RelativePath`.
3. Ensure the hub root directory exists.
4. Read the target directory, drop invalid file names, and classify entries as
   folders or files.
5. Sort folders before files; sort folders alphabetically and files by creation
   time descending with name fallback.
6. Convert entries to `FileEntryDto` values for rendering.

### File Upload
1. Authenticate the user and require the `"files"` role.
2. Sanitize the optional target path and uploaded file name.
3. Ensure the target directory exists.
4. Persist the uploaded temporary file into the hub storage directory.

### Folder Creation
1. Validate `CreateFolderForm`.
2. Authenticate the user and require the `"files"` role.
3. Sanitize the current path and requested folder name.
4. Create the directory tree under the hub storage root.

## UI Surface
- `assets/dist/index.html` renders the authenticated full-page files document
  after Rust access checks succeed.
- `assets/dist/browser.html` renders the embeddable browser document exposed by
  `GET /files/browser`.
- The shared React browser owns directory navigation, drag-and-drop uploads,
  upload progress feedback, folder creation, clipboard copying, and optional
  history handling.
- The current UI remains Bootstrap-based and intentionally lightweight.

## React Client Data APIs
- React-owned files UI MUST initialize from resource-style `/api/v1/...` JSON
  endpoints rather than page-shaped bootstrap transport.
- The active reusable GET surface is:
  - `GET /api/v1/iam`
  - `GET /api/v1/files/entries?path=...`
  - `GET /api/v1/no-access`
- Directory data MUST remain exposed as a file-entry collection resource, not
  as HTML fragments or page-named bootstrap payloads.

## Configuration
- Config is loaded from `config/default.yaml`, then `config/{APP_ENV}.yaml`,
  then `APP_` environment variables.
- Required runtime fields: `domain`, `address`, `port`, `auth_service_url`,
  `secret`, and `upload_path`.
- Startup logs an error and exits if configuration is missing or invalid.

## Error Handling
- Services return `ServiceResult<T>` with crate-owned `ServiceError` variants.
- Invalid paths and invalid file names map to `400 Bad Request`.
- Unauthorized API-style mutations map to `401 Unauthorized`.
- Unexpected storage or rendering failures map to `500 Internal Server Error`
  and are logged.
- Production request paths MUST NOT rely on `unwrap` or `expect`.

## Data Model (High Level)
- **UploadRoot**: absolute root path that contains all hub storage.
- **HubStorage**: hub-scoped view over the upload root.
- **RelativePath**: sanitized directory path relative to a hub root.
- **FileName**: sanitized single path component.
- **StorageEntry**: directory or file entry derived from the filesystem.
- **FileEntryDto**: serialized UI representation of a storage entry.

## Invariants
- A user can only operate inside their own hub's storage root.
- Paths containing parent traversal components are invalid.
- File names must be a single path component.
- Missing directories are treated as empty listings.
- Directory creation may create intermediate folders.
- Uploaded files are served from `/upload/{hub_id}/...`.

## External Integrations
- **pushkind-common**: shared auth middleware, current user extraction,
  template helpers, logout route, and not-assigned page.
- **Filesystem**: persistent storage for uploaded content.

## Operational Notes
- Session cookies are scoped to `.{domain}`.
- `cookie_secure` is `false` in code and must be `true` in production.
- The upload directory must be writable by the service process.
