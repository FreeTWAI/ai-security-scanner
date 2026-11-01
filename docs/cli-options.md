# CLI Options

[Documentation](README.md)

The `ai-security-scanner-cli` binary supports local-first security assessment casework operations from the command line.

## Global Options

| Option | Description |
| --- | --- |
| `--data-dir <DATA_DIR>` | Override the local application data directory. Environment variable: `AI_SECURITY_SCANNER_DATA_DIR`. |
| `--managed-runtime-bundle <MANAGED_RUNTIME_BUNDLE>` | Override the release-managed runtime bundle directory. Environment variable: `AI_SECURITY_SCANNER_MANAGED_RUNTIME_BUNDLE`. |
| `--release-approved-local-image-directory <DIR>` | Use digest-keyed OCI archives from a release-approved local image directory. Environment variable: `AI_SECURITY_SCANNER_RELEASE_APPROVED_LOCAL_IMAGE_DIRECTORY`. |
| `--json` | Emit compact machine-readable JSON output for commands. |
| `-h, --help` | Print help information. |
| `-V, --version` | Print version information. |

## Subcommands

### `scan`
Plan and inspect scans. Planning never starts a scanner process.
- `plan`: Persist an immutable, credential-free dispatch plan. Does not execute it.
- `start`: Request execution without accepting credentials on the command line.
- `rescan-plan`: Persist a new plan tied to a terminal baseline. Does not execute it.
- `status`: Show the status of a run.
- `pause`: Pause execution.
- `resume`: Resume paused execution.
- `cancel`: Request cancellation for a persisted plan.

### `export`
Create and verify explicit local exports.
- `create`: Create a new export artifact.
- `verify`: Verify an existing export bundle.
- `identity`: Show or establish identity for local signed exports.
- `formats`: List available export formats.

### `compare`
Compare two terminal runs and persist the coverage-aware result.
- Requires `--case-id <CASE_ID>`, `--baseline-run-id <BASELINE_RUN_ID>`, and `--current-run-id <CURRENT_RUN_ID>`.

### `bootstrap`
Inspect secret-free provider bootstrap cleanup obligations.
- `cleanup-list`: List bounded cleanup summaries for one exact case.
- `cleanup-show`: Show one cleanup summary without exposing resource IDs or endpoints.

### `doctor`
Check local storage, catalog, and container runtime readiness. No subcommand required.
