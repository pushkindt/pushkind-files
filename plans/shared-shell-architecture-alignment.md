# Plan: Shared Shell Architecture Alignment

## References
- Service baseline:
  [../SPEC.md](../SPEC.md)
- Existing React migration spec:
  [../specs/features/react-frontend-migration.md](../specs/features/react-frontend-migration.md)
- Feature spec:
  [../specs/features/shared-shell-architecture-alignment.md](../specs/features/shared-shell-architecture-alignment.md)

## Objective
Refactor `pushkind-files` onto the same React shell architecture used by
`pushkind-crm`, `pushkind-emailer`, `pushkind-todo`, and `pushkind-orders`
without changing the files page routes, the file-browser behavior, or the
embedded browser contract. The end state is shared navbar, shell, flash,
fatal-state, no-access, and shell data integration through
`pushkind-common/frontend`, with file-browser-specific rendering left local.

## Fixed Implementation Decisions
- Shared frontend shell code WILL be consumed from
  `@pushkind/frontend-shell`.
- `pushkind-files` WILL remain server-routed and non-SPA.
- The top-level files page MUST converge on the common shell architecture.
- The embedded browser contract MUST remain supported and MAY keep a lighter-
  weight mount surface if the full shell is inappropriate there.
- Existing files page and embedded browser markup MUST remain visually
  equivalent.
- File-browser-resource rendering and mutation logic MAY remain local.

## Migration Sequence

### Phase 1: Baseline Audit
Deliverables:
- Inventory of local files shell pieces still duplicating shared code.
- Clear boundary between top-level shell concerns and embedded browser
  concerns.
- Mapping from current files components to shared package modules.

Exit criteria:
- It is explicit which parts of files can converge directly and which parts
  remain files-specific.

### Phase 2: Top-Level Navbar Alignment
Deliverables:
- Replace the local top-level files navbar implementation with the shared
  navbar primitive.
- Preserve files brand, current menu ordering, and current visual structure.
- Keep embedded browser behavior unaffected.

Exit criteria:
- The full-page files navbar renders through the shared navbar component
  without visible drift.

### Phase 3: Flash And Fatal-State Alignment
Deliverables:
- Align top-level files flash presentation with the shared shell flash
  behavior.
- Replace local fatal shell state rendering with the shared fatal-state
  primitive.
- Preserve current files copy and page framing.

Exit criteria:
- Full-page files async actions use the shared shell flash behavior.
- Shell failures render through the shared fatal-state primitive.

### Phase 4: Shell Wrapper Alignment
Deliverables:
- Adopt the shared shell wrapper architecture for the full-page files route.
- Keep embedded browser mounts outside that shell where necessary.
- Remove local shell duplication that becomes obsolete.

Exit criteria:
- The full-page files shell no longer carries private copies of navbar/flash/
  fatal-state infrastructure that now exists in `pushkind-common/frontend`.

### Phase 5: Shared Shell Data And Type Alignment
Deliverables:
- Switch full-page files shell loading to shared shell API helpers where the
  payload contracts match.
- Replace duplicated shell base types with imports or aliases from the shared
  package.
- Keep file-browser-resource and embedded-browser parsing local.

Exit criteria:
- Files no longer duplicates shell-level frontend parsing and typing that is
  now shared.

### Phase 6: Embedded Contract Validation And Cleanup
Deliverables:
- Validate that the embedded browser contract still behaves the same after
  top-level shell convergence.
- Remove obsolete files-local shell helpers and components.
- Update docs if the shared shell architecture becomes part of the files
  service baseline.

Exit criteria:
- `pushkind-files` is aligned to the shared shell architecture for the
  top-level page.
- The embedded browser contract remains intact.
- Remaining local code is file-browser-specific rather than shell-generic.

## Verification
- `npm run format`
- `npm run typecheck`
- `npm run test`
- manual verification of:
  - top-level files page shell behavior
  - embedded browser mount behavior
  - flash messages and fatal states
