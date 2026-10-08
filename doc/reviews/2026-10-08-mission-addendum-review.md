# Mission Addendum A v1.0 — comparison and disposition

**Date:** October 8, 2026. **Result:** adopted by the owner as the [Mission Charter v1.0](../Rikuri_Pi_Mission_Charter_v1.0.md), with corrections; its new promises are folded into the PRD and Architecture. This is a design review, not runtime evidence.

**Source:** "Rikuri Pi — Mission & Foundational Philosophy Addendum, Proposed Addendum A, Version 1.0, October 7, 2026", pasted by the owner into the conversation on October 8, 2026. The owner's accompanying framing, paraphrased: Microsoft, with NVIDIA's help, has a hybrid intelligence implementation on Windows, and Apple is probably working on something like it on its own hardware; Rikuri should free people from being trapped by any one OS or ecosystem and expand a person's horizon and capability, rather than be a generic AI orchestration layer for an OS company. That vendor characterization is the owner's, not a fact this review checked. No file was supplied, so the draft is not in the repository. The charter preserves its text apart from renumbered headings (A.1–A.12 became §1–§12), the two additions in "What was new" (§1 "Why this matters now", §4.5) and the corrections below; this list cannot be checked against a repository copy and its completeness is Not yet proven.

**Compared against:** [PRD v1.2](../Rikuri_Pi_PRD_v1.2.md) §1–5, §9, §22, §27–28; [Architecture v1.7](../Rikuri_Pi_Architecture_Spec_v1.7.md) §10, §13, §26, §27, §49; [ADR 0000](../Rikuri_Pi_ADR_0000_Foundation0_Scope_Reconciliation.md) including the September 28 baseline freeze; the [live Stage Plan v2.1](../Rikuri_Pi_Foundation0_Rebaselined_Stage_Plan_v2.1.md); the [Master Build Sequence](../Rikuri_Pi_Master_Build_Sequence_v1.1.md); [Collaborative Intelligence v0.1](../Rikuri_Pi_Collaborative_Intelligence_Architecture_v0.1.md); and the [Writer IDE Spec](../Rikuri_Pi_Writer_IDE_Spec_v0.1.md).

## What already agrees

Most of the draft restates promises the baseline already makes. Those promises remain controlling; the charter does not duplicate them as requirements.

| Draft | Existing coverage |
| --- | --- |
| A.1 mission, amplifying the human–AI partnership | PRD §1 "Ultimate Personal Intelligence goal", nearly the same wording |
| A.4.1 model independence | PRD P1, P2, P14; §2 "One Personal Intelligence, many interchangeable AI brains"; STUDIO-009 |
| A.4.2 OS independence, Linux first | PRD P13 (not favoring a model, OS, cloud or app ecosystem); §4.1 goals; MACH-004/005; Architecture §26.1–26.2; Windows parity at product Stage 9 |
| A.4.3 no permanently superior hardware vendor | PRD P14; Architecture §49 (post-Foundation provider candidates) forbids CUDA/MLX/ROCm/Vulkan assumptions in kernel contracts; Stage 4 hardware fit |
| A.4.4 APIs over screen automation | PRD P6, P30; Stage 5 host/plugin adapters |
| A.5 lifecycle kernel; information is data, not truth | PRD GOV-010; Knowledge Context Compiler; instruction-authority boundary |
| A.7 fiction versus established fact | STUDIO-005 |
| A.8 authorship, provenance, export | STUDIO-001/002/009, WIDE-006, P9, P16 |
| A.9 "no silent loss, no stale-tail, no false green" | Five working laws; Transactional State Evolution; T04.13 stale-tail handoff test |
| A.10 not tokens or agent counts | PRD §4.2 non-goal on agent count; §22 Collaborative uplift and Discovery usefulness rows |

## What was new, and where it went

| New in the draft | Disposition |
| --- | --- |
| An explicit one-sentence mission and the A.12 decision test | Charter §1 and §12; PRD §1 Mission block; MISSION-001 records the test's answer in the parking lot's "Leaving the lot" procedure, first at the review held with S05.10 |
| More automation is not more capability; independence is a goal (A.2.7, A.8, A.10.5) | PRD P32, MISSION-002, §22 rows. Reconciled with the north-star "the user should not need to understand Bash": the infrastructure disappears from view while the person's own reasoning and craft grow |
| Four platform claim states (A.4.2) | PRD P34, MISSION-003, Architecture §26.5. Gate evidence keeps Pass / Fail / Not yet proven |
| ARM64 Linux and Windows targets | MISSION-005 declares them as targets without a stage; ARM64 CI is in the parking lot |
| NPUs, prefill/decode disaggregation, cross-device execution (A.4.3) | Architecture §26.5 principle; features in the parking lot (product Stage 4 or later) |
| Named creative applications (A.4.4) | Parking lot (product Stage 5 or 8) |
| Adaptive collaboration modes (A.6) and perseverance support (A.3.5) | MISSION-006: Stage 2 keeps what it already plans: creative-mode advice and teaching (WIDE-006), text contextual questions (WIDE-007) and requested reflection (STUDIO-004); a general mode selector is in the parking lot; learned adaptation Stage 10 |
| New success measures (A.10) | Three §22 rows: capability expansion, independent understanding, creative realization |
| The owner's OS-vendor framing (not in the draft's text) | Charter §1 "Why this matters now" and §4.5; PRD P33, MISSION-004; Architecture §26.5 vendor AI layers, including the rules that Local Only work uses a vendor runtime only inside a network-Denied envelope per execution, and that vendor memory or personalization counts as provider retention (SEC-028) |

