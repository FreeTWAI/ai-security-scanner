# Security Scanning Rules

[Documentation](README.md)

The ai-security-scanner application applies security scanning rules based on the type of asset being scanned. The execution boundary is defined when a scan is initiated.

## IT Environment

An IT-environment project can contain repositories, websites, and internal systems. The execution plan binds each asset to applicable checks; one engine never expands its scope to every compatible asset.

Each internal system is one exact hostname or IP address. Its default TCP ports are 22, 23, 25, 80, 443, 445, 3389, 5900, 8080, and 8443. Advanced settings can replace the list with up to 64 exact ports for that host. Inventory ranges remain inventory until individual systems are added and confirmed.

## Website or API

The quick website profile runs against one exact `scheme://host:port` origin. The entered path is retained as context; matching templates can request other paths on the same origin.

Technology detection is performed and applicable vulnerability and exposure templates are selected from the pinned read-only snapshot. It does not authenticate, submit forms or request bodies, follow redirects, use out-of-band callbacks, run headless flows, upload files, fuzz without bounds, or select exploit and denial-of-service templates.

A completed no-finding result records the completed checks and the exact origin. Upstream technology and template applicability determine which templates run.

## Project Folder

The app creates a bounded private snapshot and runs applicable upstream checks for:

- secrets with masked presentation;
- risky code patterns;
- vulnerable dependencies supported by the detected manifests and packages;
- application, deployment, infrastructure, and Kubernetes configuration;
- component inventory used by the report and exports.

The snapshot process follows repository ignore rules for generated, dependency, build, cache, and version-control directories. Common secret-bearing source files—including `.env` variants, private keys, registry or authentication configuration, and `*.tfvars`—remain available to the secret scanners.

The project is not built, executed, uploaded, committed, pushed, or modified.

## Internal System

The generic internal-system profile passes the exact confirmed host and ports. It derives current, non-deprecated, unauthenticated remote `gather_info` vulnerability tests from the pinned Community Feed. It leaves service and product detection, prerequisites, dependencies, severity, evidence, and remediation to the upstream engine.

The profile uses no credentials. It excludes local authenticated checks, default-account and brute-force checks, policy families, alternative port scanners, and attack, denial, destructive, kill-host, and flood categories. It does not add neighboring hosts or undisclosed ports.

Open ports and detected services appear under **Observed services — not vulnerabilities**. A security finding requires an upstream vulnerability result.

## Infrastructure, Containers, Kubernetes, and Cloud

Infrastructure files and Kubernetes manifests are scanned as selected artifacts. Exported container images are inspected without running them. Live cluster and cloud paths use their own selected resource and permission scope.

Cloud authorization uses the provider's official browser flow or an isolated administrative bootstrap request. Scan credentials are never entered in a generic app password field.

## Resource Profile

Profiles freeze applicable rate, concurrency, timeout, output, runtime, and cancellation limits before execution. Scanner-specific identifiers, severities, evidence, and remediation remain upstream-owned. Product-owned prioritization and cross-engine grouping happen in the shared report.
