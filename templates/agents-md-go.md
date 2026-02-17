# AGENTS.md (Go) — starter template

Use this as a starting point for a new Go repository, then tailor to your tooling and conventions.

## CRITICAL

- MUST: Keep `go.mod` / `go.sum` consistent with CI.
- MUST: Run `lint` and `test` before PR.
- NEVER: Commit secrets or credentials.

## Env

- Go: `>=<fill>`
- Linter: `golangci-lint` (recommended)

## Commands

```bash
# install
go mod download
# lint
golangci-lint run
# test
go test ./...
```

## Notes

- If you use `mage`, `task`, or `make`, prefer a single entry-point command (`make lint`, `task test`, etc.).

