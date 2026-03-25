# Tasks: React Frontend Migration Phase 5

## Scope
This task file covers only Phase 5 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- React-owned upload interactions for the files browser
- React-owned new-folder creation flow
- structured JSON mutation responses for upload and folder creation
- field-addressable validation errors and form-owned validation copy

Do not start Phase 6 or later phases in this file. Phase 5 is complete only
when uploads and folder creation work end-to-end through React/data-driven
mutation flows with behavior matching the current browser, while the embedded
route rollout, screenshot parity work, and legacy runtime removal remain on
later phases.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 4 is already complete:
  the top-level page is React/data-driven,
  typed GET APIs exist under `/api/v1/`,
  and shared browser/history/client helpers already exist.
- Keep authentication, authorization, path validation, and storage writes in
  Rust service code.
- Preserve current upload and folder semantics unless this phase explicitly
  changes the transport shape to structured JSON.
- Do not cut `/files/browser` over to React in this phase.

## Deliverables
- The main React file browser exposes upload UI again with drag-and-drop and
  file-input support.
- Upload progress and per-file status are visible in the React UI.
- The main React file browser exposes a React-controlled new-folder panel.
- Upload and folder creation routes support structured JSON responses usable by
  React.
- Validation errors are returned in a field-addressable DTO shape.
- Create-folder validation copy is owned by `src/forms`, not route-level
  message mapping.

## Task 1: Define Mutation Response DTOs
Goal:
establish stable backend/frontend contracts for upload and create-folder
success and failure handling.

Steps:
1. Add or extend DTOs under `src/dto/` for structured mutation responses.
2. Cover at least:
   upload success/error payloads,
   create-folder success/error payloads,
   and field-addressable validation errors for React-owned forms.
3. Keep the response shape frontend-oriented and predictable for TypeScript.
4. Separate validation failures from generic server failures in the DTO design.
5. Make the DTOs reusable by both the main page and future embedded/browser
   rollout where relevant.

Constraints:
- Do not expose raw framework error strings as the API contract.
- Do not embed HTML fragments or rendered template snippets in mutation
  responses.
- Do not leak unsanitized path values back to the client.

Acceptance checks:
- Stable DTOs exist for mutation success and failure flows.
- Validation errors can be addressed per field by the frontend.
- DTOs are suitable for both upload and folder-creation interactions.

## Task 2: Move Create-Folder Validation Copy Into Forms
Goal:
ensure folder-creation validation messages are owned by form-layer validation
instead of route-local branching.

Steps:
1. Inspect the existing folder-creation form and route flow.
2. Move validation copy and field-level error generation into `src/forms`.
3. Normalize and sanitize incoming folder names before building domain values.
4. Keep route handlers thin by relying on form/service results rather than
   custom message mapping.
5. Add or update tests around folder-name validation behavior.

Constraints:
- Keep validation rules aligned with current behavior unless the feature spec
  requires a deliberate change.
- Do not move business/storage checks out of the service layer.

Acceptance checks:
- Folder validation copy comes from form-layer validation.
- Route handlers no longer own detailed create-folder validation messages.
- Tests cover the expected validation outcomes.

## Task 3: Add Structured JSON Handling For Folder Creation
Goal:
let the React browser create folders without relying on full-page redirects or
Tera-owned UI responses.

Steps:
1. Update the folder-creation endpoint or add a compatible JSON response path
   that React can call without changing the business operation itself.
2. Accept the current required inputs, including the active directory path.
3. Preserve the same authentication, authorization, and path-sanitization
   behavior as the existing folder flow.
4. Return structured success, validation-error, and server-error responses.
5. Add backend tests for:
   successful folder creation,
   duplicate/invalid folder cases as applicable,
   invalid path handling,
   and unauthorized behavior.

Constraints:
- Avoid introducing a second business implementation for folder creation.
- Do not break the existing non-React route behavior if it still needs to live
  during migration.
- Keep redirects/flash handling available where legacy consumers still depend
  on them, but make JSON behavior explicit for React.

Acceptance checks:
- React has a stable JSON-capable folder-creation backend path.
- Validation and authorization behavior remain aligned with current rules.
- Tests cover success and failure modes.

## Task 4: Add Structured JSON Handling For Uploads
Goal:
let the React browser upload files through a structured mutation flow while
preserving current backend storage behavior.

Steps:
1. Inspect the current upload route and service flow.
2. Update the upload endpoint or add a compatible JSON response path for React
   consumers.
3. Preserve the current authentication, authorization, path validation, and
   storage semantics.
4. Return structured responses for:
   successful upload,
   validation/input failure,
   and unexpected server failure.
5. Add backend tests for representative upload success and failure scenarios.

Implementation notes:
- The request may remain multipart/form-data if that is the current backend
  contract.
- The important change in this phase is the response shape and React mutation
  handling, not replacing multipart transport.

Constraints:
- Do not redesign file-download URLs here.
- Do not bypass existing storage/service logic.
- Do not break legacy upload consumers if they still exist.

