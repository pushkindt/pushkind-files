# Tasks: React Frontend Migration Phase 6

## Scope
This task file covers only Phase 6 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- React-backed rollout of the shared browser on the main page and embedded
  route
- preservation of embedded `baseUrl` behavior and browser interactions
- screenshot-based visual parity coverage for the main page and embedded browser

Do not start Phase 7 or later phases in this file. Phase 6 is complete only
when both `GET /` and `GET /files/browser` render through the shared React
browser with working interactions and screenshot baselines, while legacy Tera
browser fragment removal and `assets/filebrowser.js` deletion remain deferred
to the cleanup phase.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 5 is already complete:
  the main React page is data-driven,
  upload and create-folder mutations are React-owned on the main page,
  and shared browser/client/mount helpers already exist.
- Keep authentication, authorization, path validation, and storage persistence
  in Rust service code.
- Do not remove legacy files or template fragments in this phase unless a thin
  compatibility wrapper requires a temporary handoff.

## Deliverables
- `GET /` remains React-backed through the shared browser component.
- `GET /files/browser` becomes React-backed for embedded usage.
- Copy-link behavior, previews, downloads, navigation, upload, and folder
  creation continue to work on both surfaces.
- Embedded mode preserves `baseUrl` and keeps history mutation disabled.
- Playwright screenshot baselines exist for the main page and embedded browser
  states.

## Task 1: Audit The Remaining Runtime Split
Goal:
identify the exact legacy/runtime seams that still keep `/files/browser` on the
old path and define the minimal rollout needed for Phase 6.

Steps:
1. Inspect the current `/files/browser` route, legacy template fragment, and
   `assets/filebrowser.js` usage.
2. Identify which responsibilities are already covered by the shared React
   browser and which still depend on legacy glue.
3. Decide whether the route should:
   serve a React mount shell,
   serve built frontend HTML,
   or serve a minimal compatibility wrapper for embedded usage.
4. Keep the rollout compatible with same-origin embedding and existing consumer
   expectations.
5. Document any temporary compatibility layer left for Phase 7 cleanup.

Constraints:
- Do not remove legacy files as part of this audit task.
- Do not invent a second React browser implementation for embedded mode.

Acceptance checks:
- The remaining rollout boundary is explicit in code and plan.
- Phase 6 implementation can proceed without ambiguity about route ownership.

## Task 2: Cut `/files/browser` Over To The Shared React Browser
Goal:
make the embedded browser route render through the React/browser data flow
instead of the legacy Tera fragment.

Steps:
1. Update the backend route for `GET /files/browser` to serve the new React
   embedded surface.
2. Keep the same authentication and `"files"` access semantics as the existing
   route.
3. Preserve support for the optional `path` parameter.
4. Keep any required shell minimal and transitional rather than rebuilding the
   old Tera browser fragment in Rust.
5. Add or update backend tests covering route success, invalid-path handling,
   and unauthorized behavior for the embedded route.

Constraints:
- Do not move embedded authorization checks into frontend code.
- Do not introduce client-side route ownership for `/files/browser`.
- Do not remove the legacy assets in this phase even if the route no longer
  uses them.

Acceptance checks:
- `GET /files/browser` is React-backed.
- The route preserves existing auth and path-validation behavior.
- Tests cover the new route behavior.

## Task 3: Finalize The Embedded React Mount Contract
Goal:
make the embedded browser mount surface explicit, stable, and compatible with
existing same-origin embed usage.

Steps:
1. Review the current browser-side mount API under `frontend/src/lib/`.
2. Preserve or clearly wrap the existing
   `window.mountFileBrowser(target, initialPath, { baseUrl })` contract.
3. Ensure embedded mounts can target arbitrary DOM nodes.
4. Keep browser-history mutation disabled in embedded mode.
5. Keep the contract future-proof enough that Phase 7 can remove legacy glue
   without breaking consumers.

Constraints:
- Do not require host pages to adopt SPA routing or a new navigation model.
- Do not break current `baseUrl` handling semantics.

Acceptance checks:
- There is one clear embedded mount contract.
- The compatibility story for `window.mountFileBrowser(...)` is explicit in
  code.
- Embedded mode remains history-disabled.

## Task 4: Reuse The Shared React Browser End To End On Both Surfaces
Goal:
ensure the same shared browser behavior works on the main page and the embedded
route without one-off divergence.

Steps:
1. Reuse the shared `FileBrowser` component for both `GET /` and
   `GET /files/browser`.
2. Verify that navigation, upload, folder creation, copy-link actions, image
   previews, and download links all work in both contexts.
3. Keep browser state/data loading driven by the shared frontend client layer.
4. Remove any page-specific hacks that prevent the shared component from being
   reused cleanly across both surfaces.
5. Add or update frontend tests for any shared behavior that becomes reusable
   across both contexts.

Constraints:
- Do not fork the browser UI into separate main-page and embedded versions.
- Do not reintroduce HTML-fragment fetches or DOM parsing.

