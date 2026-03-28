# Remove Stale Embedded Browser Route References

## Status
Accepted

## Date
2026-03-28

## Summary
Remove stale references to the nonexistent embedded browser route from the
repository while preserving the compatibility browser document at
`/assets/dist/app/browser.html` that is still used by `assets/filebrowser.js`.

## Problem
The backend no longer exposes the legacy embedded browser route, but the
repository still describes that route in product specs, plans, and README
content. That creates drift between the documented surface area and the live
application.

## Goals
- Remove every repository mention of the legacy embedded browser route path.
- Keep `frontend/app/browser.html` and the compatibility embedding flow used by
  `assets/filebrowser.js`.
- Update documentation so it describes the browser compatibility document and
  mount contract accurately.

## Non-Goals
- Removing `browser.html`.
- Removing `assets/filebrowser.js`.
- Reworking the compatibility embedding API.

## Requirements
- The repository MUST NOT contain the legacy embedded browser route path after
  the cleanup.
- README and product specs MUST describe the embedded browser surface in terms
  of the compatibility document or mount contract instead of a backend route.
- Existing `browser.html` build inputs and compatibility assets MUST remain
  intact.
