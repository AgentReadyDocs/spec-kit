---
id: NFR-0001
type: nfr
title: "Widget Service NFR Baseline"
owner: "@example"
last_updated: 2026-02-12
---

# NFR Baseline: Widget Service

## Performance
| area | metric | target | measurement_point | enforcement |
|------|--------|--------|-------------------|------------|
| api | latency_p99 | < 200ms | gateway → response | load test gate |
| api | throughput_sustained | >= 300 rps | gateway | capacity test |

## Availability
| component | target | downtime_budget | measurement | enforcement |
|----------|--------|-----------------|-------------|------------|
| overall | 99.9% | 43 min/mo | synthetic checks + SLO dashboards | release gate |

## Reliability
| concern | requirement | target | measurement | enforcement |
|---------|-------------|--------|------------|------------|
| retries | retry_policy | max 2 retries with exponential backoff for 5xx/timeouts | code review + tests | review gate |
| idempotency | idempotency_required | required for create operations; N/A for read-only | contract tests | gate |

## Security
| area | requirement | target | measurement | enforcement |
|------|-------------|--------|------------|------------|
| auth | session_timeout | 30 min idle | config audit | release gate |
| crypto | encryption_in_transit | TLS 1.2+ | automated scan | gate |

## Data Handling
| rule_id | rule (MUST / MUST NOT) | scope | enforcement |
|---------|-------------------------|-------|------------|
| DATA-001 | MUST NOT log PII, credentials, or secrets | all services | lint + review |
| DATA-002 | MUST define retention for persisted data and logs | db + logs | policy gate |

## Observability
| signal | minimum | required_fields | enforcement |
|--------|---------|-----------------|------------|
| logs | structured | correlation_id, uc_id, workspace_id, error_code? | review |
| metrics | core dashboards | request_latency_p99, error_rate, saturation | gate |
| traces | service boundaries | correlation_id | review |

## Change Control
| change_type | required_artifacts | required_approvals |
|------------|--------------------|--------------------|
| schema change | UC update, ADR if decisionful | 2 reviewers |
| authz change | UC update, ADR | 2 reviewers |

## Risk Tiers (Doc Gates)
| tier | triggers (if any apply) | required_docs | required_uc_fields | required_reviews |
|------|--------------------------|---------------|--------------------|------------------|
| tier0 | docs-only changes; no state change; no external calls | UC-lite acceptable | error handling table; ACs | 1 reviewer |
| tier1 | internal state write; non-sensitive data; no irreversible effects | UC required; NFR baseline applies | interface contract; deterministic workflow; typed errors; scenario table | 1 reviewer |
| tier2 | external calls; authz changes; schema/contract changes; money movement | UC required; ADR required when decisionful | idempotency required or explicit N/A; edge cases table; invariants table | 2 reviewers |
| tier3 | irreversible external effects; regulated data scope; broad blast radius | UC required; ADR required | full scenario coverage; explicit rollback/mitigation in UC/ADR | 2 reviewers + designated approver |