Acceptance checks:
- Both surfaces use the same shared browser implementation.
- Core browser interactions work in both contexts.
- Shared behavior stays testable and maintainable.

## Task 5: Preserve Embedded `baseUrl` Semantics
Goal:
keep same-origin embedding behavior stable after the React rollout.

Steps:
1. Verify that browser navigation URLs resolve correctly relative to `baseUrl`.
2. Verify that upload, folder-creation, download, preview, and copy-link URLs
   continue to honor `baseUrl`.
3. Ensure API requests for embedded mode also use the resolved base URL.
4. Add or update focused tests for URL resolution and embedded-mode helpers.
5. Make any fixes in shared URL/client helpers rather than embedded-only
   one-offs.

Constraints:
- Do not assume embedded mode runs at the site root.
- Do not hardcode `window.location.origin` where `baseUrl` is required.

Acceptance checks:
- Embedded navigation and actions work correctly behind a `baseUrl`.
- Shared helpers remain the single source of truth for URL composition.

## Task 6: Preserve Main-Page Behavior While Completing The Rollout
Goal:
finish the embedded rollout without regressing the already-migrated main page.

Steps:
1. Confirm that `GET /` still serves the Vite-built frontend document.
2. Keep main-page history synchronization for `path` and `popstate`.
3. Verify uploads, folder creation, previews, downloads, and copy-link actions
   still work on the main page after shared-rollout changes.
4. Keep loading and fatal states explicit for both page-level and embedded
   contexts where appropriate.
5. Add regression coverage where the rollout introduces new shared logic.

Constraints:
- Do not hand main-page behavior back to legacy scripts or templates.
- Do not remove the built-HTML route helper for `GET /`.

Acceptance checks:
- The main page remains fully React-backed and functional.
- Embedded rollout does not regress main-page interactions.

## Task 7: Add Playwright Screenshot Baselines
Goal:
capture visual parity for the React-backed main page and embedded browser
states.

Steps:
1. Extend the Playwright screenshot workflow under `frontend/` for Phase 6.
2. Capture at least:
   main page initial render,
   embedded browser initial render,
   and one representative navigated state if practical.
3. Keep screenshot generation reproducible for contributors and CI.
4. Store or document the baselines according to the repository’s existing
   screenshot workflow.
5. Record any known visual deltas that are intentional rather than regressions.

Constraints:
- Do not rely only on manual browser inspection for parity claims.
- Keep the screenshot flow compatible with the existing
  `playwright:screenshots` script.

Acceptance checks:
- Screenshot baselines exist for the main page and embedded browser.
- Contributors can run the screenshot flow locally.
- Visual parity work is part of the checked-in Phase 6 deliverable.

## Task 8: Keep Legacy Removal Explicitly Deferred
Goal:
complete the rollout without mixing in cleanup work reserved for Phase 7.

Steps:
1. Keep `assets/filebrowser.js` in the repository for now, even if runtime
   ownership has shifted away from it.
2. Keep obsolete Tera browser fragments in place until Phase 7 removes them.
3. Avoid broad cleanup/refactor work that obscures the rollout diff.
4. Leave clear comments or documentation where temporary compatibility code
   remains.

Acceptance checks:
- Phase 6 lands the runtime rollout without prematurely deleting legacy files.
- The remaining Phase 7 cleanup scope is still clear.

## Task 9: Document The Route Rollout
Goal:
make the final pre-cleanup frontend runtime understandable to contributors.

Steps:
1. Update `README.md` to describe the Phase 6 runtime state.
2. Document that both `GET /` and `GET /files/browser` are now React-backed.
3. Document where the embedded mount API and compatibility wrapper live.
4. Document that legacy asset/template removal is still deferred to Phase 7.

Minimum documentation content:
- which routes are now React-backed
- where the embedded mount API lives
- how `baseUrl` and embedded history behavior work
- what remains for Phase 7 cleanup

Acceptance checks:
- A contributor can understand the rollout state without tracing every route by
  hand.

## Task 10: Verify Phase 6
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cd frontend && npm run test`
4. `cd frontend && npm run playwright:screenshots`
5. `cargo build --all-features --verbose`
6. `cargo test --all-features --verbose`
7. `cargo clippy --all-features --tests -- -Dwarnings`
8. `cargo fmt --all -- --check`

What to confirm:
- both routes render through the shared React browser
- embedded mode works with `baseUrl` and without history mutation
- uploads, folder creation, previews, downloads, and copy-link actions work on
  both surfaces
- screenshot baselines cover the main page and embedded browser
- legacy cleanup is still deferred rather than half-completed

## Phase 6 Exit Checklist
Mark Phase 6 done only if all of the following are true:

- `GET /` remains React-backed through the shared browser.
- `GET /files/browser` is React-backed for embedded usage.
- Main-page and embedded browser interactions both work end to end.
- `baseUrl` semantics are preserved for embedded usage.
- Screenshot baselines exist for the main page and embedded browser.
- Phase 7 legacy removal work is still clearly deferred.
