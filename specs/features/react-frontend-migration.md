# React Frontend Migration Preserving Existing File Browser UI

## Status
Stable

## Summary
Migrate the current Tera-based `pushkind-files` frontend to React-managed UI
components while preserving the existing Bootstrap styling, route structure,
copy, embedded browser contract, and backend-owned file management rules. This
work MUST align with a Vite-owned static frontend document for the top-level
page, specific client data APIs under `/api/v1/`, and form-layer ownership of
validation copy. The result MUST remain server-routed and MUST NOT turn
`pushkind-files` into a SPA.

## Problem
The current frontend is split across Tera templates, inline styles, and a large
imperative `assets/filebrowser.js` script that fetches HTML fragments, rewires
DOM after every update, and manually manages history, uploads, folder creation,
and clipboard interactions. That keeps the service small, but it makes the UI
harder to test, evolve, and reuse consistently across the full page and
embedded browser contexts.

## Goals
- Introduce React as the component model for the `pushkind-files` UI.
- Preserve the current visual design, Bootstrap classes, routes, and user
  visible Russian copy.
- Preserve current server-side authorization, path validation, upload rules,
  and storage semantics.
- Replace Tera-owned file browser markup and imperative `filebrowser.js`
  behavior with typed React components and frontend-owned entry documents.
- Keep support for embedding the file browser into other Pushkind pages.
- Replace page-shaped bootstrap transport with narrower resource-style client
  data APIs where practical.
- Make validation and mutation error handling explicit through structured JSON
  responses for React-owned interactions.

## Non-Goals
- Introducing client-side routing or SPA navigation.
- Redesigning the file browser or replacing Bootstrap with another design
  system.
- Moving authorization, file validation, or filesystem rules into the client.
- Adding new file operations such as delete, rename, move, tagging, or search.
- Changing upload limits or broadening file access semantics.

## Current Baseline
The current frontend surface is implemented in Tera templates:
- `templates/base.html`
- `templates/components/navigation.html`
- `templates/components/file_browser.html`
- `templates/components/file.html`
- `templates/components/folder.html`
- `templates/main/index.html`
- `templates/main/new_folder_modal.html`

Current client behavior is owned by:
- Bootstrap JS for dropdowns, alerts, tooltips, popovers, and modal helpers.
- Inline JavaScript in `templates/base.html` and `templates/main/index.html`.
- Imperative DOM and fetch logic in `assets/filebrowser.js`.

Current interactive behavior includes:
- Directory navigation with `path` query synchronization and `popstate`
  handling on the main page.
- HTML fragment fetching from `GET /files/browser`.
- Drag-and-drop and file-input uploads to `POST /files/upload`.
- Inline folder creation via `POST /folder/create`.
- Clipboard copy for file URLs with a fallback copy mechanism.
- Base-URL rewriting so the browser can be embedded in other same-origin
  surfaces.

## In Scope
- The authenticated index page at `GET /`.
- The embeddable file browser exposed by `GET /files/browser`.
- Shared shell concerns currently handled in `templates/base.html`, including
  flash messages and Bootstrap lifecycle wiring.
- File browser interactions: breadcrumb navigation, folder cards, file cards,
  image previews, copy-link actions, uploads, upload status UI, and new-folder
  form behavior.
- Vite-built static frontend documents for full-page React-owned surfaces.
- Typed client data APIs for current-user context and directory data.
- Structured JSON responses for React-owned mutation flows.
- Frontend asset build and delivery needed to run React in production and local
  development.

## Out Of Scope
- Changes to file storage layout under `upload/{hub_id}`.
- Changes to backend role semantics or authentication/session flow.
- Changes to upload size limits or multipart handling beyond what React needs to
  integrate with current endpoints.
- Replacing file serving from `/upload/*`.

## Functional Requirements

### 1. Rendering Model
- The application MUST keep the existing server-owned route model.
- The application MUST NOT introduce client-side routing for `/` or
  `/files/browser`.
- React MUST be introduced as page-level or widget-level components mounted on
  the existing URLs.
- The long-term target MUST be:
  Vite-owned static HTML for `GET /`,
  React-owned page markup for the full files page,
  and a React-owned embeddable file-browser widget for `/files/browser`.
- During migration, Tera MAY remain only until the React/Vite implementation is
  ready for cutover, but the target state MUST eliminate Rust-owned frontend
  document rendering for the top-level page.

### 2. Frontend Document Ownership
- The browser HTML document for the top-level files page MUST be authored in
  the frontend workspace and built by Vite.
- Rust MUST continue to own route access checks before serving that built HTML
  document.
- Page initialization data MUST NOT be embedded into server-generated HTML in
  the target state.
- The embedded browser route is not required to become a full standalone HTML
  document; it MAY remain a widget-oriented route so long as its markup and
  behavior are React-owned.

### 3. Embedded Browser Compatibility
- The embeddable file browser contract MUST remain supported.
- Consumers embedding the browser MUST still be able to mount it into an
  existing DOM node with a configurable `baseUrl`.
- Top-level page usage MAY manage browser history; embedded usage MUST NOT
  mutate host-page history.
- The embedded browser MUST continue to resolve file URLs, fetch endpoints, and
  navigation targets relative to the configured base URL.

### 4. Markup And Style Preservation
- Migrated React components MUST preserve the current Bootstrap-based visual
  hierarchy and utility classes unless a deviation is explicitly documented.
