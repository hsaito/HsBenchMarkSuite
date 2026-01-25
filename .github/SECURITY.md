# Security Policy

Thank you for helping keep HsBenchMarkSuite secure.

## Reporting a Vulnerability

Please submit confidential reports through GitHub Security Advisories:

https://github.com/hsaito/HsBenchMarkSuite/security/advisories/new

Do not open public issues for security vulnerabilities. Include:
- Affected version/commit and platform (OS, architecture)
- Reproduction steps and expected/actual behavior
- Impact assessment and any suggested remediation

We will acknowledge receipt and coordinate a fix or mitigation before public disclosure.

## Supported Versions

Security fixes are generally targeted at:
- The latest released version
- The `main` branch

Older versions may not receive patches.

## Disclosure and Timelines

This is a community project maintained on a best-effort basis. There is no SLA for response times or releases. We aim to:
- Triage within 7 days when possible
- Provide a fix or mitigation as capacity allows
- Publish an advisory and release notes once a resolution is available

## Credit and Acknowledgements

We appreciate responsible disclosure. If you would like recognition, let us know how to credit you in the advisory.

## Dependencies

We rely on automated updates (e.g., Dependabot) for third‑party dependencies. If the issue is in a dependency, please reference the upstream advisory or issue when reporting.

## Automated Security Testing

- Dependency vulnerability scanning via `cargo audit` in CI to flag known issues in Rust crates.
- GitHub Dependabot monitors Rust dependencies and GitHub Actions and opens upgrade PRs when advisories are published.
- Additional security tooling (e.g., SAST or license checks) may be added as the project grows and resources allow.
