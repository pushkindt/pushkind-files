# Tasks: React Frontend Migration Phase 4

## Scope
This task file covers only Phase 4 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- typed client data APIs for the files frontend
- React-driven directory loading and refresh logic
- main-page history synchronization for `path`
- embedded-mode data loading with history disabled and `baseUrl` preserved

Do not start Phase 5 or later phases in this file. Phase 4 is complete only
when React can initialize and navigate the files browser from typed JSON data
instead of HTML fragment fetches, while uploads, folder creation, and live
embedded-route rollout remain on later phases.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 3 is already complete:
  a shared React shell exists,
  a shared `FileBrowser` component exists,
  and shared browser URL/mount utilities are in place.
- Keep authorization, path sanitization, and storage rules in Rust service
  code.
- Do not convert upload or folder-creation responses to structured JSON in this
  phase.

## Deliverables
- A typed current-user/session context endpoint exists under `/api/v1/`.
- A typed directory-listing endpoint exists under `/api/v1/`.
- The full-page React entry loads typed JSON data after the static document
  loads.
- The React browser supports directory navigation and refresh without fetching
  HTML fragments.
- Main-page `path` and browser back/forward behavior are preserved.
- Embedded-mode data loading supports `baseUrl` and history-disabled behavior.

## Task 1: Add Typed Client Data DTOs
Goal:
define the stable backend/frontend contract for shell context and directory
listing data.

Steps:
1. Add DTOs under `src/dto/` for the initial reusable client data APIs.
2. Cover at least:
   current-user/session context needed by the files shell,
   directory listing data for a sanitized path,
   and entry DTOs with the data the React browser needs for navigation,
   download links, previews, and copy-link actions.
3. Keep DTOs frontend-oriented and avoid exposing raw domain types directly.
4. Reuse existing service-layer data where practical, but keep conversion logic
   explicit.
5. Make DTO field names stable and convenient for TypeScript consumption.

Constraints:
- Do not expose flash messages through these GET APIs.
- Do not expose unsanitized filesystem paths.
- Do not leak internal-only domain details to the frontend.

Acceptance checks:
- DTOs exist for shell context and directory listing data.
- The DTO shape is sufficient for the shared React browser to render from API
  data.
- DTO conversion stays outside the route layer where appropriate.

## Task 2: Add `GET /api/v1/iam`
Goal:
provide typed current-user/session context for the React shell.

Steps:
1. Add a versioned route under `/api/v1/iam`.
2. Require the same authentication and `"files"` access semantics as the rest
   of the files UI.
3. Return the current-user/session context needed by the shell and file-browser
   page.
4. Keep the response shape intentionally narrow and reusable.
5. Add tests covering the happy path and authorization behavior.

Implementation notes:
- This endpoint is for shell/page initialization data, not transient UI
  messages.
- The endpoint should be usable by both the top-level page and future embedded
  browser initialization where relevant.

Acceptance checks:
- `GET /api/v1/iam` exists and returns typed JSON.
- The route enforces the same access rules as the files UI.
- Tests cover success and unauthorized behavior.

## Task 3: Add `GET /api/v1/files/entries?path=...`
Goal:
replace HTML fragment transport for directory data with a typed resource
endpoint.

Steps:
1. Add a versioned route under `/api/v1/files/entries`.
2. Accept an optional `path` query parameter.
3. Sanitize and validate `path` using the same Rust rules as the current file
   browser behavior.
4. Reuse the existing file service where possible to list entries.
5. Return typed JSON shaped for the shared React `FileBrowser` component.
6. Add tests for:
   root listing,
   nested-path listing,
   invalid path handling,
   and authorization behavior.

Constraints:
- Do not return HTML from this endpoint.
- Do not change upload or folder-creation routes here.
- Keep missing-directory behavior aligned with the current service semantics.

Acceptance checks:
- `GET /api/v1/files/entries?path=...` exists and returns typed JSON.
- The route preserves current path-validation and auth behavior.
- Tests cover success and invalid/unauthorized cases.

## Task 4: Add A Shared Frontend API Client
Goal:
make the frontend load shell and directory data from typed resource endpoints
through one reusable client layer.

Steps:
1. Add frontend API client helpers under `frontend/src/lib/`.
2. Implement typed fetch helpers for:
   `GET /api/v1/iam`
   and `GET /api/v1/files/entries?path=...`.
3. Parse and validate API responses at the frontend boundary.
4. Surface predictable error states for:
   unauthorized responses,
   invalid-path responses,
   and unexpected server failures.
5. Keep API client code reusable by both the top-level page and embedded mode.

Constraints:
- Do not reintroduce HTML fragment fetching from `/files/browser`.
- Do not hardcode document-specific assumptions into the API client.
- Do not add mutation JSON handling yet.

Acceptance checks:
- There is one shared frontend client layer for the initial GET APIs.
- The shared page/browser code can consume typed API results.
- Error handling is explicit and testable.

## Task 5: Switch The Full-Page React Entry To Typed Data Loading
Goal:
make the top-level files page initialize from the new JSON APIs instead of the
legacy HTML fragment runtime.

Steps:
1. Update the full-page bootstrap flow to fetch shell context and directory data
   after the static document loads.
2. Replace the legacy page-level browser mount with the shared React
   `FileBrowser` component driven by API data.
