---
id: COVERAGE-MATRIX
type: coverage
title: "Capability Coverage Matrix"
owner: "@owner"
last_updated: YYYY-MM-DD
---

# Capability Coverage Matrix

> Maps extracted capabilities to use cases. Use this template for rewrite/migration projects where
> capabilities have been extracted from an existing system and use cases are being designed for the rewrite.
>
> **Invariant**: Every capability should appear in at least 1 UC.

## Summary
| domain | total_capabilities | covered | uncovered | coverage_pct |
|--------|-------------------:|--------:|----------:|-------------:|
| [Domain Name] | [N] | [N] | [N] | [N%] |

## Coverage by Domain

### [Domain Name]
| capability_id | name | uc_ids |
|---------------|------|--------|
| [CAP-001] | [capability name] | [UC-0101, UC-3001] |

## Uncovered Capabilities
| capability_id | name | domain | suggested_uc | notes |
|---------------|------|--------|--------------|-------|
| [CAP-999] | [uncovered capability] | [domain] | [UC-XXXX] | [why uncovered / implicitly covered] |

---

## Essential Complexity Coverage

> Maps essential complexity items to use cases that preserve them.
> **Invariant**: Every essential complexity item should appear in at least 1 UC.

| complexity_id | description | uc_ids | status |
|---------------|-------------|--------|--------|
| [E-01] | [description] | [UC-3001, UC-0205] | covered / gap |

### Gaps
<!-- List any E-IDs not referenced in any UC -->

---

## Incidental Complexity Elimination

> Maps incidental complexity items to use cases that eliminate them.
> **Target**: Most incidental complexity items should be addressed in at least 1 UC.

| incidental_id | description | uc_ids | status |
|---------------|-------------|--------|--------|
| [I-01] | [description] | [UC-3001, UC-0201] | addressed / unaddressed |

### Unaddressed
<!-- List any I-IDs not addressed in any UC, with rationale or suggested action -->
| incidental_id | description | suggested_action |
|---------------|-------------|------------------|
| [I-99] | [description] | [action or "deferred — rationale"] |
