# Rikuri Pi — Writer's IDE Specification v0.1

October 1, 2026 · Owner-directed architecture/specification · **Unimplemented; runtime acceptance Not yet proven**

This specification refines the [Writer architecture v1.4](Rikuri_Pi_Writer_Creative_Authoring_Architecture_v1.4.md), [PRD STUDIO-001–010](Rikuri_Pi_PRD_v1.2.md#27-september-30-creative-studio-requirements), and shared Versioned Work service. The [Master Build Sequence](Rikuri_Pi_Master_Build_Sequence_v1.1.md) remains the sole delivery roadmap: essentials at product Stage 2 S2.C1–C4 after trusted core/Stage 1; multi-human/agent collaboration after Stage 6 governance; advanced domain/publishing tools at Stage 8. Foundation-0 00–05 is unchanged. Requirements below have positive/negative cases in §10; they are not passed tests.

## 1. Purpose and experience

A writer's IDE joins creation, organization, revision, collaboration and learning around the author's durable work. Manuscript first; reveal tools progressively. Support fiction, nonfiction, research, reports, scripts, poetry and fragments without mandatory classification or a fixed workflow. A project may begin with one unstructured thought.

Creative promise: **You write the work. Rikuri helps you develop it and your craft.** Business-production promise: **Rikuri helps your team produce material you review and approve.** These are purpose/permission distinctions, not a judgment that business lacks creativity or that AI output automatically carries exclusive rights.

Optional craft reminders:

- What am I trying to express?
- What does the reader need to notice, feel, or wonder?
- Which words, actions, details and rhythms make that possible?
- What do I want to linger after this passage?

Agents compare author intent with plausible reader experience, state uncertainty, respect deliberate ambiguity and nonstandard voice, and avoid mandatory scores or mechanical rules. “Show, don't tell” is contextual: summary and telling can serve pace, voice and emphasis. Economic prose chooses useful detail; it does not always minimize word count.

## 2. Requirements and ownership

All modules here are planned contracts, not existing crates. Canonical commands use the governed Knowledge/Versioned Work transaction path; modules cannot create competing storage authority.

| ID | Required behavior | Owning module / delivery |
|---|---|---|
| WIDE-001 | Stable identities, revisioned text, containment, lineage and portable representation independent of editor/model | Creative Model / Stage 2 S2.C1 |
| WIDE-002 | Resolve durable anchors to current or historical structure and edition locations; expose deleted, ambiguous and unavailable targets | Anchor Resolver + Layout Adapter / Stage 2 basic; Stage 8 edition fidelity |
| WIDE-003 | Checkpoints, alternatives, workspace isolation, structural/text diffs, selective proposals, conflict review and atomic promotion | Versioned Work / Stage 2 S2.C3 |
| WIDE-004 | Reversible scoped deletion, whole-subtree restoration, retention/purge disclosures and current-revocation-aware recovery | Versioned Work + lifecycle service / Stage 2 |
| WIDE-005 | Manuscript, outline, scene board and basic chronology share identity; reading order differs from event time; graph is optional | Authoring Views / Stage 2 S2.C2; richer graph Stage 8 |
| WIDE-006 | Creative agents advise/teach rather than compose manuscript; business drafting is separately explicit; enforce tool boundaries | Craft Agent + policy service / Stage 2 S2.C2–C3; business pack Stage 8 |
| WIDE-007 | Contextual prompt questions; separate ask/dictate modes; voice is opt-in and cannot silently upload or insert speech | Question Surface / Stage 2 text; voice after admitted media adapter at Stage 8 |
| WIDE-008 | Optional lessons and personal flashcards from chosen exchanges; editable source links, explicit review settings and export | Learning Notebook / Stage 2 basic within S2.C2/C4; richer review Stage 8 |
| WIDE-009 | Versioned collaboration agreement separates creative role, action authority, rights and AI processing consent | Collaboration service / Stage 2 solo policy; multi-human Stage 6 S6.C1–C4 |
| WIDE-010 | Structured sources/citations, notes, labels, contents and index entries with regeneration and unresolved-reference reporting | Reference service + publishing adapters / Stage 2 preservation/links; full tools Stage 8 |
| WIDE-011 | Open local project export/restore, provenance and protected source permissions survive component replacement | Portability service / Stage 2 S2.C4; scholarly/media adapters Stage 8 |
| WIDE-012 | Candidate engine must pass editor-independent identity/history/anchor conformance before adoption | Editor Adapter / Stage 2 S2.C1–C4 selection spike |

## 3. Creative object and reference model

Minimum objects: Work, StructuralUnit (chapter/episode/act/scene/fragment), Paragraph, Sentence, Event, Annotation, Source, Citation, Label, IndexEntry, Snapshot, Branch, Workspace, ChangeProposal, Lesson, Flashcard, AgreementRevision and AgentRole. Typed object IDs are opaque, collision-checked and never reused; human-readable numbers are projections. Revisions are immutable. Objects include schema version, provenance and applicable access/rights metadata. A branch snapshot records revisions, containment and ordering, so alternate arrangements do not mutate each other.

Containment is validated and acyclic; cross-links may form a graph. Sentence is the smallest independently identified prose unit, but annotations/edits can target sub-sentence ranges. Preserve fragments, poetry and dialogue as valid text; no requirement to force every passage into a grammatical sentence. Segmentation is language-aware, versioned and correctable. Typing a period must not silently rebuild identities. Undo, reload and segmentation correction preserve identity/lineage.

| Operation | Identity contract |
|---|---|
| Move / reorder | Preserve object and child IDs; change branch-local placement |
| Revise | Preserve identity where it is a revision of the same object; append content revision |
| Split | New resulting IDs with explicit source-revision/range lineage; original remains historical |
| Merge | New result ID with lineage to all inputs; retain input history |
| Copy | New IDs for copy and descendants, with origin links |
| Delete / restore | Tombstone scoped membership/content; restore original IDs when admitted |

Imported IDs are validated/remapped within a declared import namespace, never trusted as authority. Ambiguous semantic identity changes require visible resolution; an AI cannot invent ownership or silently assign lineage.

Example: permanent `sentence-a82f` at revision 7 displays C10P4S2. After moving its paragraph it displays C12P2S1. Another sentence can occupy C10P4S2; permanent IDs distinguish them. References specify work/branch/revision or explicitly track the current admitted branch. Historical references never silently become current references.

Anchor shape: object ID, revision, range with declared offset encoding, boundary affinity, and optional quote/context for validation. Resolver returns `resolved`, `historical`, `deleted`, `ambiguous`, `denied` or `unavailable`. Denied results must not disclose inaccessible content/metadata. Split/merge mapping uses lineage and edit maps; quote matching alone cannot silently choose a duplicate. Orphaned remarks remain visible to authorized readers.

Page/line locations are derived for a named edition: work revision, layout profile, fonts/resources and renderer version. Sentences may span pages/lines. Window wrapping is not stable numbering. Offer optional paragraph numbering and fixed-layout/deterministic review lines; labels such as C10P4S2 never enter canonical prose. Pagination changes invalidate old layout maps, not content IDs.

## 4. Versioned Work and recovery

Checkpoint = consistent snapshot; branch = alternative ancestry; workspace = isolated editable view of a branch, not a compulsory Git checkout. Multiple windows of one workspace share that workspace rather than accidentally fork. Human-friendly controls: Checkpoint, Explore alternative, Compare, Propose changes, Accept selected changes, Restore. Git/GitHub are optional adapters, not a canonical schema or required account.

A proposal records author/agent provenance, base snapshot, target branch, affected objects/ranges, edits/moves, rationale, rights/policy versions and unresolved conflicts. Three-way comparison recognizes moves rather than delete/recreate. Stale overlapping edits, delete/edit, incompatible parents/order and contradictory semantic changes are surfaced. Disjoint valid changes may be rebased with evidence; approval binds the exact resulting candidate. Partial acceptance creates a validated new proposal/result and retains rejected alternatives. Model continuity advice is advisory; author may retain an intentional contradiction, with the decision recorded.

Commands carry operation ID, expected revisions, scoped actor and policy context. Retries are idempotent. Validate permissions, references, containment, conflict resolution and approval; durably write candidate, read back, then atomically promote. Acknowledgement distinguishes pending, saved and verified. Revert makes a new change preserving ancestry. No editor/model writes canonical storage directly.

Delete distinguishes subtree deletion, container removal with child relocation, and movement. Subtree deletion captures exact snapshot/membership plus affected annotations/links; one operation is reversible. Restore validates current parents, conflicts and permissions; missing parent requires destination choice. Shared branch references prevent ordinary deletion from purging still-referenced content. Explicit privacy erasure follows platform forget/revocation semantics across retained versions and derivatives; history is not an exemption.

Retention policy is visible before use; exact default/duration and backup purge behavior must be chosen before shipping, with no indefinite recovery promise. Purge previews affected branches/notes/lessons and recovery limits. Managed restore consults current revocation and cannot resurrect revoked content/access. External exports remain outside the forget guarantee.

## 5. Structure and reader experience

Manuscript, outline, storyboard and timeline are views of shared objects. Dragging a card invokes a governed reorder command, offers undo and updates displayed numbering. Events represent happenings; passages represent tellings. One event may have several accounts. Reading order, event chronology and causal/character-knowledge relationships are separate. Flashback is a presentation relationship, not necessarily a distinct grammatical container type.

Moving a childhood scene later changes reading order without changing its event date. Continuity review may flag information revealed before its introduction; the author decides whether this is deliberate. Rich graph and multi-thread visualization are optional and follow Stage 8 depth. Neither graph completeness nor agreement with an agent is required to write.

## 6. Agent roles and contextual questions

Creative mode permits questions, critique, research, vocabulary options, sentence-structure explanations, short clearly labeled illustrative examples, and non-prose outlines/scenario alternatives. It forbids agent-written finished scenes, chapters or substantial manuscript passages for incorporation. Scene explorer means structural possibilities, not scene ghostwriting. Agent tools may create advice, annotations, lessons or structural proposals within grants; creative agents have no manuscript-composition tool. Mechanical formatting/correction proposals are separate, visibly scoped tools; no disguised substantive rewrite. Human review/promotion remains required under existing policy.

Prevent evasion by repeated sentence-sized requests assembling a scene: evaluate task purpose and cumulative generation, not just a word-count threshold. Allowed illustration stays in the advice surface, with generated provenance; no one-click insertion as author text. A failed or out-of-role response stays an explicit failed candidate, never an accepted manuscript. Imported instructions, agent personas, prompt text and branches cannot switch policy. The application cannot reliably detect externally generated pasted prose; terms and provenance are not a universal AI detector.

Business mode may draft deliverables with declared AI use, review and publication authority. A workspace purpose/policy change is explicit, versioned and visible; affected collaborators must accept revised terms before participating under them. Previous authorship provenance remains. Marketing is a later pack, not permission for Stage 2 creative generation.

Question surface binds a request to visible selection or disclosed current paragraph, branch and revision. Author may adjust scope; context expands only within admitted sources. Preserve cursor/draft. Late response identifies its original revision and cannot overwrite newer work. Reader reactions are hypotheses, not precise predictions or craft verdicts. Quiet by default; no mandatory tutor flow or unsolicited interruption quota.

Voice offers distinct Ask Rikuri and Dictate controls with visible active mode, push-to-talk/cancel, transcript correction, optional spoken replies and accessible text fallback. Ask never inserts into manuscript; dictate produces a reviewable human-speech transcript. Audio storage and provider processing require separate explicit choices, current source/egress admission and budget checks. Local Only refuses cloud speech/model HTTP before construction. Unavailable local speech returns to text visibly, without a cloud fallback. No background recording.

## 7. Learning notebook and flashcards

The author chooses Save lesson / Make flashcard, or explicitly opts into offers to save. Merely asking is not permission for durable learning capture beyond the disclosed conversation retention. Today summary includes only saved lessons, uses the user's timezone and distinguishes generated candidates from saved artifacts. Saved records include chosen explanation, technique/word, context, source passage/discussion revision and provenance, user-edited version and practice prompt. Preview third-party/private content before saving; source permissions propagate.

Lesson example: how action lets a reader infer nervousness; when telling is preferable; original and author revision; practice rewriting a different passage. Word card: meaning in context, usage/register, distinctions (recoil versus flinch), optional pronunciation and author's example. Front prompts recall or application; back explains nuance. Precise means appropriate, not obscure. Technique cards may ask whether dialogue acts between characters or merely briefs the audience.

Review/reminders are separately opt-in; scheduling uses explicit settings and user feedback, not hidden personality/trait inference. Pausing preserves cards; no unattended jobs created by saving one. No automatic project-to-global promotion. Edit/export/delete and current-revocation-aware restoration are required. Cross-episode learned coaching remains Stage 10; saved lessons and deterministic personal review do not imply it exists.

## 8. Collaboration and intellectual-property boundaries

Before a human joins or an agent receives access, show project purpose/AI policy, creative role, allowed actions, rights/credit/reuse terms, merge/publication authority, source/history access and AI destinations. Record authenticated acceptance of exact AgreementRevision. Project membership is not cloud consent; editorial control is not legal ownership. Agent role describes capability/accountability, not a human rights holder.

Support author-led, equal-coauthor, anthology/shared-world and openly licensed project arrangements. These are templates requiring explicit terms, not automatic legal conclusions. Branch ancestry retains applicable source restrictions; permission to experiment, submit, share, incorporate and publish is distinct. A branch/merge/accepted proposal does not transfer copyright. Track contributor evidence and agreed rights; do not infer sentence-by-sentence legal ownership, guarantee copyrightability/exclusivity or resolve disputes by AI vote. Joint work and derivative-work questions require jurisdiction-aware legal review before commercial templates are advertised.

Changed terms need renewed acceptance for subsequent participation; do not retroactively rewrite previous licenses. Withdrawal/revocation effects follow recorded agreements and current access controls; the UI discloses that already permitted/disclosed external copies cannot necessarily be recalled. Stage 6 delivers multi-human/agent enforcement after membership/history/audience/replay prerequisites. Stage 2 solo-policy UX does not claim shared-book collaboration works.

## 9. References, portability and engine selection

Reference service owns structured Source/Citation/Locator objects, Footnote/Endnote content, heading/figure/table labels and IndexEntries linked to anchors. Displayed citation style, numbering, contents, bibliography and final index pages are derived. Reordering updates labels; source identity and cited locator stay unchanged. Bibliography may include explicitly selected uncited sources. Index entries support terms, subentries and see/see-also; publish page locators only from a verified layout map. Missing references remain visible; formatting never verifies a source claim.

Portable package must include schema/version, objects/revisions/lineage, snapshot/branch graph, annotations, reference metadata, chosen lessons/cards and necessary admitted assets with a manifest/integrity checks. Export is scoped and explicit; excluded restricted assets have visible placeholders, not false completeness. Round-trip import must restore supported identity and relationships or report losses. Plain text/Markdown remain readable projections, but cannot alone preserve the whole graph. DOCX/PDF are publishing/interchange outputs with stated fidelity limits.

Tiptap/ProseMirror is a provisional editor candidate, Lexical a comparison candidate; novelWriter/Manuskript are workflow references rather than chosen forks. No engine supplies the complete studio. Before adoption record exact pinned version/license, maintenance, offline behavior, trust boundary, lifecycle/integration cost, accessibility/IME support and failure behavior. Verify paid versus open-source comments, pages, conversion and AI features independently; do not assume a vendor toolkit is fully open source. Scholarly processing should evaluate established citation-style components rather than custom style rules. No component/version or subscription is adopted here.

Editor conformance spike: edit/select/reorder/split/merge/copy with stable IDs; comments survive; reload, undo and export/restore retain relationships; stale proposal refuses; IME and supported-language segmentation preserve text; renderer replacement leaves project readable. Professional pagination, indexes and scholarly round trips require their own Stage 8 receipts.

## 10. Acceptance and delivery dispositions

Deterministic core fixtures need no API credentials. Scripted agent/speech outputs exercise the same policy/transactions as live adapters. Human evaluation assesses teaching usefulness and author control; model agreement is not a deterministic oracle. All cases below are **Not yet proven**.

| Case / requirements | Positive evidence | Negative/failure evidence |
|---|---|---|
| A01 / WIDE-001–002 | Move C10P4S2; original ID resolves at new position and old revision | Reused label points to different ID; duplicate quotes/split ambiguity cannot misattach; denied target leaks no metadata |
| A02 / WIDE-001–003 | Edit/split/merge/copy, restart and undo preserve documented lineage/identity | Segmentation correction/IME cannot lose text or silently regenerate IDs; cyclic parent refused |
| A03 / WIDE-003 | Two workspaces compare moves and edits; selective merge retains ancestry/rejected alternatives | Stale overlapping edit, delete/edit and unapproved candidate refuse; repeated operation ID has one effect |
| A04 / WIDE-004 | Delete and restore paragraph/subtree with original child order and annotations | Missing parent asks destination; crash leaves last good snapshot; purge limits disclosed; revoked content cannot return from backup |
| A05 / WIDE-005 | Storyboard move updates manuscript/outline; event date remains fixed | Layout/order changes cannot silently change chronology; intentional contradiction can be retained by author |
| A06 / WIDE-006 | Explain show/tell using short labeled example; human rewrites; structural scenario allowed | Generate-scene and serial sentence ghostwriting refused; agent cannot insert example/prose or change mode through prompt |
| A07 / WIDE-007 | Selection-bound text question preserves cursor; admitted voice ask/dictate remain distinct | Late response cannot overwrite newer draft; Local Only speech refuses HTTP; cancelled mic stops capture; no silent cloud fallback |
| A08 / WIDE-008 | Explicitly save/edit/export a lesson and word card; chosen review prompt works | Unsaved question creates no lesson; reminders require consent; source revocation prevents stale card exposure; no inferred traits |
| A09 / WIDE-009 | Two humans and agent accept exact terms, submit and approve admitted proposal after Stage 6 gates | Unaccepted amended terms, expired grant, unauthorized publication/cloud share denied; branch cannot enlarge license |
| A10 / WIDE-010 | Reorder headings/figures, regenerate citations/contents and resolve index in named edition | Missing source/label and stale layout remain unresolved; no invented source or promised exact DOCX fidelity |
| A11 / WIDE-011–012 | Export/restore in replacement editor without provider/subscription; manifest agrees | Corrupt/partial/unsupported schema refuses promotion; scoped omissions explicit; forbidden asset cannot leak through package |

First Stage 2 vertical slice: author writes a fragment, asks a craft question, revises it, explicitly saves a lesson/flashcard, checkpoints, explores an alternative arrangement, moves a scene and restores a deletion without losing references/history, then exports/restores. Start text-only and single human; do not require paid cloud or every advanced tool. Report failures, gaps and author-rated usefulness against plain editor/chat.

Dispositions (owner: product owner and owning-stage implementer): Stage 2 defines retention defaults, segmentation languages/rules, illustrative-example boundaries and candidate editor assessment before implementation. Stage 6 adds enforceable multi-human agreements and branch sharing after governance gates. Stage 8 adds admitted voice, richer optional review scheduling, scholarly publication fidelity and graph/domain depth after adapter prerequisites. These refine existing stage scope, not new build ordinals; implementation estimates and any material stage expansion require reconciliation in the Master Build Sequence before starting dependent work. Learned profiling remains Stage 10. Public exposure requires existing release/security gates; this specification closes none.
