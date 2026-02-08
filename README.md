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

## <img src="https://cdn.simpleicons.org/blueprint/d63031" width="24" style="vertical-align: bottom;" /> Operation & Workflow

Urchin enforces **Two-Person Integrity (TPI)**. No single staff member can unilaterally ban or kick a user. The workflow requires two distinct identities: a **Requester** and an **Approver**.

### 1. The Request (Requester)
A staff member initiates a governance action using Slash Commands.

* **Ban:** `/ban [user] [reason]`
* **Kick:** `/kick [user] [reason]`

> **Result:** Urchin does *not* execute the action immediately. Instead, it generates a **Governance Proposal Embed** in the channel, detailing the target and the reason.

### 2. The Consensus (Approver)
A *different* staff member must review the proposal.

* **Action:** Click the **[Confirm]** button on the embed.
* **Constraint:** The **Requester cannot be the Approver**. If the requester tries to click the button, the system will reject the action with a `Self-approval not allowed` error.

### 3. Execution & Audit
Once consensus is reached (2/2 signatures), Urchin immediately:
1.  **Executes** the ban or kick on the platform.
2.  **Logs** the action in the platform's Audit Log with a signed reason:
    `"Spamming | Req: Staff_A | App: Staff_B"`
3.  **Clean Up:** Removes the proposal from the active state to prevent double-jeopardy.

---

## <img src="https://cdn.simpleicons.org/github/333333" width="24" style="vertical-align: bottom;" /> Configuration

Urchin is stateless and configured via environment variables.

| Variable | Description |
| :--- | :--- |
| `DISCORD_TOKEN` | Your Bot Token from the Developer Portal. |
| `DISCORD_GUILD_ID` | The Server ID where commands are registered. |
| `DISCORD_STAFF_ROLE_ID` | The specific Role ID allowed to use commands. |
| `RUST_LOG` | Logging level (default: `info`). |

```bash
# Example .env
DISCORD_TOKEN=OTk5...
DISCORD_GUILD_ID=1234567890
DISCORD_STAFF_ROLE_ID=9876543210
```