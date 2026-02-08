# Security Policy

## Reporting a Vulnerability

We take the security of the Urchin consensus engine seriously. If you discover a vulnerability, please report it directly to our security team.

**DO NOT** open public issues for sensitive security exploits.

* **Email:** `urchin.tarantula.tech@atomicmail.io`
* **Response Time:** We aim to acknowledge receipt within 48 hours.

---

## Release Lifecycle & Readiness

Urchin uses rigorous semantic tagging to indicate the production readiness of each release. Operators should strictly adhere to these definitions when deploying in high-stakes environments.

| Tag | Status | Definition | Recommended Use |
| :--- | :--- | :--- | :--- |
| **`stable`** | <img src="https://img.shields.io/badge/-PRODUCTION-success?style=flat-square" /> | **Field Proven.** Full feature set, documentation complete, and tested under load. | **Production.** Safe for high-value communities. |
| **`rc`** | <img src="https://img.shields.io/badge/-FROZEN-blueviolet?style=flat-square" /> | **Release Candidate.** Code is frozen. No new features, only critical bug fixes. | **Staging.** Final validation before rollout. |
| **`beta`** | <img src="https://img.shields.io/badge/-FIELD_TEST-orange?style=flat-square" /> | **Feature Complete.** Core logic is built but requires broader field testing to verify edge cases. | **Early Adopters.** Use with active monitoring. |
| **`alpha`** | <img src="https://img.shields.io/badge/-VOLATILE-red?style=flat-square" /> | **Internal Mechanics.** Experimental features and potential breaking API changes. | **Development Only.** Do not use in production. |