## Corrections made on adoption

| Draft location | Problem | Correction in the charter or baseline |
| --- | --- | --- |
| Status "governing document … referenced by the PRD" and "Addendum A" | A second authority above the PRD conflicts with AGENTS.md ("does not create a competing roadmap") and the September 28 baseline freeze | Renamed Mission Charter; role stated as rationale and decision test; PRD and Architecture govern; adopted through an ADR 0000 dated clarification |
| A.11 "Required from the beginning: … platform adapters" | Reads as pulling Windows/ARM64 implementations into Foundation-0 | "Platform adapter boundaries"; added that "from the beginning" means contracts, and that Foundation-0 keeps its six stages and Ubuntu reference |
| A.3.3 "adapt … to the user's demonstrated experience"; A.3.5 "Recognize … stalled projects"; A.6 intervention proportionate to "demonstrated competence" | Learned inference about the user conflicts with "instrument early; infer late" unless placed at Stage 10 | A.3.3 adapts to objectives and the help asked for; A.3.5 "Help the user recognize", with a timing note; A.6 drops "demonstrated competence" from the proportionality list, with a timing note; MISSION-006 |
| A.7 "Evolution for existing solutions; revolution where genuine differentiation or discovery matters" | Reuses PRD P31's slogan ("revolution for core strength") with a different meaning and scope | Reworded for the user's own work, with a note that P31 covers how Rikuri is engineered |
| A.8 "Protect … through AISecurity" | Under Option C, Rikuri owns permissions and AISecurity enforces them | "Through Rikuri's permissions and AISecurity enforcement"; export bullet adds that a user can leave Rikuri itself |
| A.9 "Knowledge-as-a-Service" | Not a repository term | "Knowledge Service" |
| A.10 measures about the person | Could be read as permitting hidden profiling | Added: such measures use evidence the user opted into |
| A.12 decision principle | Could be read as overriding gates or security | Added: the test informs priority; it does not override a security boundary, a gate or the user's decision |
| Trailing "How this fits the existing documentation" section | Drafting commentary, not charter content. It contained web-citation residue (a favicon image link, "Linux Foundation +1", `&#x20;` entities) and an unchecked claim about MCP and A2A governance | Removed. The fit is recorded here and in the ADR. The MCP/A2A claim was not verified in this review and is not carried forward |
| Closing note "OS-agnostic does not mean … identical functionality … on day one" | Valuable, but sat outside the addendum body | Moved into charter §4.2 |

## Evidence limits

No code, adapter, platform probe or vendor runtime was exercised. Vendor AI layers are described generically; no specific product's local-only or privacy behavior is claimed. All MISSION rows and Test Strategy §20 cases are Not yet proven. Foundation-0 stages, gates, S/T/E IDs and dates are unchanged. [Build Record](../../docs/build-records/2026-10-08-mission-charter.md).

## Verifier round 1 — October 8

The independent verifier returned FAIL with six Important findings; all mechanical checks passed. Each was corrected before close-out:

| Finding | Correction |
| --- | --- |
| MISSION-006 placed modes in Stage 2 "through purpose presets (§9.22)", which are post-Foundation recipe configurations (CognitiveRecipe, Stage 10), not Stage 2 | Stage 2 keeps only what it plans (WIDE-006 creative-mode advice and teaching, WIDE-007 text questions, STUDIO-004 requested reflection); a general mode selector went to the parking lot. Round 2 caught that the first fix also cited voice Ask and Dictate, which WIDE-007 stages at Stage 8; corrected |
| MISSION-003 applied the four states to all existing claims, collided with S03.11 envelope assurance levels and SEC-024, and could be read as adding Foundation-0 Home or health scope | Limited to new or revised availability claims; assurance levels and gate evidence keep their vocabularies and are reported alongside; runtime reports from product Stage 2, with no S05.02/S05.08 scope |
| The vendor-layer Local Only rule read as a one-time admission, weaker than §27.2/§27.3, and vendor retention was not an admission criterion | Per-execution network-Denied envelope, requalified on runtime or OS change; OS services with their own network are cloud paths; vendor retention falls under SEC-028; acting assistants are opaque native workers that fail closed (SEC-021); adverse rows added |
| MISSION-001 was not wired into the parking-lot procedure, and "Foundation-0 is unchanged" overstated | "Leaving the lot" records the decision-test answer; the ADR names it a process step outside S05.10's acceptance; the README says stages, gates, IDs and dates are unchanged |
| MISSION-005's continuation case had no owning stage | Stage 7 second-device fixtures; Windows pairs Stage 9; ARM64 when sequenced |
| MISSION-002 could let Stage 2 infer the user's capability | Before Stage 10, the capability side shows only lessons the user saved and outcomes they declared; inferred figures are an adverse case |

Nits addressed: the PRD table of contents gained its missing §28 entry with §29 (its placement of §27–29 before Appendix B predates this change and is left alone); the charter's §11 now cites §26.2 for OS-specific and §49 for vendor-specific assumptions, with §26.5 adding processor architecture; the creative-realization measure is opt-in, research-track and creator-declared; MISSION-006's adverse cases name learned behavior and exclude deterministic status signals; the charter's §3.6 and §5 carry timing notes; the vendor characterization above is attributed to the owner; "ARM64 on Linux and Windows"; a Foundation-0 CI job would go through a dated ADR; the decision question is quoted in MISSION-001. Left as written: the charter's "Governing decision principle" heading and "shall" wording are the owner's, offset by its role statement and §12's final sentence.