3. Preserve explicit loading and fatal error states while required data is in
   flight.
4. Keep normal file download links pointing at `/upload/*`.
5. Make sure the full-page route remains server-routed even though the browser
   body is now React/data-driven.

Constraints:
- Do not cut `/files/browser` over to React in this phase.
- Do not remove `assets/filebrowser.js` yet if it is still needed by the legacy
  embedded/browser path.
- Do not add client-side routing.

Acceptance checks:
- The top-level page no longer depends on HTML fragment fetches for directory
  navigation.
- The page renders from typed API data through the shared React browser.
- Loading and fatal states remain explicit in the UI.

## Task 6: Implement Main-Page History Synchronization
Goal:
preserve the current `path` query parameter and browser back/forward behavior
for the full-page React browser.

Steps:
1. Add shared history/path helpers under `frontend/src/lib/`.
2. Initialize page state from the current `path` query parameter.
3. Update the URL when the user navigates folders on the main page.
4. Handle `popstate` so browser back/forward reloads the correct directory
   through the typed API flow.
5. Add frontend tests for path parsing, URL updates, and history-disabled mode
   behavior.

Constraints:
- Keep history mutation disabled when the browser runs in embedded mode.
- Do not introduce SPA route ownership for `/` or `/files/browser`.

Acceptance checks:
- Main-page folder navigation keeps `path` in sync with the URL.
- Back/forward navigation works through the React/data-driven browser.
- The history logic is reusable and test-covered.

## Task 7: Wire Embedded-Mode Data Loading Without Route Cutover
Goal:
prepare the future embedded React browser to load typed data with `baseUrl`
preserved and host-page history untouched.

Steps:
1. Update the future-facing React mount surface to load directory data through
   the new API client.
2. Preserve `baseUrl` handling for embedded usage.
3. Disable browser-history mutation when mounted in embedded mode.
4. Ensure browser URLs, file-download links, and copy-link URLs still resolve
   relative to `baseUrl`.
5. Keep the live `/files/browser` route on its current runtime for now if full
   rollout is deferred to a later phase.

Constraints:
- Do not change the live `/files/browser` route to React here unless that
  happens purely as an internal compatibility wrapper without changing the
  public contract.
- Do not break same-origin embedding.

Acceptance checks:
- The React browser mount layer can load typed directory data in embedded mode.
- Embedded mode does not mutate host-page history.
- `baseUrl` semantics remain preserved.

## Task 8: Keep Mutation Flows Deferred
Goal:
complete the data-loading migration without partially redesigning upload and
folder-creation mutations.

Steps:
1. Keep uploads on the current endpoint and behavior in this phase.
2. Keep folder creation on the current endpoint and behavior in this phase.
3. If the full-page React browser shows upload/new-folder UI, keep those parts
   clearly transitional until Phase 5 wires structured mutation handling.
4. Do not introduce field-addressable validation JSON yet unless required as a
   small compatibility step for later work.

Acceptance checks:
- Phase 4 remains focused on GET data APIs and navigation.
- Upload/folder JSON semantics are not half-introduced here.

## Task 9: Document The New Client Data Architecture
Goal:
make the Phase 4 API/data-loading layer understandable to contributors.

Steps:
1. Update `README.md` to document the new `/api/v1/iam` and
   `/api/v1/files/entries` endpoints.
2. Document that the top-level page now initializes from typed JSON data.
3. Document the current split:
   React full page uses typed APIs,
   while live embedded-route rollout and mutation JSON work are still pending.
4. Document where frontend API-client and history helpers live.

Minimum documentation content:
- the purpose of the two initial GET APIs
- where the frontend API client lives
- where history/path helpers live
- what is deferred to Phase 5 and later

Acceptance checks:
- A contributor can locate the new backend/frontend data-loading layer from the
  docs.

## Task 10: Verify Phase 4
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run test`
3. `cd frontend && npm run build`
4. `cargo build --all-features --verbose`
5. `cargo test --all-features --verbose`
6. `cargo clippy --all-features --tests -- -Dwarnings`
7. `cargo fmt --all -- --check`

What to confirm:
- the new client data APIs build and test cleanly
- the top-level page initializes from typed JSON data
- main-page `path` and back/forward behavior still work
- embedded-mode data loading preserves `baseUrl` and disables history mutation
- no HTML fragment fetches remain on the top-level React page

## Phase 4 Exit Checklist
Mark Phase 4 done only if all of the following are true:

- `GET /api/v1/iam` exists and returns typed JSON.
- `GET /api/v1/files/entries?path=...` exists and returns typed JSON.
- The full-page React entry initializes from the typed APIs.
- Main-page directory navigation no longer depends on HTML fragment fetches.
- Main-page `path` and browser back/forward behavior are preserved.
- Embedded-mode data loading supports `baseUrl` with history disabled.
- `README.md` documents the new client data architecture.

## Explicit Non-Goals For This Task File
Do not do these here:

- convert uploads to structured JSON responses
- convert folder creation to structured JSON responses
- add field-addressable validation JSON for mutations as the main goal
- remove `assets/filebrowser.js`
- delete Tera file-browser fragments
- fully cut `/files/browser` over to React if rollout is deferred to a later
  phase
- add client-side routing or SPA navigation
