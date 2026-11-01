# Export Formats

[Documentation](README.md)

Every export is generated from the same terminal report model using the CLI or desktop app. Results can be shared in readable formats or specialized structured formats.

## Available Formats

| Format ID | Output Type | Description |
| --- | --- | --- |
| `html` | Readable | Local human-readable report. Contains actionable items at the top and technical evidence at the bottom. Preserves all scan facts. |
| `json` | Structured | Machine-readable master report JSON. Preserves the full canonical product report model. |
| `case-bundle` | Archive | Portable case bundle. Includes the selected case data and referenced evidence. Export options decide exactly what evidence is included. Supports integrity signatures. |
| `framework-report` | Structured | Master NIST CSF, ISO/IEC 27001, and AIDEFEND relationship report with incomplete and unknown coverage details. |
| `ocsf` | Structured | OCSF detection findings. Preserves normalized security findings mapped to the OCSF standard. |
| `oscal` | Structured | OSCAL assessment results; outputs coordinates only based on reviewed control references. |

## Integrity and Signatures

Exports like the `case-bundle` can include a locally signed envelope for integrity enhancement. Verification checks the archive structure and hashes, but this verification reports integrity consistency only, not identity assurance or compliance validation.
