# Tasks: React Frontend Migration Phase 2

## Scope
This task file covers only Phase 2 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- Vite-managed HTML for `GET /`
- backend route behavior that serves the built HTML after auth checks
- an initial React entry for the full files page
- loading and fatal error states for future client-side data fetching

Do not start Phase 3 or Phase 4 in this file. Phase 2 is complete only when
the authenticated top-level files page is served from a Vite-built HTML
document instead of Rust-rendered page HTML, while the actual file-browser UI
and APIs remain on their current implementation.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 1 is already complete:
  `frontend/` exists,
  `assets/dist/` is configured,
  and the backend can load the Vite manifest and serve built HTML files.
- Keep the current auth and authorization behavior for `GET /`.
- Do not migrate the embedded browser compatibility surface, uploads, folder
  creation, or `/api/v1/files` endpoints in this phase.

## Deliverables
- `GET /` is served from a Vite-built HTML document after auth checks.
- The top-level page uses an initial React app entry instead of Rust-assembled
  page markup.
- The React page shows explicit loading and fatal states for future data
  loading.
- The current Tera/browser-script compatibility flow remains intact for
  embedded use and later migration phases.

## Task 1: Cut Over `GET /` To Built HTML
Goal:
stop assembling the top-level page document in Rust and serve the Vite-built
HTML document instead.

Steps:
1. Reuse the Phase 1 frontend asset helper for serving the built `index.html`
   document from `assets/dist/`.
2. Update the `GET /` route so it still performs authentication and role checks
   before returning the built HTML file.
3. Keep explicit failure behavior when:
   the built HTML file is missing,
   the frontend build has not been run,
   or the serving helper fails unexpectedly.
4. Remove Rust page-document assembly from the `GET /` path.
5. Keep `/assets` static serving unchanged so the built document can load its
   JS and CSS.

Constraints:
- Do not silently fall back to `templates/main/index.html` once this cutover is
  made.
- Do not bypass `RedirectUnauthorized` or `SERVICE_ACCESS_ROLE`.
- Do not change the embedded browser compatibility surface in this task.

Acceptance checks:
- `GET /` no longer renders `templates/main/index.html`.
- `GET /` still requires the same auth and authorization gates.
- Missing built HTML produces a clear server-side error path rather than an
  opaque blank response.

## Task 2: Add A Minimal React-Owned Full-Page Shell
Goal:
replace the Rust-owned top-level page document with a React-owned shell without
yet migrating the file-browser UI itself.

Steps:
1. Turn the Phase 1 placeholder into an initial files-page shell for `GET /`.
2. Mount React from the Vite-built document using the existing full-page entry.
3. Render a minimal page-level shell that makes it obvious the document is now
   React-owned.
4. Keep the shell intentionally small and transitional so Phase 3 can replace
   it with shared layout and file-browser components.
5. Avoid embedding server-generated JSON into the HTML document.

Implementation notes:
- The page shell does not need to render the actual file browser in this phase.
- The shell should be structured so future phases can introduce shared layout,
  flash handling, and file-browser state without replacing the whole entry.

Acceptance checks:
- The built document for `GET /` mounts React successfully.
- The top-level page markup is no longer authored by Tera.
- No file-browser interactions are migrated to React yet.

## Task 3: Add Loading And Fatal States For Future Data Fetching
Goal:
prepare the top-level React page to load typed data later without taking on the
real browser/API migration yet.

Steps:
1. Add a page bootstrap abstraction in `frontend/src/lib/` for future client
   data loading.
2. Render an explicit loading state while that bootstrap step is pending.
3. Render an explicit fatal error state when bootstrap fails.
4. Keep the bootstrap implementation intentionally narrow in this phase:
   it MAY use a temporary stubbed async loader or similarly small transitional
   abstraction,
   but it MUST NOT migrate file-browser data fetching yet.
5. Make sure the loading and fatal UI are production-buildable and visible in
   code, not just planned comments.

Constraints:
- Do not add `GET /api/v1/files/entries` in this phase.
- Do not reintroduce HTML fragment fetching in the embedded browser
  compatibility flow.
- Do not make the page depend on embedded JSON in the built HTML.

Acceptance checks:
- The React page code contains explicit loading and fatal states.
- The initial page shell can be extended to real fetches later without
  rewriting the route cutover.
- No real file-browser data flow is migrated yet.

## Task 4: Keep Existing File-Browser Runtime Behavior Intact
Goal:
cut over only the top-level HTML document without partially migrating the file
browser or embedded route.

Steps:
1. Keep the embedded browser compatibility document rendering through the
   current Tera/browser-script flow.
2. Keep `assets/filebrowser.js` as the active browser runtime for the current
   file-browser UI.
3. Keep the existing upload and folder-creation endpoints and response shapes.
4. Keep the current `path`-driven browser behavior unchanged outside the new
   top-level document shell.

Acceptance checks:
- The embedded browser route still uses the existing Tera fragment.
- No file-browser card grid, breadcrumb, upload flow, or folder form is moved
  to React in this phase.
- No `/api/v1/...` resource endpoint is introduced purely for the browser UI in
  this phase.

## Task 5: Document The New Runtime Expectation
Goal:
make the Phase 2 cutover understandable to contributors and operators.

Steps:
1. Update `README.md` to explain that `GET /` now depends on built frontend
   assets.
2. Document whether `cargo run` requires `cd frontend && npm run build` before
   local startup in Phase 2.
3. Document that the embedded browser compatibility surface is still using the
   legacy Tera/browser-script path.
4. Document that the React page shell is present, but the full browser/data API
   migration happens in later phases.

Minimum documentation content:
- that `GET /` is now served from built frontend HTML
- whether missing `assets/dist/index.html` breaks the top-level page
- the current split between the React full-page shell and legacy file-browser
  runtime

Acceptance checks:
- A contributor can understand the new startup expectation for `GET /`.
- The docs make it clear that this is not yet the full React browser
  migration.

## Task 6: Verify Phase 2
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cargo build --all-features --verbose`
4. `cargo test --all-features --verbose`
5. `cargo clippy --all-features --tests -- -Dwarnings`
6. `cargo fmt --all -- --check`

What to confirm:
- the frontend build still succeeds
- the backend still builds cleanly after route cutover
- `GET /` can serve the built HTML document after auth checks
- the embedded browser compatibility surface remains on the existing Tera path
- no file-browser behavior has been partially migrated yet

## Phase 2 Exit Checklist
Mark Phase 2 done only if all of the following are true:

- `GET /` is served from a Vite-built HTML document.
- Rust still owns access checks before the document is served.
- The top-level page mounts an initial React app entry.
- The React page has explicit loading and fatal states for future bootstrap
  data loading.
- the embedded browser compatibility surface is still rendered by the existing
  Tera fragment route.
- `assets/filebrowser.js` is still the active runtime for the current browser
  UI.
- `README.md` explains the new asset-build expectation for `GET /`.

## Explicit Non-Goals For This Task File
Do not do these here:

- migrate the embedded browser compatibility surface to React
- add shared React file-browser components
- add `GET /api/v1/files/entries`
- convert uploads or folder creation to structured JSON responses
- replace `assets/filebrowser.js`
- remove the Tera fragment templates used by the current browser UI
- add client-side routing or SPA navigation
