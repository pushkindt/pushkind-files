# File Card Bottom Alignment

## Status
Accepted

## Date
2026-03-27

## Summary
Keep file card filenames and copy-link controls aligned to the bottom edge of
each card in the React file browser so mixed preview heights do not shift the
label row vertically between entries.

## Requirements
- The shared React file browser MUST keep the filename and copy-link action row
  visually anchored to the bottom of each file card.
- Image previews MAY consume remaining vertical space above the filename row,
  but MUST NOT push the filename row to arbitrary vertical positions.
- The change MUST preserve the existing Bootstrap-based card layout and copy
  link behavior.
- Folder cards and non-image file cards MUST continue to render without
  behavioral changes.
