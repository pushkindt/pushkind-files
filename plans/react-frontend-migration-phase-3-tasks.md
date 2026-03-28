# Tasks: React Frontend Migration Phase 3

## Scope
This task file covers only Phase 3 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- shared React shell infrastructure for the full page
- React-safe Bootstrap helper utilities
- an initial shared `FileBrowser` component and endpoint builder layer

Do not start Phase 4 or later phases in this file. Phase 3 is complete only
when the repository has a reusable React browser UI foundation that can render
from typed data and support both the top-level page and embedded mode later,
without yet switching live browser data loading to `/api/v1/...`.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 2 is already complete:
  `GET /` is served from built frontend HTML,
  the top-level page mounts React,
  and the legacy file-browser runtime is still active.
- Keep the embedded browser compatibility surface on its current Tera route in
  this phase.
- Do not introduce the Phase 4 client data APIs yet.

## Deliverables
- A shared React shell exists for page layout, flash presentation wiring, and
  future page bootstrap concerns.
- Bootstrap lifecycle behavior is moved into React-safe helpers under
  `frontend/src/lib/`.
- An initial shared `FileBrowser` component exists with subcomponents and
  endpoint-builder utilities.
- The new shared browser UI can render from typed props or fixture data without
  depending on Tera-authored browser markup.

## Task 1: Introduce A Shared React Page Shell
Goal:
replace the temporary phase 2 top-level shell with reusable layout
infrastructure for the files page and future embedded/browser contexts.

Steps:
1. Create a shared page-shell component under `frontend/src/components/` or
   `frontend/src/pages/` that owns the top-level layout structure.
2. Move transitional page-header/banner/layout concerns out of the phase 2
   bootstrap component and into that shared shell.
3. Add explicit extension points for:
   flash message display,
   page-level loading/error rendering,
   and future current-user/session context.
4. Keep the shell reusable so later phases can use the same primitives for both
   `GET /` and embedded browser surfaces where appropriate.
5. Keep the shell compatible with the current Vite-built full-page document.

Constraints:
- Do not make the shared shell depend on `/api/v1/iam` yet.
- Do not restore Rust-authored page HTML.
- Do not embed server-generated JSON into the page document.

Acceptance checks:
- The top-level page shell no longer looks like a one-off phase 2 placeholder.
- The shell is reusable by later browser/page implementations.
- Shell rendering stays in React and remains buildable through Vite.

## Task 2: Move Bootstrap Lifecycle Behavior Into React-Safe Helpers
Goal:
stop relying on inline document scripts for Bootstrap wiring and make those
behaviors reusable from React.

Steps:
1. Add helper modules under `frontend/src/lib/` for Bootstrap lifecycle work.
2. Cover the behaviors currently handled inline in `templates/base.html` where
   they are relevant to the migrated surface:
   alerts,
   dropdowns,
   tooltips,
   popovers,
   and modal/show-message wiring as needed.
3. Make the helpers React-safe:
   initialize on mount,
   clean up when necessary,
   and avoid global one-off DOM assumptions where possible.
4. Update the phase 2 full-page shell to use these helpers instead of inline
   page-document scripts for migrated behavior.
5. Keep Bootstrap CSS and Bootstrap Icons in the rendered output.

Constraints:
- Do not remove Bootstrap from the runtime.
- Do not rewrite Bootstrap behavior with a custom design system.
- Do not delete legacy inline scripts that are still required by untouched
  Tera routes unless their responsibility has actually moved.

Acceptance checks:
- Bootstrap lifecycle behavior needed by the React-owned page is initialized
  from frontend code, not only from inline document scripts.
- The helper surface is reusable by later React components.
- The migrated page remains visually compatible with Bootstrap styling.

## Task 3: Create Shared Endpoint Builder Utilities
Goal:
prepare the frontend to compose future navigation and mutation URLs from a
single typed utility layer.

Steps:
1. Add endpoint-builder helpers under `frontend/src/lib/` for browser-related
   routes.
2. Cover at least:
   the embedded browser compatibility surface,
   `/files/upload`,
   `/folder/create`,
   file-download URLs under `/upload/*`,
   and base-URL handling for embedded usage.
3. Preserve the current same-origin embedding contract and `baseUrl` behavior.
4. Make the helpers typed and reusable by both the full page and the future
   embedded React mount API.
5. Replace ad hoc URL concatenation in the new React browser infrastructure
   with these helpers.

Constraints:
- Do not add `/api/v1/files/entries` here.
- Do not fetch HTML fragments from these helpers in the target state of this
  phase.
- Do not break absolute file URLs or same-origin embedded usage.

Acceptance checks:
- There is one shared frontend utility surface for browser URLs.
- Base-URL normalization is preserved for embedded usage.
- The utility layer is ready for later API/data loading work.

## Task 4: Add An Initial Shared `FileBrowser` Component
Goal:
introduce the first React-owned shared browser UI that can later replace both
the top-level widget and embedded Tera fragment.

Steps:
1. Create a `FileBrowser` component and supporting subcomponents under
   `frontend/src/components/`.
