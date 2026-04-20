# Shared Shell Architecture Alignment

## Status
Proposed

## Date
2026-04-13

## Summary
Align `pushkind-files` with the shared React shell architecture used by
`pushkind-crm`, `pushkind-emailer`, `pushkind-todo`, and `pushkind-orders`.
This work MUST preserve the existing files UI, embedded browser contract, and
route behavior while converging on the same navbar, shell, flash, fatal-state,
and shell data architecture through `pushkind-common/frontend`.

## Problem
`pushkind-files` was migrated to React with a file-browser-specific shell model
that still differs from the common hub shell used by the other services. That
difference makes the files service a structural outlier, reduces UX
consistency, and limits how much frontend code can be centralized in
`pushkind-common`.

## Goals
- Converge `pushkind-files` on the same shared shell architecture as the other
  migrated services.
- Preserve the current files page UI, embedded browser contract, and Bootstrap
  styling.
- Reuse shared frontend shell modules from `@pushkind/frontend-shell`.
- Keep the top-level files page and embedded browser server-routed and non-SPA.
- Make navbar, shell, flash, fatal-state, no-access, and shell data handling
  consistent across services.

## Non-Goals
- Redesigning the file browser.
- Breaking or removing the embedded browser mount contract.
- Moving file authorization, path handling, or storage logic into the client.
- Introducing client-side routing or changing the files page URLs.

## In Scope
- Files full-page shell architecture.
- Files top navbar/user-menu architecture.
- Files flash presentation and fatal-state handling.
- Files shell data loading and shared shell type usage where contracts match.
- Files no-access or shell-denied page architecture if applicable.
- Alignment of the full-page files shell with the shared Pushkind hub shell.

## Out Of Scope
- File-browser resource rendering and entry-card internals that are already
  files-specific.
- New file operations or embedded-browser API changes unrelated to shell
  alignment.
- Rust service-layer refactors unrelated to frontend shell delivery.

## Functional Requirements

### 1. Shell Convergence
- `pushkind-files` MUST use the same shared shell architecture primitives as
  the other migrated services where the contracts now match.
- Shared primitives SHOULD come from `pushkind-common/frontend`.
- The resulting files shell MUST preserve the current visual output for the
  top-level page and MUST NOT break embedded browser behavior.

### 2. Navbar Convergence
- The files top navigation MUST converge on the shared navbar component shape.
- Differences such as service label, search behavior, and files-specific local
  menu items MUST be expressed through props rather than a separate navbar
  implementation.
- Existing navbar HTML/CSS semantics MUST remain visually equivalent.

### 3. Flash Behavior
- Top-level files flash presentation MUST converge on the same behavior used by
  the other migrated services.
- React-owned async actions on the main files page MUST surface messages
  through the shared shell flash mechanism.
- Existing Bootstrap alert styling MUST remain intact.

### 4. Fatal State And No-Access Handling
- Fatal shell failures MUST render through the shared fatal-state primitive.
- If files has a no-access or shell-denied page flow, it MUST use the shared
  no-access architecture.
- Files-specific copy MAY remain if the shared primitives support it through
  props.

### 5. Embedded Browser Boundary
- The embedded browser contract MUST remain supported.
- Full-page shell convergence MUST NOT force the embedded browser into a shell
  model that breaks embedding or host-page integration.
- Shared shell extraction MAY apply only to the top-level page if the embedded
  browser intentionally remains a lighter-weight mount surface.

### 6. Shell Data APIs
- Shared shell data loading MUST use the same frontend helper model as the
  other migrated services where the payload contracts match.
- `pushkind-files` MUST rely on shared shell parsing/fetch helpers where the
  top-level page contract is compatible.
- File-browser-resource parsing MAY remain local.

### 7. UI Preservation
- Existing full-page files UI MUST remain substantially unchanged.
- Existing embedded browser UI MUST remain substantially unchanged.
- Markup drift is not acceptable merely to make the shell abstractions fit.

## Acceptance Criteria
- `pushkind-files` uses the shared navbar architecture for the top-level page.
- `pushkind-files` uses the shared flash delivery pattern for the top-level
  page.
- `pushkind-files` uses shared fatal-state and shell helpers where contracts
  match.
- The files frontend imports shared shell modules from
  `@pushkind/frontend-shell` instead of keeping local duplicates.
- The embedded browser contract still works.
- Visual output remains substantially unchanged.

## Risks
- `pushkind-files` has a stronger embedded-browser constraint than the other
  services, so convergence must not leak full-page shell assumptions into the
  embedded path.
- The current files shell may carry file-browser-specific lifecycle behavior
  that must stay local even after top-level shell alignment.
