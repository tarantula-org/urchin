# QALv1: Quick and Light

---

## 1. THE EFFICIENCY MANDATE

The objective is technical velocity through the reduction of complexity. Code volume is inversely proportional to system stability.

### 1.1 Speculative Features

**Prohibited.**

* **Immediate Requirement:** Code is implemented primarily for the immediate requirement. Abstractions built solely for "hypothetical future use cases" are classified as technical debt.
* **Concrete Implementation:** Introducing an abstraction layer (interface, trait, abstract class) requires documenting at least two concrete, immediate use cases. One theoretical use case is insufficient.

### 1.2 The Dependency Budget

**Default: Reject.**

* **Native Priority:** The Standard Library is the primary toolset.
* **Justification:** External dependencies are liabilities (security, size, compilation time). A dependency is only permitted if implementing the functionality natively requires >10x the effort of integration.

### 1.3 The Generalist Toolset

**Converge, don't diverge.**

* **Redundancy Check:** Do not introduce a new tool if an existing one within the stack can achieve the outcome.
* **Cognitive Compression:** A smaller vocabulary of libraries reduces context switching. Mastery of a few versatile tools is superior to superficial usage of many niche ones.

---

## 2. ARCHITECTURAL RIGOR

Architecture exists to enforce boundaries and control state, not to demonstrate cleverness.

### 2.1 Decoupling

* **Single Responsibility:** A component addresses a single functional requirement. Logic describing "X and Y" implies a structural defect.
* **Topology:** Prefer flat, wide structures over deep, nested ones. A module should be comprehensible within a single screen of code. Inheritance depth > 2 is a failure of composition.

### 2.2 Data Flow

* **Unidirectional:** State flows down; events flow up.
* **Explicit State:** "Magic" (implicit side effects, hidden mutations, macro-heavy logic) is banned. Control flow must be traceable by static analysis.

---

## 3. DEVELOPMENT PROTOCOL

Velocity is maintained via rigorous automation. The machine is the gatekeeper.

### 3.1 Atomicity

* **Commit Scope:** One logical change per commit.
* **Review Threshold:** Diff size is capped. Any change that cannot be understood in a single, focused review session (~30 minutes) must be decomposed.

### 3.2 The Automaton

* **Binary Pass/Fail:** Linting, formatting, and tests are non-negotiable gates. Logic is not reviewed until syntax is perfected by the CI system.
* **Main Branch Sanctity:** `main` is deployable at all times. A broken build is a P0 emergency.

### 3.3 Functional Precedence

**Make it work, then make it right, then make it fast.**

* **Non-Destructive Progress:** A functional implementation that passes tests is the priority. Perfectionism that delays merging is a failure.
* **Regression Zero:** "Working" implies zero regressions. New features must not break existing contracts. Refactoring happens on green builds only.

---

## 4. SOURCE HYGIENE

Source code is written for maintenance, not for the compiler.

### 4.1 Cognitive Load

* **Cleverness is a Defect:** Readability is measured by the speed at which a new team member can understand the logic. "Boring" code is correct code.
* **Self-Documentation:** Variable names describe *content*. Function names describe *action*. Comments explain *intent* (Why), not implementation (How).

### 4.2 Formatting

* **Zero Debate:** The `.editorconfig` / formatter configuration is law.
* **Dead Code:** Commented-out blocks and unused imports are noise. Delete them immediately. The git history is the archive.

---

## 5. PERFORMANCE & SAFETY

Performance is not an afterthought; it is a constraint.

### 5.1 Critical Path

* **Targeted Optimization:** Do not optimize initialization. Optimize the loop.
* **The Hot Path:** Code executed per-frame or per-request must be allocation-free where possible. Complex logic is pushed to the cold path.

### 5.2 Failure State

* **Fail Fast:** Systems in invalid states must halt immediately.
* **Error Visibility:** Use `Result`/`Option`-type patterns. Global error states are prohibited. Exceptions are reserved for unrecoverable, programmatic errors only.

---

## 6. COMMIT STANDARD

Commit history is a database. It must be queryable via semantic prefixes.

* `feat`: Adds capability.
* `fix`: Restores intended behavior.
* `cut`: **Mandatory for deprecations.** The removal of any public API must be prefixed with `cut:` and include a migration note in the commit body.
* `docs`: Documentation-only changes.
* `refactor`: Structural change, zero behavioral change.
* `perf`: Measurable resource improvement.

---

## 7. GOVERNANCE

To prevent dogma, these rules allow for evolution under specific conditions.

* **Amendment Process:** Changes to this Constitution require a 2/3 majority vote of the maintainers.
* **Waiver Process:** A deviation requires a documented RFC approved by two senior maintainers, citing the specific constraint.

---

## 8. THE GOLDEN RULE

**The spirit of this document governs over its letter.** When in doubt, choose the option that best serves long-term system clarity and maintenance burden.

---

**QALv1.** *Released into the Public Domain.*