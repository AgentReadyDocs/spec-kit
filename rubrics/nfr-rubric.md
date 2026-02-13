# NFR Baseline Rubric

- Total points: 100
- Pass threshold: >= 70
- Decision rule: FAIL if any Critical Fail is FAIL; else PASS if score >= 70.

## Critical Fails (Must PASS)

- NFR-CF-001 (all): No qualitative-only requirements (e.g., "fast/secure/scalable") without a numeric target or an explicit measurement method.
  - Evidence: Performance / Availability / Reliability / Security tables (target + measurement columns).
- NFR-CF-002 (all): Every metric row has a numeric `target` and a non-empty `measurement_point` (or measurement method) showing where/how it is measured.
  - Evidence: `## Performance`, `## Availability`, `## Reliability`, `## Security`.
- NFR-CF-003 (all): Data handling contains at least one `MUST` and one `MUST NOT` rule with scope and enforcement.
  - Evidence: `## Data Handling`.
- NFR-CF-004 (tier2+): Includes explicit breach/consequence handling for at least availability and error budget (what happens when targets are violated).
  - Evidence: `## Availability` (notes/enforcement) and/or a dedicated policy row.

## Scored Checks (100 points)

| check_id | points | applies_to | pass_criteria | evidence |
|---|---:|---|---|---|
| NFR-S-001 | 15 | all | Performance covers latency and throughput (or equivalent) with numeric targets and measurement points. | `## Performance` |
| NFR-S-002 | 12 | all | Availability includes an explicit target and a downtime/error budget and states how it is measured. | `## Availability` |
| NFR-S-003 | 10 | all | Reliability includes explicit retry policy and idempotency expectations by operation class (or explicit N/A). | `## Reliability` |
| NFR-S-004 | 15 | all | Security requirements are concrete and testable (config/scan/audit), not narrative prose. | `## Security` |
| NFR-S-005 | 10 | all | Data handling rules are scoped and include enforcement (lint/review/test/runtime). | `## Data Handling` |
| NFR-S-006 | 10 | all | Observability defines minimum signals and required fields needed for incident triage. | `## Observability` |
| NFR-S-007 | 8 | all | Change control specifies required artifacts and approvals for high-risk change types. | `## Change Control` |
| NFR-S-008 | 10 | all | Risk tiers table exists and matches the project’s tier definitions (tier0..tier3 with gates). | `## Risk Tiers (Doc Gates)` |
| NFR-S-009 | 10 | all | Each major requirement row includes an `enforcement` method that would catch violations before or during runtime. | enforcement columns across sections |

