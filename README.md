<div align="center">

  <img src="logo.svg" alt="URCHIN" width="200" />

  <br />

  <h1> URCHIN <br>
  <br />

  <img src="https://img.shields.io/badge/License-AGPL_v3-343a40?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/badge/Core-Rust-b7410e?style=for-the-badge&logo=rust&logoColor=white" alt="Language" />
  <img src="https://img.shields.io/badge/Adapter-Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Discord" />
  <img src="https://img.shields.io/badge/Adapter-Telegram-26A5E4?style=for-the-badge&logo=telegram&logoColor=white" alt="Telegram" />

</div>

<br />

## <img src="https://cdn.simpleicons.org/blueprint/d63031" width="24" style="vertical-align: bottom;" /> Overview

**URCHIN** is a platform-agnostic consensus engine enforcing **Two-Person Integrity (TPI)**.

It decouples governance decisions from execution. Operating as a centralized async kernel, Urchin ingests requests from disparate front-ends (Discord, Telegram) and holds them in a suspended state. Actions execute only after ratification by a second, distinct authorized identity.

## <img src="https://cdn.simpleicons.org/polywork/e67e22" width="24" style="vertical-align: bottom;" /> Architecture

The system follows a "Spoke-and-Hub" topology, isolating core consensus logic from volatile transport layers.

| Module | Stack | Responsibility |
| :--- | :--- | :--- |
| **Nucleus** | <img src="https://img.shields.io/badge/Tokio-18181b?style=flat&logo=rust&logoColor=white" /> | **State Machine.** Handles TPI validation, identity resolution, and atomic audit logging. |
| **Spike: Discord** | <img src="https://img.shields.io/badge/Serenity-5865F2?style=flat&logo=discord&logoColor=white" /> | **Transport.** Manages Gateway intents, interactions, and strict role hierarchy checks. |
| **Spike: Telegram** | <img src="https://img.shields.io/badge/Teloxide-26A5E4?style=flat&logo=telegram&logoColor=white" /> | **Transport.** Functional dispatch pipeline for command parsing and chat verification. |
| **Shell** | <img src="https://img.shields.io/badge/Sled-b7410e?style=flat&logo=sqlite&logoColor=white" /> | **Persistence.** Embedded, lock-free structured storage for state recovery. |

## <img src="https://cdn.simpleicons.org/github/333333" width="24" style="vertical-align: bottom;" /> Distribution

Urchin is **Open Source (AGPLv3)**. Network-accessible modifications must be public.

| Availability | Description |
| :--- | :--- |
| **Self-Hosted** | <img src="https://img.shields.io/badge/Repo-GitHub-18181b?style=flat&logo=github&logoColor=white" /> Full source access. Manual compilation and maintenance required. |
| **Managed** | <img src="https://img.shields.io/badge/Service-Tarantula_Tech-e67e22?style=flat" /> SLA-backed deployment, zero-config consensus, and priority support. |

[![Deploy on Tarantula](https://img.shields.io/badge/DEPLOY-MANAGED_INSTANCE-18181b?style=for-the-badge&logo=rust&logoColor=white&labelColor=e67e22)](https://tarantula.tech/urchin)