- The main page MUST continue to show the same navbar, breadcrumb area, upload
  panel, folder button, and entry-card layout.
- User-visible Russian copy SHOULD remain unchanged except for bug fixes or
  accessibility improvements.
- Existing Bootstrap Icons usage, image previews, and flash presentation MUST
  continue to work.

### 5. Behavioral Parity
- `GET /` MUST continue to render the authenticated files page with the current
  file browser surface and `path` handling.
- `GET /files/browser` MUST continue to expose the browser UI for embedded or
  partial-page use.
- Folder navigation MUST continue to support the current breadcrumb and card
  interactions.
- Main-page navigation MUST continue to synchronize the `path` query parameter
  and browser back/forward behavior.
- Drag-and-drop uploads and file-input uploads MUST continue to work.
- Upload status UI MUST continue to provide per-file feedback and refresh the
  directory contents when uploads finish.
- Inline folder creation MUST continue to validate and display backend errors.
- File cards MUST continue to support image previews, downloads, and copy-link
  actions.

### 6. Client Data API Model
- React-owned page initialization MUST prefer specific client data APIs over
  page-shaped bootstrap endpoints.
- New GET endpoints introduced for this migration MUST be versioned under
  `/api/v1/`.
- The initial reusable GET surface SHOULD include:
  current-user/session context needed by the files UI,
  and directory listing data for a requested path.
- Directory data SHOULD be exposed as a resource endpoint such as
  `GET /api/v1/files/entries?path=...` rather than as HTML fragments.
- GET APIs MUST NOT be used to expose transient alert messages.

### 7. Backend Boundary
- Authorization, path sanitization, upload persistence, and folder creation
  rules MUST remain in Rust service code.
- Routes MUST expose typed DTO or page-model payloads to React instead of
  relying on handwritten DOM parsing.
- Interactions currently driven by HTML fragment fetches MUST move to typed
  JSON or equivalent structured responses before the migration is considered
  complete.

### 8. Mutation And Validation Semantics
- React-owned mutation flows SHOULD use structured JSON success/error responses
  instead of ad hoc text bodies or flash-message-driven reload behavior.
- Existing endpoints MAY be reused for those JSON responses if their route
  ownership and authorization semantics remain correct.
- Validation errors MUST be field-addressable so the frontend can render them
  inline.
- Field-level validation copy MUST be owned by `src/forms`, not by route-level
  matching code.
- Redirect-driven flows MAY remain only where the interaction has not yet moved
  under React ownership.

### 9. Progressive Enhancement
- Direct page loads for `/` and `/files/browser` MUST remain server-routed.
- File download links MUST remain normal links to `/upload/*`.
- Uploads and folder creation MAY remain asynchronous client-side interactions,
  but backend validation and status semantics MUST stay authoritative.

### 10. Frontend Tooling
- The repository MUST gain a supported frontend toolchain for React and
  TypeScript source code.
- Production builds MUST emit versioned static assets and any required static
  HTML documents that can be served from the existing `/assets` path.
- The server MUST serve the compiled frontend assets directly.
- Local development MUST support efficient frontend iteration without manual
  asset copying.

## Data Requirements
- React pages and widgets MUST initialize from typed DTO contracts.
- The target state SHOULD avoid one-off page bootstrap endpoints when the same
  result can be composed from reusable client data APIs.
- The top-level page SHOULD be able to initialize from a combination of:
  current-user/session context,
  and directory listing data for the selected path.
- The embedded browser SHOULD be able to initialize from directory data plus
  explicit frontend configuration such as `baseUrl` and history behavior.
- DTO structs MUST live under `src/dto/` or another backend-facing UI model
  module and MUST NOT expose raw domain internals unnecessarily.

## Migration Requirements
- The migration MUST be incremental.
- A shared React file browser component SHOULD be introduced first, then reused
  by the main page and the embedded browser shell.
- The migration SHOULD converge on:
  Vite-built static HTML for the full page,
  specific `/api/v1/...` client data APIs,
  and form-owned validation messages.
- Existing Tera templates MAY remain as compatibility wrappers during rollout,
  but duplicated file browser markup SHOULD be removed once React owns it.
- `assets/filebrowser.js` and related inline scripts SHOULD be removed only
  after equivalent React behavior is verified.
- Playwright screenshot baselines SHOULD exist before the corresponding Tera
  implementation is removed.

## Acceptance Criteria
- Same URLs continue to serve the main files page, embedded browser, uploads,
  folder creation, assets, and file downloads.
- The visual appearance remains substantially unchanged for the navbar,
  breadcrumb, upload area, folder panel, folder cards, and file cards.
- The embedded browser still works with a configurable `baseUrl`.
- `GET /` is served from a Vite-built static frontend document after backend
  access checks.
- React page data comes from typed client data APIs rather than HTML-embedded
  bootstrap payloads.
- Browser navigation, uploads, copy-link actions, and folder creation preserve
  current behavior.
- No authorization or path-safety rules move to the client.
- The React frontend builds reproducibly and its assets are served by the
  application runtime.
- Regression coverage exists for backend page-data contracts and critical
  frontend behavior.

## Risks
- Initial render of the top-level page depends on client data fetches after the
  static HTML document loads.
- Embedded-browser compatibility can regress if base-URL handling changes.
- Upload progress and refresh behavior can drift during the move away from
  imperative DOM code.
