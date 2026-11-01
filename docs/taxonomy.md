# Vulnerability Classification Taxonomy

[Documentation](README.md)

The ai-security-scanner categorizes findings into a specific classification taxonomy based on the type of issue discovered. This allows for grouping and routing of related concerns.

## Finding Families

| Family | Description |
| --- | --- |
| **CloudPosture** | Cloud environment configuration and posture issues. |
| **CloudIdentity** | Cloud IAM and identity permissions configuration. |
| **Microsoft365** | Microsoft 365 tenant and service configuration. |
| **NetworkExposure** | Network-reachable exposures or exposed services. |
| **SourceCode** | Risky code patterns and static source analysis results. |
| **Secret** | Hardcoded secrets, private keys, or credential patterns in source or configuration. |
| **InfrastructureAsCode** | Misconfigurations in infrastructure declarations (e.g. Terraform, CloudFormation). |
| **VulnerableComponent** | Vulnerable dependencies, libraries, or OS packages. |
| **Kubernetes** | Kubernetes cluster and workload configuration issues. |
| **ModelBehavior** | A deployed model answered an adversarial prompt the way the probe was trying to make it answer. |
| **McpSecret** | MCP configuration text contains a credential pattern. |
| **McpConfiguration** | A static MCP configuration grants a server command or tool broader capabilities than it needs. |