Acceptance checks:
- React has a stable structured upload response path.
- Upload behavior still respects existing backend rules.
- Tests cover success and error handling.

## Task 5: Add Shared Frontend Mutation Clients
Goal:
centralize upload and folder-creation requests in a reusable frontend client
layer.

Steps:
1. Extend `frontend/src/lib/` with typed mutation helpers for upload and
   create-folder operations.
2. Reuse the existing API-client conventions from Phase 4 where practical.
3. Parse structured success and validation-error responses at the frontend
   boundary.
4. Surface predictable typed results for:
   success,
   field-level validation errors,
   unauthorized responses,
   and generic server failures.
5. Keep the client layer reusable by the main page now and the embedded React
   browser later.

Constraints:
- Do not scatter mutation fetch logic across individual UI components.
- Do not reintroduce HTML-fragment or redirect-based mutation handling into the
  React flow.
- Do not hardcode full-page-only assumptions into the client helpers.

Acceptance checks:
- There is one shared frontend mutation client surface.
- Mutation results are typed and explicit.
- Validation and server error states are consumable by React components.

## Task 6: Reintroduce React Upload UI On The Main Page
Goal:
restore upload capability to the main React browser with explicit status and
error handling.

Steps:
1. Re-enable the upload region in the main-page `FileBrowser` flow.
2. Support both file-input selection and drag-and-drop.
3. Show per-file upload state in the UI, including in-flight, success, and
   failure states.
4. Keep file downloads, previews, and navigation behavior intact while uploads
   are in progress.
5. Refresh the current directory data through the typed API flow after
   successful uploads.

Constraints:
- Do not add client-side routing.
- Do not regress current Bootstrap-based visual structure without a clear
  reason.
- Avoid global DOM event wiring when React component ownership is sufficient.

Acceptance checks:
- The main React page supports drag-and-drop and file-input uploads.
- Per-file upload status is visible and understandable.
- Successful uploads refresh the visible directory contents.

## Task 7: Reintroduce React New-Folder UI On The Main Page
Goal:
restore folder creation to the main React browser with field-level validation
handling.

Steps:
1. Re-enable the new-folder panel in the main-page `FileBrowser` flow.
2. Keep the panel React-controlled rather than DOM-script controlled.
3. Submit folder-creation requests through the shared mutation client.
4. Render field-level validation errors directly in the form UI.
5. Refresh the current directory data after successful folder creation.

Constraints:
- Keep the UX aligned with the current browser unless a React-owned equivalent
  is clearly better and still preserves behavior.
- Do not push validation message formatting into the route layer.

Acceptance checks:
- The main React page supports folder creation end-to-end.
- Validation errors render at the field level.
- Successful folder creation refreshes the current directory listing.

## Task 8: Keep Embedded Rollout Deferred While Preserving Compatibility
Goal:
complete mutation infrastructure without prematurely cutting the embedded route
over to the React runtime.

Steps:
1. Keep the live `/files/browser` route on its current runtime unless a thin
   compatibility wrapper is required.
2. Make sure shared mutation helpers are written so embedded React usage can
   adopt them later.
3. Preserve `baseUrl` compatibility in URL and client helpers where it matters
   for future embedded usage.
4. Do not remove `assets/filebrowser.js` in this phase.

Acceptance checks:
- Phase 5 lands reusable mutation infrastructure without forcing the embedded
  route rollout.
- Later phases can adopt the same mutation client/helpers for embedded mode.

## Task 9: Document The Mutation Architecture
Goal:
make the Phase 5 mutation flow understandable to contributors.

Steps:
1. Update `README.md` to describe the new structured upload and create-folder
   behavior for the React main page.
2. Document where the mutation DTOs live in Rust.
3. Document where the frontend mutation client/helpers live.
4. Document that embedded-route rollout and legacy removal are still deferred
   to later phases.

Minimum documentation content:
- which mutation flows are now React/data-driven
- where field-level validation DTOs live
- where frontend mutation helpers live
- what remains deferred to Phase 6 and Phase 7

Acceptance checks:
- A contributor can locate and understand the mutation architecture without
  reverse engineering it.

## Task 10: Verify Phase 5
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cd frontend && npm run test`
4. `cargo build --all-features --verbose`
5. `cargo test --all-features --verbose`
6. `cargo clippy --all-features --tests -- -Dwarnings`
7. `cargo fmt --all -- --check`

What to confirm:
- the new mutation DTOs and routes compile cleanly
- upload and folder creation work through the React main page
- field-level validation errors render correctly in the React UI
- successful mutations refresh directory data through the typed API flow
- `/files/browser` remains on the current rollout path

## Phase 5 Exit Checklist
Mark Phase 5 done only if all of the following are true:

- The main React browser supports uploads with drag-and-drop and file-input
  flows.
- The main React browser shows per-file upload status.
- The main React browser supports folder creation through a React-controlled
  form.
- Upload and create-folder responses are structured for React consumption.
- Validation errors are field-addressable.
- Create-folder validation copy is owned by `src/forms`.
- Later-phase embedded rollout and legacy removal work remain clearly deferred.
