<div align="center">

  <img src="urchin_logo.svg" alt="URCHIN" width="450" />

  <br />

  **Universal Runtime for Consensus & Hybrid Integration Networks**

  <br />
  <br />

  <img src="https://img.shields.io/badge/License-AGPL_v3-white?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/badge/Language-Rust-b7410e?style=for-the-badge&logo=rust&logoColor=white" alt="Language" />
  <img src="https://img.shields.io/badge/Crate-Tokio_Async-333333?style=for-the-badge&logo=rust&logoColor=white" alt="Async" />
  <img src="https://img.shields.io/badge/Status-Pre--Alpha-d63031?style=for-the-badge" alt="Status" />

</div>

<br />

## <img src="https://cdn.simpleicons.org/blueprint/d63031" width="24" style="vertical-align: bottom;" /> Overview

**URCHIN** is a platform-agnostic consensus engine designed for high-stakes community governance.

Unlike monolithic bots bound to a single API, Urchin operates as a decentralized kernel with radial connectivity. It decouples **governance logic** (bans, approvals, consensus) from **transport layers** (Discord, Telegram, Matrix). This architecture ensures that security policies remain immutable and unified, regardless of the ingestion surface. It is built to enforce "Two-Person Integrity" (TPI) rules across distributed networks.

## <img src="https://cdn.simpleicons.org/polywork/e67e22" width="24" style="vertical-align: bottom;" /> Architecture

The system mimics an echinoderm's morphology: a central hard-shell nucleus handling state, with modular "spikes" interfacing with external platforms.

| Module | Stack | Responsibility |
| :--- | :--- | :--- |
| **Nucleus** | <img src="https://img.shields.io/badge/Rust-Core-b7410e?style=flat&logo=rust&logoColor=white" height="20" /> | The central state machine. Handles consensus logic, TPI enforcement, and audit logging. |
| **Spikes** | <img src="https://img.shields.io/badge/Tokio-Transport-333333?style=flat&logo=rust&logoColor=white" height="20" /> | Pluggable, async adapters for external APIs (Discord Gateway, Telegram Bot API). Stateless and hot-swappable. |
| **Shell** | <img src="https://img.shields.io/badge/Sled-Persistence-5c5c5c?style=flat&logo=sqlite&logoColor=white" height="20" /> | Embedded, high-performance structured storage for persistent state and audit trails. |

## <img src="https://cdn.simpleicons.org/github/ffffff" width="24" style="vertical-align: bottom;" /> Deployment & Licensing

Urchin is **Open Source** software. The core runtime is free to inspect, audit, and self-host under the **AGPLv3** license.

### Managed Infrastructure (Tarantula Tech)
For production environments requiring guaranteed uptime, zero-config deployment, and cross-platform synchronization, we offer a managed solution.

| Tier | Availability | Features |
| :--- | :--- | :--- |
| **Self-Hosted** | <img src="https://img.shields.io/badge/Source-GitHub-ffffff?style=flat&logo=github&logoColor=black" /> | Full source access. Manual compilation and maintenance required. |
| **Managed** | <img src="https://img.shields.io/badge/Service-Tarantula-e67e22?style=flat" /> | Instant deployment, SLA uptime, and priority consensus handling. |

[![Deploy on Tarantula](https://img.shields.io/badge/DEPLOY-MANAGED_INSTANCE-ffffff?style=for-the-badge&logo=rust&logoColor=black&labelColor=e67e22)](https://tarantula.tech/urchin)