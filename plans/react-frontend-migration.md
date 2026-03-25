# Plan: React Frontend Migration

## References
- Service baseline:
  [../SPEC.md](../SPEC.md)
- Feature spec:
  [../specs/features/react-frontend-migration.md](../specs/features/react-frontend-migration.md)

## Objective
Introduce React for the `pushkind-files` frontend while preserving the current
UI structure, styling, routes, embedded browser contract, and backend-owned
authorization and storage logic. The migration remains server-routed,
non-SPA, and converges on:
Vite-built static HTML for `GET /`,
specific `/api/v1/...` client data APIs,
form-owned validation copy,
and removal of Tera-owned file-browser markup and imperative browser scripts
from runtime paths.

## Fixed Implementation Decisions
- Frontend source code WILL live in `frontend/`.
- Production frontend build output WILL live in `assets/dist/`.
- The React toolchain WILL use `npm`, React, TypeScript, and Vite.
- The backend WILL continue to own routing, authentication, authorization, path
  validation, redirects, and storage persistence.
- The application server WILL continue to serve compiled frontend assets from
  the existing `/assets` path.
- Vite WILL own the top-level HTML document served for `GET /`.
- The migration WILL preserve an embeddable file-browser mount API with
  configurable `baseUrl`.
- React page initialization WILL fetch typed JSON data from backend endpoints;
  page data WILL NOT remain embedded into server-generated HTML in the target
  state.
- New GET endpoints introduced for React-owned page data WILL be versioned
  under `/api/v1/`.
- HTML fragment fetching from `/files/browser` WILL be removed and replaced by
  structured JSON data loading.
- Validation copy for React-owned forms WILL live in `src/forms`.
- Tera WILL be used only as a temporary migration wrapper and WILL be removable
  from the top-level page once the full-page route is React-backed.
- Visual parity in CI SHOULD use Playwright screenshot comparisons for the main
  page and embedded browser states.

## Repository Layout
The implementation SHOULD create and use the following structure:

```text
frontend/
  package.json
  package-lock.json
  tsconfig.json
  vite.config.ts
  src/
    entries/
    components/
    pages/
    styles/
    lib/
assets/
  dist/
src/
  dto/
  routes/
  services/
templates/
```

Directory intent:
- `frontend/src/entries/`:
  entrypoints for the full page and for the embeddable browser bundle.
- `frontend/src/components/`:
  reusable file-browser, breadcrumb, entry-card, flash, and upload UI
  components.
- `frontend/src/pages/`:
  page-level React components for `/` and any standalone embedded shell.
- `frontend/src/lib/`:
  payload readers, endpoint builders, clipboard helpers, history integration,
  and Bootstrap lifecycle adapters.
- `frontend/src/styles/`:
  CSS imports that preserve the current Bootstrap-based output.
- `assets/dist/`:
  compiled JavaScript, CSS, and manifest output.

## Toolchain And Build Outputs

### Frontend Package Management
- Use `npm` as the package manager.
- Commit `frontend/package-lock.json`.
- Do not introduce `pnpm`, `yarn`, or an alternative JavaScript runtime.

### Build Tool
- Use Vite to build the React frontend.
- Configure Vite to emit compiled assets into `assets/dist/`.
- Configure Vite to emit a manifest file at `assets/dist/manifest.json`.
- Configure explicit entrypoints for:
  the top-level files page HTML document,
  the embeddable file-browser bundle.

### Required `package.json` Scripts
The frontend package MUST expose at least these scripts:
- `dev`
- `build`
- `preview`
- `test`
- `lint`
- `typecheck`
- `playwright:screenshots`

### Source Control Hygiene
- Add `frontend/node_modules/` to `.gitignore`.
- Add `assets/dist/` to `.gitignore` unless deployment later requires committed
  build artifacts.

## Backend Integration

### Asset Serving
- Keep Actix static serving for `/assets` and ensure it covers `assets/dist/`.

### Built HTML Serving
- Add a backend helper that serves the built Vite HTML entry for `GET /` after
  authentication and authorization checks.
- Rust MUST stop assembling the full-page HTML document at request time once
  the migration is complete.

### Asset Manifest Loading
- Add a backend helper that reads `assets/dist/manifest.json` and resolves JS
  and CSS assets for each frontend entrypoint.
- The helper MUST fail clearly when a required manifest entry is missing.

### Client Data APIs
- Add typed DTOs under `src/dto/` for the reusable client data APIs.
- The initial GET DTO set SHOULD cover:
  current-user/session context for the files shell,
  directory listings for a sanitized path,
  and entry DTOs carrying navigation/download/copy-link data.
- Prefer specific resource endpoints such as:
  `GET /api/v1/iam`,
  `GET /api/v1/files/entries?path=...`
  over page-specific bootstrap endpoints.
- Do not expose raw domain types directly to the frontend.

### Structured Browser Data
- Introduce a typed structured response for browser refresh/navigation data so
  React no longer has to fetch HTML fragments to update directory contents.
- Keep `/files/browser` as a React-owned embedded browser route with the same
  mount contract and base-URL behavior.

### Server-Rendered Shell During Migration
- During migration, the backend MAY render a minimal HTML shell that:
  includes the React entrypoint,
  includes compiled CSS,
  provides the mount node for React.
