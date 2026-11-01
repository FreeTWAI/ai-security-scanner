# Severity Matrix

[Documentation](README.md)

The product normalizes upstream scanner severities into a standard representation while preserving the original source severity when provided.

## Normalized Severities

The report layer presents the following severity categories:

| Severity | Description |
| --- | --- |
| **Critical** | Critical severity condition on the assessed asset. |
| **High** | High severity condition on the assessed asset. |
| **Medium** | Medium severity condition on the assessed asset. |
| **Low** | Low severity condition on the assessed asset. |
| **Informational** | Informational context on the assessed asset. |
| **Unknown** | The upstream scanner did not provide a recognized impact rating. This is explicitly marked as Unknown rather than treated as informational or low. |

## Upstream Severity Preservation

The product preserves original scanner identity, rule ID, version, severity, location, evidence reference, timestamps, and check outcome.
The raw upstream severity word is kept verbatim in the technical details (e.g. `Source severity: ...`).

## Unknown Severity Basis

When an upstream scanner provides a finding without a parseable or defined severity, the report marks the severity as **Unknown**. The product assigns a derived explanation basis, such as:

- a failed infrastructure-as-code policy check, rated flat because Checkov publishes no per-check severity offline.
- a Greenbone vulnerability-test alarm whose pinned feed entry carries no parseable severity vector.
- a count of adversarial probe attempts a garak detector judged as failures, which garak publishes without a severity.
- an open port or reachable HTTP service (classified as exposure/inventory).
