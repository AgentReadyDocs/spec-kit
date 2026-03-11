# Use Case Template Feedback

Based on writing UC-MIG-001 (Data Migration) — a tier-1 ETL pipeline spec with 10 workflow steps, 12 invariants, and 54 acceptance tests — against `spec-kit/templates/usecase.md`.

**Guiding principle**: Use case specs should be specific enough to reimplement the module without access to existing source code. Only externally observable behavior should be specified — never internal implementation details (data structures, algorithm internals, execution ordering that doesn't affect outcomes).

## What Worked Well

**Sections that provided direct value with minimal friction:**

- **Goal / Scope** — The in/out table forced early boundary decisions (e.g., "offers = stub only" was captured immediately rather than discovered mid-implementation).
- **Interface Contract** — Inputs/Outputs/Side Effects/Idempotency gave the spec a clear "API surface" from the start. The idempotency section was particularly valuable for a migration tool where replay semantics are central.
- **Preconditions** — Small but important; forced documentation of runtime dependencies (staging schema loaded, Visma schema loaded) that would otherwise be tribal knowledge.
- **Invariants and Policies** — The MUST/MUST NOT framing was the single highest-value section for agent consumption. Invariants like "non-migration rows MUST NOT be deleted" became direct test assertions.
- **Acceptance Tests** — Having a dedicated section with IDs created a traceable link between spec rules and test code.
- **Open Questions** — Useful parking lot that prevented scope creep during drafting.

## What Was Weak or Required Modification

### P1 (Critical)

#### 1. Workflow section is over-structured for multi-step pipelines

**Template:** Single table with columns `step | actor/system | action | reads | writes | emits | state_before | state_after`.

**Problem:** This works for a 3-4 step request/response use case (the widget example). For a 10-step ETL pipeline with entity-specific transform rules, the flat table became unusable. Each step needed subsections with prose, mapping tables, and algorithm descriptions (e.g., the 5-step email deduplication algorithm, the 18-step stale deletion order).

**What we did:** Replaced the table entirely with `### Step NNN: Entity Name` subsections using free-form prose, bullet lists, and per-step tables. The `reads | writes | emits | state_before | state_after` columns were dropped.

**Recommendation:** Make the workflow format a guideline, not a mandatory table. Add a note: _"For complex multi-step workflows, use numbered subsections (`### Step NNN`) with domain-appropriate structure. The table format is recommended for linear request/response flows."_ The rubric (UC-CF-003, UC-S-007) should accept either format as long as state transitions are coherent.

**Rationale:** Forcing transform-heavy business logic into a reads/writes/emits table creates information loss. The template should serve the spec, not the other way around.

---

#### 2. No section for verification / post-operation checks

**Template:** No section exists between workflow and error catalog for describing operational verification.

**Problem:** Our migration tool has a `verify` command that runs 5 integrity checks post-migration (row counts, completeness, FK/PK integrity, Parquet-vs-DB counts). This is a first-class behavioral contract — it determines exit code 0 vs 1 — but has no natural home in the template. It's not a workflow step (it's a separate command), not an acceptance test (it's runtime behavior), and not an error (it produces errors).

**What we did:** Added a `## Verification` section between Workflow and Error Catalog.

**Recommendation:** Add an optional `## Verification / Post-Conditions` section to the template after Workflow. Template content:

```
## Verification / Post-Conditions (If Applicable)
| check_id | check | description | pass_criteria |
|----------|-------|-------------|---------------|
| VER-001 | [check name] | [what it verifies] | [when it passes] |
```

**Rationale:** Many real systems have verification steps (health checks, data integrity checks, reconciliation). Preconditions capture "before"; there should be a symmetric section for "after."

---

#### 3. Actors section is wrong for non-interactive systems

**Template:** `| actor_id | role | permissions |` — assumes human actors with roles and permissions.

**Problem:** Our use case has no human actors at runtime. It's a CLI tool invoked by a developer. The "actor" is the migration script itself, and there are no AuthZ rules. Both Actors and AuthZ sections were dropped entirely because they added zero information.

**What we did:** Omitted both sections.

**Recommendation:** Mark Actors and AuthZ as conditional sections with guidance: _"Omit for batch/CLI/ETL use cases where there is no runtime authorization. Include `N/A` in front matter if the rubric requires it."_ The rubric (UC-S-003, UC-S-005) should allow SKIP for non-interactive systems.

**Rationale:** Forcing empty Actor/AuthZ tables into batch processing specs creates noise. The template implicitly assumes request/response web services.

---

### P2 (Important)

#### 4. Entity mapping table is too generic for ETL/migration use cases

**Template:** `| entity | identifier | notes |` — designed for referencing a glossary.

**Problem:** Our core complexity is source-to-target entity mapping with transforms. The template's entity table doesn't capture the mapping direction (source -> target) or the cardinality (1:N, N:1). We replaced it with a `| Source Entity | Target Entity |` table and linked to a separate glossary.

**Recommendation:** Add a variant for data transformation use cases:

```
| source_entity | target_entity | cardinality | notes |
|---------------|---------------|-------------|-------|
```

Or document in the template that the entity table format should be adapted to the domain.

**Rationale:** ETL and migration specs are a common use case category. The entity section is where the most important domain context lives, and it needs to show the mapping, not just list entities.

---

#### 5. Acceptance Tests table is too narrow for large test suites

**Template:** `| scenario_id | given | when | then | validates | test_ref |`

**Problem:** With 54 tests, the given/when/then columns became repetitive boilerplate ("Source data loaded" / "Migration runs" / "Row created"). The `validates` column (AC1, AC2) assumes acceptance criteria are defined elsewhere, but our invariants serve that role. The 6-column table was replaced with a 3-column `| ID | Description | Test Reference |` table organized by category.

**Recommendation:** Allow two formats:
- **Detailed format** (current) for specs with <15 tests: full given/when/then.
- **Catalog format** for specs with 15+ tests: `| scenario_id | description | test_ref |` grouped by category with subsection headers.

Add a note: _"For large test suites, use the catalog format with category subsections. The detailed format is preferred when test count is small enough for each scenario to be read individually."_

**Rationale:** The purpose of the acceptance tests section is traceability (spec rule -> test). For large suites, the given/when/then columns reduce readability without adding information that isn't already in the test name.

---

#### 6. Error Catalog is over-specified for simple error models

**Template:** `| error_code | condition | detection_point | retryable | client_action | status | message | telemetry_fields |` — 8 columns.

**Problem:** Our CLI tool has 4 error codes, all exit code 1, none retryable, no HTTP status, no telemetry fields. The 8-column table was replaced with a 4-column `| ID | Condition | Exit Code | Message |` table. Half the template columns were N/A.

**Recommendation:** Distinguish between API-facing and non-API error catalogs. Add guidance: _"For CLI tools and batch processes, the minimal columns are: `error_code | condition | exit_code | message`. The full table is intended for API services."_

**Rationale:** The current table is designed for HTTP APIs. Forcing CLI tools into `status | client_action | telemetry_fields` columns produces empty cells that obscure the actual error behavior.

---

#### 7. No section for "Not Migrated" / explicit exclusions

**Template:** Scope has Out of Scope, but no section for _deliberate_ omissions from within in-scope entities.

**Problem:** We needed to document fields and mappings that were considered but deliberately skipped (e.g., "Activity.InterpreterGender is NOT mapped to prebookings.skill_id — gender skill is at proficiency level"). These are different from out-of-scope items: they are within scope but intentionally not implemented, and the reason matters.

**What we did:** Added a `## Not Migrated` section listing each omission with rationale.

**Recommendation:** Add an optional `## Deliberate Omissions` section after Acceptance Tests:

```
## Deliberate Omissions (If Applicable)
| item | rationale |
|------|-----------|
| [field/entity/mapping] | [why it was excluded] |
```

**Rationale:** Without this section, agents and future developers will repeatedly investigate why obvious mappings are missing. Documenting "we thought about it and decided no" prevents wasted analysis cycles.

---

#### 8. Template does not guide authors toward observable behavior over implementation

**Template:** No guidance anywhere in the template, rubric, or rubric-guidance about the distinction between externally observable behavior and internal implementation details.

**Problem:** During authoring, implementation details repeatedly leaked into the spec. Two examples from UC-MIG-001 that required correction:

- _Stale cleanup deletion order_: An 18-step numbered list specified the exact sequence of DELETE/UPDATE operations to avoid FK constraint errors. This is an execution strategy, not behavior. The observable behavior is: "when entity X becomes stale, children Y are deleted and nullable FK Z is set to NULL." A reimplementer can achieve the same outcome in any order (e.g., deferred constraints, single CTE, different sequencing).
- _"Junction tables use delete + reimport pattern (not upsert)"_: This describes a mechanism. The observable behavior is: "junction links are fully refreshed each run; manually-added links for migration-owned rows are not preserved."

Without explicit guidance, spec authors (both human and AI) default to describing _how_ they built it rather than _what_ it does.

**Recommendation:** Add a guidance block at the top of the template (below the review-against-rubric note):

> _Specify only externally observable behavior: inputs, outputs, state changes, and invariants. Do not specify internal execution order, data structures, algorithm internals, or mechanism choices unless they affect observable outcomes. The spec should be sufficient to reimplement the module without access to existing source code._

Also add a rubric check (scored or critical fail): _"Workflow and side-effect descriptions specify observable outcomes, not internal implementation strategies (e.g., execution order, algorithmic mechanisms, internal data structures)."_

**Rationale:** This is the single most common spec quality problem. Implementation details create false constraints (a reimplementer follows the specified order when any correct order would do), make specs harder to maintain (implementation changes require spec updates even when behavior hasn't changed), and obscure the actual behavioral contract under mechanical noise.

---

### P3 (Minor)

#### 9. Observability section is irrelevant for offline tools

**Template:** `| signal | required_fields | sampling | redaction |` with logs/metrics/traces rows.

**Problem:** A CLI migration tool has no metrics, no distributed traces, no sampling rates. We omitted the section entirely.

**Recommendation:** Mark as conditional: _"Omit for offline/CLI tools. Required for services with production traffic."_

**Rationale:** Low impact but adds to the "this template is for web APIs" impression.

---

#### 10. Front matter schema is too rigid

**Template/Schema:** `id` must match `^UC-[0-9]{4}$`. We used `UC-MIG-001` which fails validation.

**Recommendation:** Relax the pattern to `^UC-[A-Z0-9-]+$` to allow domain-prefixed IDs.

**Rationale:** When a project has multiple use case categories, prefixed IDs (`UC-MIG-001`, `UC-SYNC-001`) are more readable than sequential numbers.

---

#### 11. Open Questions table is missing `status` column

**Template:** `| open_id | question | owner | due | impact |`

**Problem:** We simplified to `| ID | Question |` because owner/due/impact were unknown for all our questions. But the bigger gap is that there's no `status` column. Questions get resolved over time and the resolution should be captured in-place rather than deleting the row (which loses the decision record).

**Recommendation:** Add a `status` column (`open | resolved | deferred`) and an optional `resolution` column:

```
| open_id | question | status | resolution | owner | due | impact |
```

**Rationale:** Resolved questions are valuable context — they record decisions. Deleting them loses institutional knowledge; keeping them without status creates ambiguity about whether they're still open.

---

## Summary

| # | Change | Priority | Category |
|---|--------|----------|----------|
| 1 | Allow free-form workflow subsections for complex pipelines | P1 | Structure |
| 2 | Add Verification / Post-Conditions section | P1 | Omission |
| 3 | Make Actors and AuthZ conditional (skip for non-interactive) | P1 | Applicability |
| 4 | Add source->target entity mapping variant for ETL | P2 | Structure |
| 5 | Add catalog format for large acceptance test suites | P2 | Scalability |
| 6 | Simplify Error Catalog for non-API use cases | P2 | Applicability |
| 7 | Add Deliberate Omissions section | P2 | Omission |
| 8 | Add guidance: observable behavior only, no implementation details | P1 | Correctness |
| 9 | Mark Observability as conditional | P3 | Applicability |
| 10 | Relax front matter ID pattern | P3 | Schema |
| 11 | Add status/resolution to Open Questions | P3 | Structure |

**Overall assessment:** The template is strong for API-service use cases (the widget example fits perfectly). Two systemic gaps emerged:

1. **Applicability** — The template assumes a request/response interaction model. Batch jobs, ETL pipelines, and CLI tools need conditional sections and format alternatives. Three P1 issues (workflow rigidity, missing Actors skip, missing verification) stem from this.

2. **Observable behavior vs implementation** — Neither the template nor the rubric guides authors toward specifying _what_ the system does (observable outcomes) rather than _how_ it does it (execution mechanics). This is the most fundamental quality issue for reimplementability and was found in practice during UC-MIG-001 authoring.