2. Model the component from typed props rather than DOM parsing or fetched HTML.
3. Include the main structural regions the migration plan expects:
   breadcrumb area,
   toolbar/header area,
   entry-grid region,
   folder-card rendering,
   file-card rendering,
   and slots/placeholders for uploads and folder creation.
4. Preserve current Bootstrap-based class structure and Russian copy as closely
   as practical.
5. Support optional history management and configurable `baseUrl` in the
   component contract, even if the real Phase 4 data flow is not wired yet.

Implementation notes:
- This phase is about component structure and shared rendering primitives.
- The component may be rendered from stubbed typed data, fixture data, or a
  thin transitional bootstrap contract.
- The component should be able to support image previews, copy-link actions,
  and normal download links at the prop/interface level, even if later phases
  finish the live behavior wiring.

Acceptance checks:
- A React-owned shared browser UI exists in the frontend workspace.
- The shared browser can render without depending on Tera-authored browser
  markup.
- The component structure is reusable by both the full page and future embedded
  mode.

## Task 5: Add A Compatibility-Oriented Embedded Mount Surface
Goal:
prepare the React browser layer for future embedded use without cutting over the
live embedded browser compatibility surface yet.

Steps:
1. Add a browser-side mount API or compatibility wrapper for the future
   embedded React browser.
2. Preserve the expected shape of:
   `window.mountFileBrowser(target, initialPath, { baseUrl })`
   or a clearly documented thin wrapper around it.
3. Ensure the new mount surface can target an arbitrary DOM node and disable
   history mutation for embedded usage.
4. Keep the live legacy implementation in place for now if Phase 3 does not yet
   switch runtime ownership.
5. Make the compatibility layer point toward the shared `FileBrowser`
   component, not a separate one-off tree.

Constraints:
- Do not cut the embedded browser compatibility surface over to React in this
  phase.
- Do not remove `assets/filebrowser.js` yet unless an equivalent compatibility
  layer fully replaces it and Phase 6 explicitly allows the route cutover.

Acceptance checks:
- There is a defined React-oriented embedded mount surface for later phases.
- Embedded history behavior remains configurable.
- The compatibility story for the eventual `window.mountFileBrowser(...)`
  migration is explicit in code.

## Task 6: Keep Runtime Behavior Stable While Building The New Layer
Goal:
land the shared infrastructure without partially switching live browser data
flows or route ownership.

Steps:
1. Keep the embedded browser compatibility document rendering through the
   current Tera fragment flow.
2. Keep the current live upload and folder-creation flows unchanged.
3. Keep file-browser data loading out of `/api/v1/...` for now.
4. Avoid partial cutovers where the live page mixes React-owned browser cards
   with Tera-owned data flow in unsupported ways.

Acceptance checks:
- The repository still has a clear boundary between Phase 3 shared UI
  infrastructure and later data/API migration work.
- No new live route depends on Phase 4 APIs.
- The legacy route and browser remain functional until later cutover phases.

## Task 7: Document The New Frontend Architecture Layer
Goal:
make the Phase 3 shared browser infrastructure understandable to contributors.

Steps:
1. Update `README.md` with a short description of the new shared React shell and
   shared browser component layer.
2. Document where Bootstrap lifecycle helpers now live.
3. Document that the shared `FileBrowser` component exists before the real
   client data APIs are introduced.
4. Document that the embedded browser compatibility surface still remains on
   the legacy route/runtime in this phase.

Minimum documentation content:
- where the shared shell lives
- where the shared browser component lives
- where Bootstrap helper utilities live
- what is still deferred to Phase 4 and later

Acceptance checks:
- A contributor can locate the new frontend architecture pieces without reverse
  engineering the codebase.

## Task 8: Verify Phase 3
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cd frontend && npm run test`
4. `cargo build --all-features --verbose`
5. `cargo test --all-features --verbose`
6. `cargo clippy --all-features --tests -- -Dwarnings`
7. `cargo fmt --all -- --check`

What to confirm:
- the shared frontend components build cleanly
- the React-owned page still loads after the refactor
- the shared browser UI can render from typed data without Tera browser markup
- Bootstrap lifecycle wiring still works for the migrated page
- the embedded browser compatibility surface remains on the existing
  route/runtime

## Phase 3 Exit Checklist
Mark Phase 3 done only if all of the following are true:

- A shared React shell exists for the files page.
- Bootstrap lifecycle behavior needed by the migrated page is handled through
  React-safe frontend helpers.
- A shared `FileBrowser` component exists with supporting subcomponents.
- Shared endpoint-builder utilities exist for browser-related URLs.
- The browser layer can render from typed data without relying on Tera browser
  markup.
- A future-facing embedded mount API or compatibility wrapper exists.
- the embedded browser compatibility surface still remains on the legacy
  route/runtime until a later cutover phase.

## Explicit Non-Goals For This Task File
Do not do these here:

- add `GET /api/v1/iam`
- add `GET /api/v1/files/entries`
- migrate live directory loading to typed client data APIs
- convert uploads or folder creation to structured JSON responses
- cut the embedded browser compatibility surface over to React
- remove `assets/filebrowser.js`
- delete Tera file-browser fragments
- introduce client-side routing or SPA navigation
