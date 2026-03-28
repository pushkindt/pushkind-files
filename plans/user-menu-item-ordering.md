# Plan: User Menu Item Ordering

## Scope
Make the files page user dropdown ordering explicit so local links render
before fetched auth menu entries and logout remains last.

## Steps
1. Refactor the React user menu component to compose local and fetched menu
   items in a single ordered list.
2. Filter fetched logout-style entries so the local logout action remains the
   only trailing logout control.
3. Add a frontend render test that verifies local-first ordering and logout
   placement, then run targeted frontend checks.
