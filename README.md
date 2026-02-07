<div align="center">

  <img src="urchin_logo.svg" alt="URCHIN" width="450" />

  <br />

  **Universal Runtime for Consensus & Hybrid Integration Networks**

  <br />
  <br />

  <img src="https://img.shields.io/badge/License-AGPL_v3-white?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/badge/Language-Rust-b7410e?style=for-the-badge&logo=rust&logoColor=white" alt="Language" />
  <img src="https://img.shields.io/badge/Discord-Serenity-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Discord" />
  <img src="https://img.shields.io/badge/Telegram-Teloxide-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white" alt="Telegram" />

</div>

<br />

## <img src="https://cdn.simpleicons.org/blueprint/d63031" width="24" style="vertical-align: bottom;" /> Overview

**URCHIN** is a platform-agnostic consensus engine designed for high-stakes community governance.

Engineered to enforce **"Two-Person Integrity" (TPI)** protocols, Urchin decouples the *decision* to ban from the *execution* of the ban. It operates as a centralized async kernel that ingests moderation requests from multiple front-ends (Discord, Telegram) and holds them in a suspended state until a second, distinct authorized user ratifies the action.

## <img src="https://cdn.simpleicons.org/polywork/e67e22" width="24" style="vertical-align: bottom;" /> Architecture

The system functions as a concurrent "Spoke-and-Hub" daemon. The core logic remains isolated from the transport layers, allowing for unified security policies across disparate chat protocols.

| Module | Stack | Responsibility |
| :--- | :--- | :--- |
| **Nucleus** | <img src="https://img.shields.io/badge/Tokio-Async_Runtime-333333?style=flat&logo=rust&logoColor=white" height="20" /> | The central state machine. Handles TPI validation, identity linking, and audit logging. |
| **Spike: Discord** | <img src="https://img.shields.io/badge/Serenity-Library-5865F2?style=flat&logo=discord&logoColor=white" height="20" /> | A strict implementation of `serenity-rs`. Handles Gateway intents, button interactions, and role hierarchy checks. |
| **Spike: Telegram** | <img src="https://img.shields.io/badge/Teloxide-Library-2CA5E0?style=flat&logo=telegram&logoColor=white" height="20" /> | A functional dispatch pipeline using `teloxide`. manages command parsing and chat-id verification. |
| **Shell** | <img src="https://img.shields.io/badge/Sled-Persistence-5c5c5c?style=flat&logo=sqlite&logoColor=white" height="20" /> | Embedded, high-performance structured storage for persistent state and recovery. |

## <img src="https://cdn.simpleicons.org/github/ffffff" width="24" style="vertical-align: bottom;" /> Distribution & Licensing

Urchin is **Open Source** software. The core runtime is free to inspect, audit, and self-host under the **AGPLv3** license. This license requires any network-accessible modifications to be made public.

### Managed Infrastructure (Tarantula Tech)
For production environments requiring guaranteed uptime, zero-config deployment, and cross-platform synchronization, we offer a managed solution.

| Tier | Availability | Features |
| :--- | :--- | :--- |
| **Self-Hosted** | <img src="https://img.shields.io/badge/Source-GitHub-ffffff?style=flat&logo=github&logoColor=black" /> | Full source access. Requires manual compilation (`cargo build --release`) and maintenance. |
| **Managed** | <img src="https://img.shields.io/badge/Service-Tarantula-e67e22?style=flat" /> | Instant deployment, SLA uptime, and priority consensus handling. |

[![Deploy on Tarantula](https://img.shields.io/badge/DEPLOY-MANAGED_INSTANCE-ffffff?style=for-the-badge&logo=rust&logoColor=black&labelColor=e67e22)](https://tarantula.tech/urchin)