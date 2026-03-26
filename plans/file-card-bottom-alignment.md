# Plan: File Card Bottom Alignment

## Scope
Adjust the shared React file browser card layout so filenames sit on the bottom
edge of file cards regardless of preview height.

## Steps
1. Inspect the shared file card component and stylesheet to locate the current
   vertical alignment behavior.
2. Update the card markup/CSS so preview content expands and the filename row
   is pushed to the bottom consistently.
3. Rebuild the frontend bundle and verify the relevant frontend checks still
   pass.
