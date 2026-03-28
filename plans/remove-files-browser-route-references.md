# Plan: Remove Stale Embedded Browser Route References

## Goal
Eliminate stale embedded-browser route references from the repository without
removing the compatibility browser document used by `assets/filebrowser.js`.

## Steps
1. Create a narrow cleanup spec and plan documenting the intended scope.
2. Update README, `SPEC.md`, and feature specs so they describe the embedded
   browser compatibility document rather than the removed embedded-browser
   route.
3. Update migration plans and task docs to remove the literal route reference
   and replace it with compatibility-document wording where needed.
4. Verify with `rg` that the removed embedded-browser route path no longer
   appears in the repository.
5. Run focused checks to confirm the project still builds and the test suite
   remains green.

## Validation
- Run a repo-wide `rg` search for the removed embedded-browser route path.
- `cargo test --all-features --test e2e -- --ignored`
