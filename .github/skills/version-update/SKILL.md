---
name: Version Update
description: Update version references consistently across the project.
---

# Skill: Version Update

## Purpose
Update version references consistently across the project.

## Scope
- Cargo.toml (package version)
- README.md (badges or version text)
- CHANGELOG.md (release entry)
- Any other files that reference a version string

## Inputs
- Target version (e.g., 1.2.3)
- Release date (YYYY-MM-DD) if CHANGELOG uses dates
- Short summary of changes for CHANGELOG

## Steps
1. Update Cargo.toml:
   - Set package version to target version.
2. Update README.md:
   - Replace displayed version badges or text references with the target version.
   - Ensure no stale version numbers remain.
3. Update CHANGELOG.md:
   - Add a new entry for the target version at the top.
   - Include release date if required.
   - Summarize changes briefly.
4. Check other references:
   - Update any docs or configs that mention the version.
5. Verify consistency:
   - Ensure the same version appears everywhere.
   - Ensure formatting matches existing conventions.

## Output
- All version references updated consistently.
- README.md and CHANGELOG.md reflect the new version.
