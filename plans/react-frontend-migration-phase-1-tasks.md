# Tasks: React Frontend Migration Phase 1

## Scope
This task file covers only Phase 1 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- `frontend/` directory with React, TypeScript, and Vite configured
- build output emitted to `assets/dist/`
- backend manifest loader and helpers for serving built HTML
- developer documentation for installing Node and building frontend assets

Do not start Phase 2 in this file. Phase 1 is complete only when the repository
can build frontend assets and the backend has the infrastructure needed to serve
those built assets later.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Do not migrate route behavior, file-browser UI behavior, or `/api/v1/...`
  endpoints in this phase.
- Do not remove Tera templates in this phase.

## Deliverables
- `frontend/` exists with a working React + TypeScript + Vite setup.
- `assets/dist/` is the configured production output directory.
- Backend helper code exists for:
  serving a built HTML file,
  loading and resolving the Vite manifest.
- Project docs explain how to install Node tooling, build the frontend, and run
  the service with frontend assets.

## Task 1: Create The Frontend Workspace
Goal:
add the frontend toolchain without changing current runtime behavior.

Steps:
1. Create `frontend/`.
2. Add `frontend/package.json`.
3. Add `frontend/tsconfig.json`.
4. Add `frontend/vite.config.ts`.
5. Add an initial source tree:
   `frontend/src/entries/`
   `frontend/src/components/`
   `frontend/src/pages/`
   `frontend/src/styles/`
   `frontend/src/lib/`
6. Add a minimal React entry module for the future full-page app.
7. Add a minimal Vite HTML entry file for the top-level files page.
8. Install dependencies and commit `frontend/package-lock.json`.

Required `package.json` scripts:
- `dev`
- `build`
- `preview`
- `test`
- `lint`
- `typecheck`
- `playwright:screenshots`

Recommended package baseline:
- `react`
- `react-dom`
- `typescript`
- `vite`
- `vitest`
- `eslint`
- `@types/react`
- `@types/react-dom`

Acceptance checks:
- `frontend/package.json` exists and includes the required scripts.
- `frontend/package-lock.json` exists.
- `npm run build` can be invoked from `frontend/`.

## Task 2: Configure Vite Output For This Service
Goal:
make the build emit files exactly where the Rust service expects them.

Steps:
1. Configure Vite to write build artifacts to `../assets/dist/`.
2. Enable Vite manifest output at `assets/dist/manifest.json`.
3. Configure the top-level HTML entry so Vite emits a built HTML document for
   the future `GET /` route.
4. Make sure asset paths are compatible with serving from `/assets`.
5. Ensure the build output layout is deterministic and suitable for Actix file
   serving.

Implementation notes:
- The source HTML entry should live in `frontend/`, not in `templates/`.
- The output should be usable by Actix without a separate Node runtime.
- Do not wire `/files/browser` yet unless the bundler requires a placeholder
  bundle entry for shared code.

Acceptance checks:
- Running `cd frontend && npm run build` creates:
  `assets/dist/manifest.json`
  and at least one built HTML file for the top-level page.
- `assets/dist/` contains hashed JS/CSS assets referenced by the manifest.

## Task 3: Add Backend Frontend-Asset Infrastructure
Goal:
prepare the Rust service to understand built frontend artifacts without cutting
over any route yet.

Steps:
1. Add a small backend module for frontend asset concerns.
2. Implement a helper to locate and read `assets/dist/manifest.json`.
3. Implement typed manifest parsing for the fields the service needs.
4. Implement a helper to resolve a named frontend entry to its built file(s).
5. Implement a helper to serve a built HTML file from `assets/dist/`.
6. Make failure behavior explicit when:
   the manifest is missing,
   an entry is missing,
   or the built HTML file is missing.
7. Add unit tests for manifest parsing and entry resolution.

Suggested placement:
- `src/frontend.rs` or
- `src/services/frontend_assets.rs` with a small route/helper surface

Constraints:
- Do not switch `GET /` to built HTML in this phase.
- Do not embed page JSON in HTML helpers.
- Keep current `/assets` static serving in place.

Acceptance checks:
- There is backend code that can resolve a built entry from
  `assets/dist/manifest.json`.
- There is backend code that can serve a chosen built HTML file from
  `assets/dist/`.
- Unit tests cover success and missing-file/missing-entry cases.

## Task 4: Keep Runtime Behavior Unchanged
Goal:
land Phase 1 without partially migrating the UI.

Steps:
1. Confirm `src/lib.rs` still serves the current Tera-based files page.
2. Confirm no route starts using the new built HTML helper yet.
3. Confirm no file-browser interaction is moved to React in this phase.
4. Confirm no `/api/v1/...` frontend data endpoints are introduced in this
   phase.

Acceptance checks:
- Existing Tera templates remain the active rendering path.
- Existing `assets/filebrowser.js` remains the active browser runtime.

## Task 5: Add Developer Documentation
Goal:
make Phase 1 usable by a contributor without reverse engineering the setup.

Steps:
1. Update `README.md` with frontend prerequisites.
2. Document how to install Node for this service.
3. Document how to install frontend dependencies.
4. Document how to build frontend assets.
5. Document whether backend startup expects prebuilt frontend assets in Phase 1.
6. Document the current migration state:
   frontend workspace exists,
   but runtime still uses Tera until later phases.

Minimum documentation content:
- `cd frontend && npm install`
- `cd frontend && npm run build`
- where build output lands
- whether `cargo run` requires a prior frontend build in Phase 1

Acceptance checks:
- A contributor can read `README.md` and reproduce the Phase 1 setup.

## Task 6: Add Ignore Rules If Needed
Goal:
keep generated assets and dependencies out of version control unless explicitly
required.

Steps:
1. Check whether `pushkind-files/.gitignore` exists.
2. If not, create it.
3. Add `frontend/node_modules/`.
4. Add `assets/dist/` unless deployment policy explicitly requires committed
   build artifacts.

Acceptance checks:
- Generated frontend dependencies are ignored.
- Generated build output is ignored unless you intentionally choose otherwise.

## Task 7: Verify Phase 1
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cargo build --all-features --verbose`
4. `cargo test --all-features --verbose`
5. `cargo clippy --all-features --tests -- -Dwarnings`
6. `cargo fmt --all -- --check`

What to confirm:
- frontend build succeeds
- backend still builds cleanly
- new manifest/helper tests pass
- no route behavior changed yet

## Phase 1 Exit Checklist
Mark Phase 1 done only if all of the following are true:

- `frontend/` exists with React, TypeScript, and Vite configured.
- `frontend/package-lock.json` is committed.
- Vite emits build artifacts into `assets/dist/`.
- `assets/dist/manifest.json` is produced by the build.
- Rust has tested helpers for manifest loading and built HTML resolution.
- `README.md` explains how to build frontend assets.
- The service still renders the existing Tera UI at runtime.

## Explicit Non-Goals For This Task File
Do not do these here:

- switch `GET /` to Vite-built HTML
- migrate `/files/browser` to React
- add `/api/v1/iam`
- add `GET /api/v1/files/entries`
- convert upload or folder creation responses to structured JSON
- remove `assets/filebrowser.js`
- delete Tera templates
