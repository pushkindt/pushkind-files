# Tasks: React Frontend Migration Phase 7

## Scope
This task file covers only Phase 7 from
[react-frontend-migration.md](../plans/react-frontend-migration.md):

- removal of obsolete Tera-owned file-browser markup
- removal of compatibility glue left from the migration rollout
- cleanup of temporary bootstrap/runtime transport kept during earlier phases
- trimming unused template and asset wiring from the legacy browser

Do not start any post-migration redesign work in this file. Phase 7 is
complete only when no runtime path depends on imperative `filebrowser.js`,
page-shaped bootstrap transport, or Tera-owned file-browser markup, while the
React/Vite architecture introduced in earlier phases remains intact.

## Preconditions
- Work in `/home/matrizaev/pushkind/pushkind-files`.
- Treat [SPEC.md](../SPEC.md) and
  [react-frontend-migration.md](../specs/features/react-frontend-migration.md)
  as the source of truth.
- Assume Phase 6 is already complete:
  `GET /` is React-backed,
  `GET /files/browser` is React-backed,
  and the shared React browser is the active runtime for both surfaces.
- Keep authentication, authorization, path validation, and storage persistence
  in Rust service code.
- Do not reintroduce Tera browser rendering or compatibility wrappers while
  cleaning up the legacy path.

## Deliverables
- Obsolete Tera file-browser fragments are removed.
- Legacy page/browser-specific inline script responsibilities are removed where
  they no longer power any runtime path.
- `assets/filebrowser.js` is removed once the React bundle fully covers its
  contract.
- Temporary migration-only transport and wiring are removed.
- Unused template partials, browser-specific helpers, and stale docs are
  trimmed.

## Task 1: Inventory The Remaining Legacy Runtime Pieces
Goal:
make the cleanup set explicit before deleting code.

Steps:
1. Inspect the repository for remaining legacy browser-specific assets,
   templates, and helper modules.
2. Identify all runtime references to:
   `assets/filebrowser.js`,
   Tera file-browser fragments,
   migration-only browser bootstrap code,
   and obsolete embedded compatibility glue.
3. Separate active runtime dependencies from dead-but-still-checked-in files.
4. Confirm that removing each item will not break `/` or `/files/browser`.
5. Use that inventory to drive the cleanup sequence.

Acceptance checks:
- The remaining legacy surface is explicit before deletion starts.
- There is a clear list of files/modules that can be removed safely.

## Task 2: Remove Tera-Owned File-Browser Markup
Goal:
eliminate server-rendered browser fragment ownership now that both routes are
React-backed.

Steps:
1. Remove the Tera file-browser fragment(s) that previously powered
   `/files/browser`.
2. Remove route-layer rendering logic that existed only to build that fragment.
3. Remove no-longer-needed DTOs or context wiring that only served Tera browser
   rendering.
4. Update or remove tests that were specific to Tera fragment rendering.
5. Keep any still-valid non-browser templates untouched.

Constraints:
- Do not remove unrelated Tera templates that still power non-browser pages.
- Do not delete template helpers that are still used elsewhere.

Acceptance checks:
- No route renders Tera-owned file-browser markup anymore.
- Browser-specific Tera wiring is gone from the active runtime path.

## Task 3: Remove `assets/filebrowser.js`
Goal:
delete the legacy imperative browser runtime once React fully owns its contract.

Steps:
1. Confirm that the embedded/browser contract is fully covered by the React
   bundle.
2. Remove `assets/filebrowser.js`.
3. Remove any loader/helper code that injected or delegated to that asset.
4. Preserve the public embedded mount behavior through the React-owned mount
   layer.
5. Update any docs or references that still mention `filebrowser.js` as an
   active runtime dependency.

Constraints:
- Do not remove the public mount contract without a React-owned replacement.
- Do not leave broken references to the deleted asset in templates, docs, or
  runtime code.

Acceptance checks:
- `assets/filebrowser.js` is no longer present.
- No runtime path depends on that file.
- The embedded mount contract still works through React.

