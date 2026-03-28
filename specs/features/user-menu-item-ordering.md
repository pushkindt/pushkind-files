# User Menu Item Ordering

## Status
Accepted

## Date
2026-03-28

## Summary
Keep the files page user dropdown ordered so locally defined navigation entries
appear before entries fetched from the auth menu API, while the local logout
action remains the final item regardless of fetched menu contents.

## Requirements
- The files page user dropdown MUST render local navigation items before any
  entries fetched from the hub menu API.
- Fetched hub menu items MUST render after local navigation items.
- Logout MUST remain the final dropdown action even when the fetched menu
  payload includes a logout entry.
- The dropdown MUST continue to expose the local POST `/logout` action rather
  than replacing it with a fetched link.
