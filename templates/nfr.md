---
id: NFR-BASELINE
type: nfr
title: "[System / Product] NFR Baseline"
owner: "@owner"
last_updated: YYYY-MM-DD
---

# NFR Baseline: [System / Product]

## Performance
| area | metric | target | measurement_point | enforcement |
|------|--------|--------|-------------------|------------|
| api | latency_p99 | [e.g. < 200ms] | [edge -> response] | [slo gate / load test] |
| api | throughput_sustained | [e.g. 1000 rps] | [gateway] | [capacity test] |

## Availability
| component | target | downtime_budget | measurement | enforcement |
|----------|--------|-----------------|-------------|------------|
| overall | [e.g. 99.9%] | [e.g. 43 min/mo] | [synthetic checks] | [release gate] |

## Reliability
| concern | requirement | target | measurement | enforcement |
|---------|-------------|--------|------------|------------|
| retries | retry_policy | [declared per external call] | [code review] | [lint] |
| idempotency | idempotency_required | [Y/N by operation class] | [tests] | [gate] |

## Security
| area | requirement | target | measurement | enforcement |
|------|-------------|--------|------------|------------|
| auth | session_timeout | [e.g. 30 min idle] | [config audit] | [release gate] |
| crypto | encryption_in_transit | [e.g. TLS 1.2+] | [scan] | [gate] |

## Data Handling
| rule_id | rule (MUST / MUST NOT) | scope | enforcement |
|---------|-------------------------|-------|------------|
| DATA-001 | MUST NOT log PII, credentials, or secrets | all services | [lint / review] |
| DATA-002 | MUST define retention for persisted data | db + logs | [policy gate] |

## Observability
| signal | minimum | required_fields | enforcement |
|--------|---------|-----------------|------------|
| logs | [structured] | [correlation_id, uc_id, actor_id?] | [review] |
| metrics | [core dashboards] | [slo metrics] | [gate] |
| traces | [service boundaries] | [correlation_id] | [review] |

## Change Control
| change_type | required_artifacts | required_approvals |
|------------|--------------------|--------------------|
| schema change | [UC update, ADR if needed] | [reviewers] |
| authz change | [UC update, ADR] | [reviewers] |

## Risk Tiers (Doc Gates)
| tier | triggers (if any apply) | required_docs | required_uc_fields | required_reviews |
|------|--------------------------|---------------|--------------------|------------------|
| tier0 | docs-only changes; no state change; no external calls | UC-lite acceptable | error handling table; ACs | 1 reviewer |
| tier1 | internal state write; non-sensitive data; no irreversible effects | UC required; NFR baseline applies | interface contract; low-variance workflow trace; typed errors; scenario table | 1 reviewer |
| tier2 | external calls; authz changes; schema/contract changes; money movement | UC required; ADR required when decisionful | idempotency required or explicit N/A; edge cases table; invariants table | 2 reviewers |
| tier3 | irreversible external effects; regulated data scope; broad blast radius | UC required; ADR required | full scenario coverage; explicit rollback/mitigation in UC/ADR | 2 reviewers + designated approver |