## Task 4: Remove Migration-Only Bootstrap Transport
Goal:
trim any temporary bootstrap or compatibility transport left over from the
migration sequence.

Steps:
1. Inspect frontend and backend code for migration-only bootstrap helpers,
   fallback payload readers, and compatibility branches.
2. Remove temporary wrappers that only existed to bridge from Tera/legacy
   runtime to React.
3. Collapse duplicated code paths where the React-only flow is now sufficient.
4. Keep the remaining frontend bootstrap path explicit and minimal.
5. Add or update tests where cleanup changes code ownership or assumptions.

Acceptance checks:
- Temporary migration-only transport is removed.
- The remaining bootstrap/runtime path is React-native and easier to follow.

## Task 5: Trim Unused Browser-Specific Helpers And DTOs
Goal:
remove dead code that survived the rollout.

Steps:
1. Identify frontend helpers, backend DTOs, and Rust route/service helpers that
   are no longer referenced after legacy removal.
2. Remove unused URL helpers, compatibility adapters, or fixture plumbing that
   only served transitional phases.
3. Remove unused DTOs that existed purely for Tera browser rendering.
4. Remove stale tests that only covered transitional code.
5. Keep the cleanup disciplined so active shared browser code remains readable.

Acceptance checks:
- The codebase no longer carries obviously dead migration scaffolding.
- Remaining browser helpers and DTOs all serve active runtime paths.

## Task 6: Clean Up Templates And Documentation
Goal:
make the repository reflect the final post-migration runtime.

Steps:
1. Update `README.md` to describe the final React-owned browser runtime.
2. Remove stale references to:
   Tera browser fragments,
   compatibility wrappers,
   and transition-only runtime splits.
3. Document the final ownership boundaries:
   Vite-built HTML,
   typed JSON APIs,
   and React-owned embedded mount behavior.
4. Remove template partial references that no longer exist.
5. Keep documentation aligned with the actual runtime after cleanup.

Acceptance checks:
- Contributor docs describe the final runtime accurately.
- No docs refer to deleted legacy browser assets as active dependencies.

## Task 7: Re-Verify The Final Runtime
Goal:
confirm that cleanup did not break the now-final React browser runtime.

Steps:
1. Verify that `GET /` still serves the built React document.
2. Verify that `GET /files/browser` still serves the embedded React document.
3. Verify uploads, folder creation, previews, downloads, navigation, and
   copy-link behavior still work through the shared React browser.
4. Verify the embedded mount contract still works without the deleted legacy
   asset.
5. Update tests where the cleanup changes assumptions or removes transitional
   files.

Acceptance checks:
- The final runtime still behaves correctly after legacy removal.
- Cleanup does not regress any core browser interaction.

## Task 8: Verify Phase 7
Run these commands from `pushkind-files` unless noted otherwise:

1. `cd frontend && npm run typecheck`
2. `cd frontend && npm run build`
3. `cd frontend && npm run test`
4. `cd frontend && npm run playwright:screenshots`
   Note: keep this as the current placeholder/no-op in this environment unless
   authenticated screenshot capture becomes available later.
5. `cargo build --all-features --verbose`
6. `cargo test --all-features --verbose`
7. `cargo clippy --all-features --tests -- -Dwarnings`
8. `cargo fmt --all -- --check`

What to confirm:
- no runtime path depends on Tera-owned browser markup
- no runtime path depends on `assets/filebrowser.js`
- `/` and `/files/browser` remain React-backed and functional
- cleanup removed transitional code instead of breaking live behavior

## Phase 7 Exit Checklist
Mark Phase 7 done only if all of the following are true:

- No route renders Tera-owned file-browser markup.
- `assets/filebrowser.js` is removed.
- No runtime path depends on migration-only browser bootstrap transport.
- Obsolete browser-specific template and asset wiring is removed.
- Contributor docs reflect the final React-owned runtime.
- The final React browser runtime still passes the full verification set.
