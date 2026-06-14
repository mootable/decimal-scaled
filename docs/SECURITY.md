# Security Policy

## Supported Versions

Security fixes are applied to the latest released minor version on
crates.io. Older minor versions are not patched; downstream consumers
are expected to upgrade.

## Reporting a Vulnerability

Report suspected security issues **privately** via GitHub Security
Advisories:

  https://github.com/mootable/decimal-scaled/security/advisories/new

If you cannot use GitHub Security Advisories, open a regular
[GitHub issue](https://github.com/mootable/decimal-scaled/issues) with a
brief description — leave out exploit specifics — and the maintainer will
follow up to arrange a private channel.

Please include:

- A description of the vulnerability and its impact.
- Steps to reproduce, including a minimal reproducer if possible.
- The affected version(s).

You can expect an initial acknowledgement within 7 days. Coordinated
disclosure timelines are agreed case by case once the report is
triaged.

## Scope

In scope:

- Memory-safety issues in the `decimal-scaled` crate.
- Numerical correctness issues that could lead to a deterministic
  exploit downstream (e.g., a wide-tier kernel returning a value
  outside the documented `≤ 0.5 ULP` contract).
- Build-time supply-chain issues in the published crate.

Out of scope:

- Performance regressions (use a regular GitHub issue).
- Documentation errors (use a regular GitHub issue).
- Issues in dependencies — please report those upstream and to
  RustSec (`https://rustsec.org`).