- Any such shell is transitional only. The target state for `GET /` is a
  Vite-built static HTML document, not a Rust-rendered shell.

## Frontend Runtime Requirements

### Shared Browser Component
- Implement a shared React `FileBrowser` component used by both `/` and the
  embedded browser integration.
- The component MUST support:
  breadcrumb navigation,
  folder cards,
  file cards,
  image previews,
  copy-link behavior,
  upload progress UI,
  new-folder form toggling,
  backend error display,
  optional history management.

### Embedded Mount API
- Expose a browser-side mount API compatible with embedding into an existing DOM
  node.
- The API SHOULD preserve the current shape of
  `window.mountFileBrowser(target, initialPath, { baseUrl })` or provide a thin
  compatibility wrapper during rollout.

### Bootstrap Integration
- Keep Bootstrap CSS and Bootstrap Icons in the rendered output.
- Preserve Bootstrap JS behavior for dropdowns, alerts, tooltips, and popovers.
- Move inline Bootstrap lifecycle code into React-safe helpers under
  `frontend/src/lib/`.

### Data Loading
- The full-page React entry MUST fetch typed JSON data after the static HTML
  document loads.
- The frontend SHOULD use a shared API client that can compose page state from
  narrower resource endpoints.
- React MUST render explicit loading and fatal error states while required data
  is in flight.

### Form And Action Handling
- File downloads remain normal links to `/upload/*`.
- Upload and folder creation interactions SHOULD use structured JSON
  request/response handling from React.
- Existing routes MAY be reused for those JSON responses if changing the URL is
  unnecessary.
- Field-level validation errors MUST be returned in a stable, addressable shape.
- Validation copy MUST come from `src/forms`, not route-level message mapping.
- Directory navigation SHOULD reload data through typed structured responses,
  not server-rendered HTML fragments.

## Migration Sequence

### Phase 1: Foundation
Deliverables:
- `frontend/` directory with React, TypeScript, and Vite configured.
- Build output emitted to `assets/dist/`.
- Backend manifest loader and helpers for serving built HTML.
- Developer documentation for installing Node and building frontend assets.

Exit criteria:
- `npm run build` succeeds.
- The server can serve one Vite-built frontend document and load its compiled
  assets.

### Phase 2: Static Full-Page Document
Deliverables:
- Vite-managed HTML entry file for `GET /`.
- Backend route helper that serves the built HTML file after auth checks.
- Initial React app entry for the files page.
- Loading and fatal error states for client data fetches.

Exit criteria:
- `GET /` can be served from a Vite-built HTML document without Rust document
  assembly.

### Phase 3: Shared Shell And Browser Infrastructure
Deliverables:
- Shared React shell for flash messages and layout wiring.
- React-safe Bootstrap helper layer.
- Initial shared `FileBrowser` component and endpoint builder utilities.

Exit criteria:
- Shared browser UI can render from typed data without relying on Tera entry
  markup.

### Phase 4: Client Data APIs
Deliverables:
- Typed current-user/session context endpoint for shell rendering.
- Typed directory-listing response endpoint under `/api/v1/`.
- React-driven directory refresh and navigation logic.
- Main-page history synchronization equivalent to the current `path` and
  `popstate` behavior.
- Embedded mode with history disabled and `baseUrl` handling preserved.

Exit criteria:
- React can initialize and navigate directories without page-shaped bootstrap
  endpoints or HTML fragment fetches.

### Phase 5: Upload And Folder Actions
Deliverables:
- React upload zone with drag-and-drop and file-input support.
- Per-file upload status UI.
- React-controlled new-folder panel and submission flow.
- Structured JSON error handling for upload and folder creation responses.
- Field-addressable validation DTOs for form errors.
- Form-layer ownership of create-folder validation messages.

Exit criteria:
- Uploads and folder creation work end-to-end with behavior matching the
  current browser.

### Phase 6: Main Page And Embedded Browser Rollout
Deliverables:
- React-backed `/` page using the shared browser component.
- React-backed `/files/browser` route for embedded usage.
- Preserved copy-link behavior, file previews, and download links.
- Playwright screenshot baselines for main page and embedded browser states.

Exit criteria:
- Both the full page and the embedded browser render through React with visual
  parity and working interactions.

### Phase 7: Legacy Frontend Removal
Deliverables:
- Remove obsolete Tera file-browser fragments and page-specific inline scripts.
- Remove `assets/filebrowser.js` once compatibility is provided by the React
  bundle.
- Remove any temporary page bootstrap transport kept during migration.
- Trim unused template partials and wiring left over from the Tera browser
  implementation.

Exit criteria:
- No runtime path depends on imperative `filebrowser.js`, page-shaped bootstrap
  transport, or Tera-owned file-browser markup.

## Verification Strategy
- Add backend tests for built-HTML route selection, client data DTOs, and new
  structured directory-data responses.
- Add frontend unit tests for payload parsing, path/history helpers, and
  clipboard/base-URL utilities.
- Add Playwright coverage for:
  main page initial render,
  directory navigation,
  upload state UI,
  folder creation flow,
  embedded browser rendering.
