# ARSF Versioning

ARSF versioning is separate from the `ard` tool version.

## ARSF Standard Version

- The standard uses semantic versioning: `MAJOR.MINOR.PATCH`.
- MAJOR increments when conformance rules change in a way that can break existing compliant docsets.
- MINOR increments when new optional fields or new doc types are introduced without breaking existing compliant docsets.
- PATCH increments for clarifications and non-behavioral fixes.

## `ard` Tool Version

- `ard` uses its own semantic version.
- `ard` SHOULD report both `tool_version` and `standard_version` in machine-readable output.

## Deprecations

- Deprecations MUST be documented with a target removal version and a migration path.
- The validator SHOULD continue accepting deprecated constructs for at least one MINOR release cycle when feasible.

