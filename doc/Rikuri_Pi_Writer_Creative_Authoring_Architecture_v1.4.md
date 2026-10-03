# Rikuri Pi Writer / Creative Authoring Architecture

> Repository consistency revision: September 25, 2026. Source version retained; see [reconciliation record](reviews/2026-09-25-documentation-consistency.md).

**Human-first authorship, world creation, adaptive storyworlds, and timeless creative artifacts**  
**Architecture & Roadmap Consolidation v1.4 - September 2026**

## Scope and roadmap alignment

This is Rikuri’s lead post-Foundation creative domain architecture. The September 30 direction brings essential Writer foundation/MVP into product Stage 2 S2.C1–C4; deeper world, scientific, multimedia and interactive packs remain product Stage 8 in the Master Build Sequence v1.1. Its “Foundation,” phases 1–6, and MVP describe Writer delivery order, not Foundation-0 commitments or platform stage numbers. Shared services may land earlier through their own stage gates. Learned cross-episode Pattern Mirror, cognitive ensembles, and Inquiry/Closure research follow Collaborative Intelligence v0.1 and product Stage 10 prerequisites (product Stage 4 evidence and Stage 6 governance). Ordinary author-requested critique does not require learned user-pattern inference.

Canonical creative changes use the governed Knowledge Service and Rikuri-owned transaction path. Generated interpretations, critique, patterns, and story alternatives remain candidates until authorized promotion; fictional canon is author-approved project truth, not an assertion about the external world.

## Executive Summary

Rikuri Pi should provide a first-class **Writer / Creative Authoring** system whose purpose is to strengthen human creativity rather than replace it. The system is not primarily an AI story generator. It is a flexible creative environment that helps users discover, organize, examine, challenge, revise, and express their own ideas across fiction, nonfiction, poetry, history, academic writing, screenwriting, stage writing, marketing, interactive fiction, games, and future multimedia forms.

The architecture should support radically different writing processes. A user may begin with a plot, character, image, object, scene, historical event, piece of dialogue, emotional tension, research note, ending, formal constraint, or free-form fragment. Rikuri should help the writer develop the work without forcing a single workflow.

The creative system should also be **timeless by design**. The durable object is not a DOCX file, screenplay format, video, model conversation, or application workspace. It is the user's persistent intellectual artifact: canonical text, storyworld, characters, relationships, events, themes, rules, constraints, provenance, rights, and author intent. Models, harnesses, interfaces, file formats, databases, devices, and presentation media operate on that artifact but do not own or define it.

> **Creative principle:** Rikuri expands the author's field of possibilities without deciding what the author must create.

> **Architectural principle:** The work persists; the machinery evolves.

> **Complexity principle:** Offer a rich creative system, but activate depth only where the author needs it.

> **Character principle:** Identity before function; let beliefs, duties, desires, relationships, and moral tensions drive choices.

> **World-character principle:** The world shapes the person; the person can reshape the world.

> **Versioned-work principle:** Preserve experimentation without forcing commitment; borrow version-control ideas without imposing version-control complexity.

> **Fresh-eye principle:** Rikuri should help the writer see the work from perspectives the writer is no longer able to access easily after prolonged immersion. It asks useful questions and teaches craft; creative-mode agents do not compose finished manuscript passages.

> **Closure principle:** Curiosity serves completion. Exploration budgets shrink as a work approaches delivery; late-stage questions are triaged by whether they materially threaten or strengthen the finished work.

---

# Part I - Architecture Hierarchy

## 1. Layer A - Durable Creative Core

This is the layer that must survive changes in models, vendors, operating systems, editors, storage engines, and media formats.

### 1.1 Canonical creative objects

Rikuri should be able to represent durable objects such as:

- projects and works;
- manuscripts and canonical passages;
- scenes, chapters, sequences, acts, stanzas, beats, or other structural units;
- characters, creatures, groups, organizations, and relationships;
- places, regions, worlds, planets, environments, and spatial relationships;
- events and chronology;
- objects, symbols, motifs, and recurring images;
- world rules and constraints;
- themes and tensions;
- research sources and factual notes;
- audience assumptions;
- author intent and protected creative choices;
- adaptation permissions;
- version history and provenance;
- branches, alternatives, and derived works.

These objects should use stable identifiers and versioned metadata so they remain portable over time.

### 1.2 Canon versus derivation

The architecture must clearly separate:

- **canonical source** - the author's authoritative work;
- **draft alternatives** - exploratory possibilities;
- **AI suggestions** - proposals that remain distinguishable from author text;
- **derived adaptations** - screenplay, audio, game, interactive, translation, or other branches;
- **reader-created branches** - optional transformations allowed by the author.

Derived artifacts never silently overwrite canon.

### 1.3 Provenance and authorship

Rikuri should preserve enough provenance to answer:

- Who wrote this?
- Was this user-authored, imported, quoted, or AI-suggested?
- What version did it come from?
- What changed?
- Which branch or adaptation produced it?
- Which author permissions apply?

The system should protect authorship without turning ordinary writing into a bureaucratic process.

---

## 2. Layer B - Common Authoring Engine

The Common Authoring Engine provides services shared across writing forms.

### 2.1 Core services

- drafting and revision;
- fragments and idea capture;
- structure and reordering;
- timeline and chronology;
- entity and relationship tracking;
- search and semantic retrieval;
- notes and research integration;
- continuity and contradiction checking;
- version comparison;
- comments and annotations;
- style and craft analysis;
- audience / reader lenses;
- provenance-aware AI assistance;
- export and publishing adapters.

### 2.2 Writing is a graph, not a pipeline

A conventional word processor assumes:

**Page 1 -> Page 2 -> Page 3**

Creative work is often closer to:

**character <-> scene <-> memory <-> theme <-> object <-> research <-> later scene <-> ending**

Rikuri should therefore support connected creative objects beneath a simple interface. The writer may never need to see the graph directly, but it enables continuity checks, timeline reasoning, storyworld exploration, adaptation, and future media.

### 2.3 No mandatory workflow

A project may begin with:

- plot or premise;
- character;
- scene;
- dialogue;
- image or photograph;
- object or symbol;
- historical event;
- research finding;
- emotion or tension;
- voice;
- theme;
- ending;
- formal constraint;
- fragment;
- free-form exploratory writing.

Rikuri should adapt to planners, discovery writers, hybrids, and writers who change methods from project to project.

### 2.4 Versioned Work / Change Management

Writing, coding, research, and other knowledge work share a durable change-management pattern:

**plan -> create -> branch -> compare -> review -> revise -> merge -> preserve history**

Rikuri should implement these as shared platform primitives and present them in domain-appropriate language rather than reproducing GitHub as a product.

For writers, the core capabilities include:

- **snapshots / checkpoints** - preserve a known state before a major revision;
- **branches / alternate versions** - explore a different scene, chapter order, ending, point of view, tone, side story, or structural experiment without destroying the current version;
- **semantic diffs** - show not only changed words, but changed scenes, facts, relationships, chronology, character knowledge, tone, and structural consequences;
- **revision proposals** - a writer-friendly analogue to a pull request: package a proposed change for human or AI review before accepting it;
- **review and annotation** - invite critique, comments, or fresh-context evaluation without modifying canon;
- **merge** - combine useful material from parallel drafts or branches;
- **semantic conflict detection** - identify narrative conflicts even when the text itself does not overlap, such as incompatible timelines, motivations, object states, or character knowledge;
- **rollback** - return safely to an earlier state while preserving later work;
- **provenance** - record which changes were author-written, imported, AI-suggested, reviewed, accepted, or rejected.

A textual merge that succeeds mechanically may still fail creatively. Rikuri should therefore distinguish **text conflicts** from **meaning conflicts**.

> **Shared-platform principle:** Borrow version-control primitives, not version-control complexity.

Git and GitHub integration may be offered as optional adapters for users who want them, but creative work must never depend on GitHub or require writers to learn developer terminology.

---

## 3. Layer C - Writer's Studio: Linked Views

The Writer's Studio is the primary user-facing workspace. Different windows represent the same underlying work rather than separate copies.

| Workspace | Purpose |
|---|---|
| Manuscript | Canonical authored text or script |
| Fragments / Scratchpad | Dialogue, images, observations, objects, phrases, unplaced scenes |
| Scene / Beat Board | Spatial organization and rearrangement |
| Outline | Hierarchical or sequential structure |
| Timeline | Chronology across events, places, and characters |
| Character / World Bible | People, creatures, places, relationships, terminology, rules |
| Continuity View | Contradictions, dates, objects, knowledge-state errors |
| Theme / Motif View | Recurring images, ideas, tensions, symbols, emotional arcs |
| Research | Sources, factual notes, references, quotations, provenance |
| Style Lab | Voice, rhythm, syntax, punctuation, formal techniques |
| Reader Lens | Audience assumptions, comprehension, pacing, accessibility |
| Revision View | Structural, scene-level, line-level, and consistency passes |
| World Creation View | Geography, culture, systems, species, history, civilization |
| Narrative Canvas | Text plus image, sound, motion, data, maps, interaction |

The user should be able to open several linked views at once and arrange them freely or in optional grid layouts.

---

# Part II - Creative Intelligence Modules

## 4. Writer's Technique Library

The technique library should function like a programming library for expression: a catalog of reusable methods rather than a menu of celebrity imitation.

### 4.1 Technique families

- point of view and narrative distance;
- stream of consciousness;
- free indirect discourse;
- unreliable narration;
- compression and omission;
- defamiliarization;
- dialogue and subtext;
- scene versus summary;
- pacing and temporal manipulation;
- foreshadowing and delayed revelation;
- motif and symbolic recurrence;
- metaphor and analogy;
- syntax and sentence architecture;
- repetition, anaphora, parallelism;
- meter, rhyme, lineation, caesura;
- dramatic beats and reversals;
- rhetorical structure and persuasion.

Named writers may be used as historical or analytical case studies, but the system should teach **what the technique does** rather than encourage reproduction of a particular author's voice.

### 4.2 Style as dimensions

Rikuri can help authors inspect dimensions such as:

**sentence length - syntactic complexity - narrative distance - concreteness - abstraction - dialogue density - metaphor density - rhythm - vocabulary register - exposition density - interiority - description density - emotional explicitness**

The purpose is not to score the writer. It is to make choices visible.

---

## 5. Expressive Mechanics

Creative writing often gains power by deliberately violating conventional grammar or page structure. Rikuri must distinguish **accidental error** from **intentional effect**.

The Style Lab should recognize and teach uses of:

- commas as hesitation, accumulation, simultaneity, or breath;
- semicolons as linkage, suspension, or controlled accumulation;
- dashes as interruption, rupture, intrusion, or acceleration;
- ellipses as omission, hesitation, fading, or silence;
- colons as revelation, expansion, or pivot;
- fragments as impact, perception, or emotional fracture;
- run-ons as urgency, overflow, panic, or continuity;
- paragraph breaks and white space as time, silence, isolation, or emphasis;
- capitalization and typographic variation as expressive devices;
- repeated punctuation or syntax as rhythm.

A proofreading pass should be able to say **nonstandard but plausibly intentional** rather than automatically normalizing the author's prose.

---

## 6. Creative Expansion & Insight

Rikuri should help writers break blocks and surface ideas without claiming to know the author's hidden psychology.

### 6.1 Divergent exploration

- orthogonal prompts;
- inversion;
- counterfactuals;
- unexpected analogy;
- association maps;
- time shifts;
- distance / viewpoint shifts;
- constraint play;
- medium shifts;
- deliberate tangent exploration.

### 6.2 Convergent reflection

Rikuri can ask:

- What keeps recurring?
- What is nobody saying?
- What contradiction drives this scene or person?
- Which image or object carries emotional weight?
- What changed between the beginning and end?
- What does the character want versus what they believe about themselves?
- What might be underneath this passage?

These are hypotheses and mirrors, not diagnoses. The author decides whether they resonate.

---

## 6A. Inquiry, Pattern Mirror & Closure Lab

Writer reuses the platform-wide Collaborative Intelligence substrate but applies creative-domain semantics. The purpose is not to score the author or autonomously rewrite the work. It is to surface patterns, questions, counterfactuals, forgotten intent, and alternative perspectives that give the writer useful material to consider.

### 6A.1 Pattern Mirror

Across authorized works, Rikuri may detect recurring craft choices such as plot structures, character roles, sentence/rhythm tendencies, conflict mechanisms, point of view, recurring resolutions, or repeated emotional moves. These are labeled as candidate signature/habit/emerging-pattern observations, not defects. The author can preserve, invert, exaggerate, or ignore them. Evidence links and counterexamples remain available.

### 6A.2 Perspective portfolio

A manuscript may be reread through independent lenses: character causality, plot causality, reader knowledge, world consequences, theme, setup/payoff, convention challenge, counterfactual exploration, and fresh-reader confusion/surprise. Different models/roles/prompts may be used where they provide genuine methodological diversity. Independent passes should not see prior pass conclusions until the comparison stage when independence matters.

### 6A.3 Ending / Closure Lab

Near completion, Rikuri can return to original intent, early promises, character starting states, themes, unresolved causal chains, and later manuscript drift. It asks whether the ending follows from who the characters became, whether the protagonist causes the resolution, whether emotional and plot problems both receive appropriate closure, what must remain unresolved, and whether a familiar ending is serving this story or merely habit/genre convention. It proposes questions/ending shapes before prose.

### 6A.4 Completion throttle

Generated questions are triaged as blocking, high-value, exploratory, curiosity, future-work, or resolved/dismissed. Early stages permit broad exploration; final revision surfaces only issues likely to materially improve/save the work. Resolved/dismissed questions remain remembered so Rikuri does not repeatedly reopen intentional choices.

## 7. Continuity, Consistency & Fresh-Eyes Review

Writers become accustomed to their own work. Rikuri should provide review passes that expose possible issues without dictating artistic choices.

Possible passes include:

- continuity: names, ages, dates, geography, injuries, possessions, weather, chronology;
- character knowledge: what each character knows at each point;
- character consistency: motivation, behavior, voice, unexplained changes;
- causality: whether events follow plausibly or exist only for plot convenience;
- promise/payoff: setups without resolution and resolutions without preparation;
- point of view: head-hopping, unavailable information, unstable narrative distance;
- voice drift: unexplained linguistic or tonal shifts;
- pacing: long low-change stretches, exposition clusters, abrupt compression;
- thematic continuity: motifs that disappear or shift meaning unintentionally;
- canon integrity: contradictions between current work and established storyworld facts.

A fresh-context AI model or human reviewer can provide an independent pass. The reviewer should show evidence and questions rather than issue a verdict.

---

## 8. Character Identity, Archetype & Moral Tension

Characters should be modeled first as persons with an internal structure, not merely as plot functions.

> **Identity before function.** A character is not interesting because they are "the mentor," "the villain," "the ruler," or "the hero." Plot function should emerge from who the person is, what they believe, what they want, what they owe, and what they are willing to sacrifice.

### 8.1 Character identity model

Rikuri should be able to represent, where useful:

- beliefs and worldview;
- moral code and taboos;
- epistemology - what the person accepts as evidence or knowledge;
- metaphysics - what the person believes is real or possible;
- loyalties and obligations;
- social position, role, class, profession, and status;
- culture, religion, philosophy, or secular commitments;
- fears, desires, ambitions, attachments, and resentments;
- habits, temperament, language, and aesthetic preferences;
- personal history and formative experiences;
- contradictions between professed values and actual behavior;
- lines the character believes they will not cross;
- conditions under which those lines may be tested.

### 8.2 Archetypes as lenses, not molds

Archetypes such as sage, mentor, warrior, ruler, martyr, trickster, rebel, saint, outcast, lover, or destroyer can help a writer recognize recurring human patterns. They should never become rigid templates.

A sage may be wise but cowardly. A warrior may hate violence. A saint may be vain. A ruler may crave anonymity. A skeptic may be deeply compassionate. A believer may doubt. A villain may possess a coherent moral philosophy.

The archetype identifies a pattern; culture, history, temperament, relationships, and contradiction make the character particular.

### 8.3 Character Tension Map

Rikuri should help the author examine forces acting on the same person at once:

| Dimension | Question |
|---|---|
| **Identity** | Who does this person believe they are? |
| **Desire** | What do they want badly enough to act? |
| **Duty** | What do they owe family, office, community, country, faith, profession, or promise? |
| **Morality** | What do they believe is right or wrong? |
| **Fear** | What loss, humiliation, failure, abandonment, death, or meaninglessness do they dread? |
| **Relationship** | Whose love, approval, rivalry, pressure, or betrayal changes the decision? |
| **World pressure** | What do law, hierarchy, custom, economics, war, technology, or circumstance demand? |
| **Price** | What must they sacrifice to obtain what they want? |
| **Red line** | What do they refuse to sacrifice - until the story tests it? |

A useful creative question is:

> **What will this person sacrifice first, and what will they refuse to sacrifice until the very end?**

The strongest drama often occurs when several legitimate commitments become mutually incompatible.

### 8.4 Moral conflict rather than simple alignment

Rikuri should be able to surface conflicts such as:

- ambition versus conscience;
- duty versus love;
- loyalty versus justice;
- faith versus doubt;
- freedom versus obligation;
- honor versus survival;
- mercy versus vengeance;
- truth versus belonging;
- desire versus restraint;
- public responsibility versus private loyalty.

These tensions should not be reduced automatically to hero-versus-villain morality. Two sincere moral commitments can conflict, and a character can make a catastrophic choice while believing it is the honorable one.

Shakespearean tragedy is a useful craft case study: Macbeth can be read through ambition, loyalty, conscience, marriage, prophecy, fear, and status; Hamlet through duty, morality, uncertainty, grief, love, and reflection; Brutus through public duty and personal loyalty; Lear through authority, pride, aging, family, inheritance, and loss of identity. Rikuri should use such examples to analyze patterns, not to reproduce plots or voices.

### 8.5 Portable conflict, world-specific expression

Some human tensions can travel across eras and cultures, but their expression should belong to the world in which the character lives.

> **The archetypal conflict may be portable; its expression must belong to the world.**

Ambition, loyalty, grief, jealousy, honor, mercy, prejudice, love, vengeance, and fear may recur across human history, but what those ideas mean - and what choices are available - depend on institutions, beliefs, technologies, social roles, laws, family structures, and material conditions.

A useful causal loop is:

**world conditions -> beliefs -> identity -> choices -> consequences -> changed world conditions**

---

## 9. Reader Lens & Audience Model

Rikuri should model reader perspectives as structured questions, not pretend to predict market success.

Possible lenses include:

- first-time reader;
- genre-experienced reader;
- casual reader;
- subject-matter expert;
- young reader;
- international / second-language reader;
- audiobook listener;
- accessibility perspective;
- specific customer or market segment.

Useful output is concrete and testable:

> "This chapter assumes knowledge introduced 100 pages earlier."

Less useful output is unsupported prediction:

> "Readers will love this."

Actual beta readers, editors, audiences, and market tests remain more authoritative than synthetic reader simulation.

---

# Part III - Story Extension & Interactive Narrative

## 10. Story Extension Diagnostic

Sequels, prequels, spin-offs, adaptations, and games should be treated as optional derived works, not automatic content opportunities.

Before helping create an extension, Rikuri should ask:

> **What does this new work reveal, explore, or let the audience experience that the original could not?**

### 10.1 Extension risks

- **Closure damage** - the original arc was complete and continuation weakens it.
- **Character reset** - previous growth is undone merely to create another conflict.
- **Mystery depletion** - a prequel explains what was more powerful when unknown.
- **Inevitability problem** - the audience already knows the destination.
- **Canon burden** - continuity requires awkward retcons or contradictions.
- **Escalation trap** - every sequel merely makes the threat larger.
- **Nostalgia substitution** - familiar imagery replaces a new dramatic idea.
- **Theme repetition** - the extension asks the same question and gives the same answer.
- **Tone drift** - emotional identity changes unintentionally.
- **Audience expectation debt** - the extension inherits the standard set by the original.

### 10.2 Prequel test

A prequel should do more than fill a fictional encyclopedia. It should create meaningful uncertainty despite a known endpoint and should preserve mysteries whose ambiguity is artistically valuable.

### 10.3 Sequel test

A sequel should explore **consequence rather than repetition**. The prior character transformation should ideally create new problems rather than be reset.

### 10.4 Spin-off test

A spin-off should reveal a part of the world, class, culture, perspective, profession, era, or conflict the original protagonist could not show.

### 10.5 Franchise litmus test

> **If the famous title, nostalgia, and recognizable names disappeared, would there still be a story worth experiencing?**

---

## 11. Contingency, Surprise & Predictability

Coherence is necessary, but over-determined stories become predictable. Rikuri should help writers introduce plausible contingency without replacing causality with arbitrary twists.

Possible sources of genuine disruption include:

- incomplete information;
- asymmetric knowledge;
- believable miscommunication;
- timing;
- institutional friction;
- irrational or emotionally inconsistent human decisions;
- fatigue, fear, shame, pride, love, envy, grief;
- accident and environmental disruption;
- unexpected competence;
- unexpected weakness;
- hidden but previously compatible facts;
- temporary alliances created by changing incentives.

The goal is **surprising but retrospectively believable**.

> **Surprise should violate expectation, not the integrity of the world.**

For story extensions, Rikuri should also ask:

> **What new pressure exists here that did not exist in the original, and does it force genuinely new choices?**

---

## 12. Game & Interactive Adaptation

Games are not merely stories with buttons. Rikuri should translate world conflicts into player systems rather than mechanically convert chapters into levels.

Important questions include:

- What does the player repeatedly do?
- Do those mechanics reinforce the themes?
- Are player choices meaningful?
- Does the world remember consequences?
- Are fictional rules and gameplay rules consistent?
- Can player freedom force the protagonist to behave incompatibly with canon?
- What can interaction reveal that prose or film cannot?

A story about mistrust may become systems of alliances, incomplete information, reputation, betrayal risk, and shared resources. The goal is medium-specific expression rather than plot transcription.

---

# Part IV - World Creation Engine

## 13. World Creation as Interacting Systems

A world is not scenery. It is a network of interacting physical, biological, social, material, cultural, institutional, and psychological systems.

Rikuri should help build worlds across:

- different historical periods;
- regions and cultures;
- alternate histories;
- planets and habitats;
- fantasy realms;
- species and non-human societies;
- galactic or multi-world civilizations.

### 13.1 World dimensions

The engine may support dimensions such as:

- geography and climate;
- ecology and environmental constraints;
- species and biology;
- resources and energy;
- food and agriculture;
- materials;
- shelter and architecture;
- clothing and fashion;
- technology;
- transportation and logistics;
- communication and media;
- economy, ownership, trade, and labor;
- government, politics, law, and institutions;
- military, security, and conflict;
- religion, theology, philosophy, nonbelief, morality, mythology, mysticism, and superstition;
- language and writing;
- family, kinship, sex/gender, and social roles;
- race, ethnicity, caste, class, or other identity structures where relevant;
- age, generation, childhood, adulthood, and aging;
- education;
- medicine and health systems;
- art, ritual, entertainment, and aesthetics;
- psychology and social norms;
- infrastructure;
- demographics and migration;
- everyday habits and domestic life.

### 13.2 Selective depth

Authors should **not** be required to model every dimension.

Rikuri should let the author select the few dimensions that matter most and expand only where consequences naturally propagate into character, plot, theme, or setting.

> **Offer a rich world model, but let the author choose where the world has depth.**

---

## 14. Cross-System Causality

Worldbuilding becomes believable when changes propagate.

For example:

**fuel scarcity -> expensive transportation -> denser settlements -> changed trade -> changed food access -> changed political power -> changed conflict**

Or:

**cheap fusion energy -> desalination -> agricultural expansion -> migration -> new cities -> geopolitical realignment -> different forms of scarcity**

Rikuri should help surface such second-order and third-order consequences without forcing one deterministic outcome.

### 14.1 Reskinning diagnostic

Rikuri should ask:

> **If names and visual decoration were removed, would these two cultures or worlds still behave differently?**

and:

> **What behavior exists here that could plausibly emerge only from this world?**

This helps prevent "same society, different costume" worldbuilding.

---

## 15. Temporal, Regional & Planetary Drift

Time and distance should matter.

A world 100 years earlier or later should usually show meaningful changes in some combination of:

- technology;
- architecture;
- fashion;
- language;
- institutions;
- warfare;
- media;
- medicine;
- transportation;
- energy;
- social norms;
- economic structure;
- everyday life.

If something does not change, that stability should preferably have a reason: regulation, resource limits, religious prohibition, institutional lock-in, collapse, deliberate preservation, or some other force.

Regional variation should also be explicit. A planet does not need one culture any more than Earth does.

Useful hierarchy:

**world -> region -> culture -> community -> household -> individual**

---

## 16. Belief, Meaning & Moral Systems

Religion, theology, philosophy, nonbelief, mythology, superstition, and morality should be treated as structural forces in a world rather than decorative lore.

They help determine what people think is real, sacred, shameful, honorable, forbidden, just, beautiful, corrupt, or worth dying for. They can shape law, family, government, education, art, war, death practices, sexuality, legitimacy, social cohesion, rebellion, and personal identity.

### 16.1 Belief and nonbelief as first-class worldviews

Rikuri should be able to represent:

- religions and theological traditions;
- gods, deities, cosmologies, creation accounts, salvation, sin, virtue, ritual, clergy, and sacred texts;
- philosophy and major schools of thought;
- atheism, agnosticism, deism, scientific naturalism, skepticism, secular humanism, existentialism, stoicism, and other secular or mixed positions;
- orthodoxy, reform movements, sects, schisms, syncretism, heresy, folk practice, and private belief;
- differences between official doctrine and lived practice;
- doubt, conversion, apostasy, indifference, and intellectual independence.

Nonbelief is not merely an absence of religion. It may include strong commitments about evidence, reality, ethics, autonomy, or the limits of knowledge.

Rikuri should distinguish at least four questions:

- **Epistemology:** What counts as evidence or knowledge?
- **Metaphysics:** What does the person or culture believe exists or could exist?
- **Ethics:** What makes an act right, wrong, virtuous, shameful, or obligatory?
- **Identity / loyalty:** Which community, tradition, philosophy, profession, or self-conception does the person defend?

A scientist need not reject spirituality; a believer need not reject science; a skeptic need not be isolated; an independent thinker is not automatically correct. These should remain character possibilities rather than stereotypes.

### 16.2 Mythic, symbolic & archetypal psyche

Mysticism, ghosts, monsters, devils or demons, angels, saints, deities, mythical figures, heroes, sacred sexuality, violence, sacrifice, death, purity, corruption, taboo, and transcendence can embody deep cultural fears, desires, aspirations, and moral tensions.

Rikuri should help an author ask:

> **What does this figure mean to the people inside this world?**

The same symbol may carry radically different meanings across cultures. A ghost might represent unfinished obligation, pollution, ancestry, terror, memory, protection, or something else entirely. A dragon, saint, demon, ancestor, or hero should not be flattened into a universal generic archetype.

Recurring tensions may include:

- pride versus humility;
- prejudice versus recognition;
- good versus evil;
- love versus war;
- mercy versus vengeance;
- order versus chaos;
- freedom versus duty;
- desire versus restraint;
- mortality versus transcendence;
- purity versus corruption;
- belonging versus truth.

These are lenses for exploration, not fixed universal binaries.

### 16.3 Belief as behavior generator

Belief systems matter because they affect choices.

A culture that treats filial duty as morally prior to individual desire will generate different character decisions from one that treats personal autonomy as sacred. A warrior culture that sees surrender as dishonorable may fight differently from one that treats survival as duty. A society that regards ancestors as continuing participants in communal life may structure inheritance, law, property, and ritual differently.

Rikuri should therefore ask:

> **What would a good person in this culture believe they are required to do?**

and:

> **What would another culture consider wrong about that same act?**

Belief systems should make sense from inside the worldview of their adherents rather than being judged only from an external contemporary perspective.

---

## 17. World-Character Co-Evolution

Characters emerge from nested contexts while retaining room for variance, dissent, mutation, accident, and idiosyncrasy.

A useful hierarchy is:

**civilization -> region -> culture -> institution -> community -> family -> immediate circumstance -> individual**

Each layer creates pressures and probabilities, but none mechanically determines the person.

> **Context creates tendencies, not destiny.**

Two people raised under similar conditions may diverge because of temperament, unusual experiences, mixed identities, migration, relationships, trauma, opportunity, innovation, chance, or different interpretations of the same event.

Rikuri should explicitly preserve space for:

- dissenters and reformers;
- cultural hybrids;
- migrants and outsiders;
- converts and apostates;
- innovators and eccentrics;
- people who reject expected social roles;
- local variations in supposedly uniform traditions;
- accidents and encounters that change a life.

The causal relationship also runs in the opposite direction. Individuals and small groups can alter institutions, norms, technologies, beliefs, borders, languages, and historical trajectories.

A local variation may become:

**variation -> idea -> followers -> institution -> social change -> new world conditions**

The core creative loop is therefore:

> **The world shapes the person; the person can reshape the world.**

or, more operationally:

> **Context creates pressure. Character creates response. History emerges from the interaction.**

This reciprocal model should connect character development directly to the World Creation Engine rather than treating character sheets and worldbuilding as separate activities.

---

## 18. Creatures, Species & Non-Human Perspectives

Characters and creatures should emerge from their environments rather than being humans with cosmetic alterations.

Rikuri can explore:

- sensory systems;
- lifespan;
- reproduction;
- locomotion;
- metabolism;
- communication;
- social organization;
- cognition;
- sleep;
- memory;
- territoriality;
- dependence on ecology;
- tool use;
- relationships between biology and culture.

A species that communicates chemically may have different concepts of privacy. A centuries-long lifespan may produce different attitudes toward education, risk, government, inheritance, and commitment. Distributed cognition may alter the meaning of individuality.

> **World shapes character; character changes world.**

---

## 19. Civilizational Dynamics

Worlds should be able to progress, regress, stagnate, fragment, collapse, recover, experience renaissance, develop unevenly, or move in mixed directions.

Rikuri should not default to either static worlds or dystopia.

### 19.1 Drivers of civilizational change

- energy and resources;
- technology;
- institutions;
- incentives;
- education;
- trade;
- conflict;
- demographics;
- ecology and climate;
- health;
- belief systems;
- information flow;
- inequality;
- migration;
- shocks and disasters;
- innovation;
- adaptation.

Progress in one domain may coexist with regression in another.

A civilization could be technologically advanced but politically repressive, materially abundant but psychologically insecure, socially egalitarian but ecologically fragile, or culturally vibrant during economic decline.

### 19.2 Beyond dystopia

Rikuri should support worlds involving:

- broad prosperity;
- regenerative development;
- renaissance;
- post-scarcity or partial post-scarcity;
- pluralistic societies;
- frontier cultures;
- local self-sufficiency;
- high-trust institutions;
- cyclical development;
- asymmetric regional progress;
- recovery after collapse;
- mixed success and failure.

> **Progress should create new problems, not merely remove old ones. Regression should have causes, not merely atmosphere.**

---

## 20. Scenario Foundry / Transition Paths

A future should contain the history of how it became possible.

Rikuri should help writers move both forward and backward between present conditions and future states.

Possible interacting variables include:

- technology capability and cost;
- energy availability;
- ownership;
- labor markets;
- education;
- distribution systems;
- regulation;
- institutions;
- culture;
- demographics;
- infrastructure;
- shocks;
- adaptation;
- feedback loops.

### 20.1 Multiple futures from the same starting point

The same technology can lead to different civilizations depending on ownership, institutions, values, accidents, and policy.

For example, advanced AI might contribute to:

- broad abundance with distributed ownership;
- abundance with concentrated control;
- painful displacement followed by adaptation;
- regional divergence;
- stagnation after technical or energy bottlenecks;
- local empowerment through personal AI and fabrication;
- new dependency on a few infrastructure providers.

Rikuri should identify **branch points** rather than assert one inevitable future.

Optimistic futures should still include transition pain, tradeoffs, losers, new scarcity, and unintended consequences. Pessimistic futures should still allow adaptation, resistance, institutional learning, and recovery.

---

## 21. Political Economy & Power Dynamics

Abundance, equality, ownership, access, status, and power must be modeled as distinct variables.

A society may be:

| Material conditions | Ownership / agency | Possible character |
|---|---|---|
| High abundance | Broadly distributed | High autonomy, broad prosperity, new status conflicts |
| High abundance | Highly concentrated | Comfortable access with dependency and concentrated power |
| Low abundance | Broadly distributed | Shared scarcity, resilient commons, constrained opportunity |
| Low abundance | Highly concentrated | Scarcity plus dependency and potentially intense conflict |

Rikuri should explore:

- who owns productive assets;
- who controls access;
- who bears transition costs;
- what remains scarce;
- how rents are extracted;
- how legitimacy is maintained;
- whether institutions broaden or narrow agency;
- whether formerly powerless groups become powerful;
- whether reform, revolt, capture, reconciliation, or elite turnover follows;
- whether conflict is zero-sum, positive-sum, or changes character over time.

Historical cycles are possible patterns, not predetermined laws.

A useful scenario exercise is:

> **Hold the technology constant and change ownership / institutions. What civilization results?**

Then reverse it:

> **Hold institutions constant and change the technology.**

---

# Part V - Genre and Medium Modules

## 22. Common Engine, Specialized Craft Modules

| Form | Shared Core | Specialized Needs |
|---|---|---|
| Novel / fiction | Voice, character, scene, structure, revision | Long continuity, POV, world rules, foreshadowing |
| Short story / essay | Voice, structure, audience, compression | Economy, focus, rapid setup/payoff |
| Poetry | Image, voice, rhythm, revision | Meter, stress, sound, spacing, lineation, formal constraints |
| History / narrative nonfiction | Narrative, chronology, audience | Provenance, competing accounts, factual uncertainty |
| Academic | Structure, clarity, argument | Evidence, citation, methods, claim calibration |
| Screenplay | Character, dialogue, scene, structure | Visual action, beats, runtime, production constraints |
| Stage play | Character, dialogue, scene | Physical stage, entrances/exits, performability |
| Marketing | Voice, audience, persuasion | Brand, channel, behavioral objective, testing |
| Interactive fiction | Character, scene, narrative | Branches, state, player agency, alternate outcomes |
| Game narrative | World, character, conflict | Mechanics, agency, state, systemic consequences |
| Multimedia story | Narrative, voice, pacing | Image, sound, motion, timing, interaction, medium transitions |

Modules should plug into the common architecture rather than create isolated applications.

---

## 23. Narrative Canvas / Multimedia

Modern storytelling can combine:

- prose or script;
- image, illustration, and photography;
- ambient audio and sound;
- voice and narration;
- music cues;
- video and motion;
- maps and spatial information;
- data visualization;
- chronology;
- character state;
- objects and symbols;
- branching and interaction;
- reader knowledge and intended experience.

The guiding rule is **medium intentionality**: use a medium because it contributes meaning, not simply because it is available.

---

## 24. Adaptive Narrative / Transformative Storyworlds

Long-term, a creative work may become an explorable storyworld rather than only a fixed text.

An author may choose to permit transformations such as:

- novel -> stage play;
- prose -> audio drama;
- fixed story -> interactive branch;
- viewpoint changes;
- different eras or settings;
- language or presentation changes;
- reader-directed exploration;
- spatial / immersive experiences;
- future presentation media not yet invented.

The author should be able to define:

- **Canonical elements** - immutable facts, relationships, themes, rules, or events.
- **Flexible elements** - details that may change within limits.
- **Transformable presentation** - medium, language, pacing, viewpoint, visual treatment.
- **Branchable events** - outcomes the audience may influence.
- **Forbidden transformations** - changes the author disallows.
- **Divergence budget** - how far a branch may move from canon.

> **The author defines the world and its boundaries; the audience may explore its possibilities.**

---

# Part VI - Timeless Technical Architecture

## 25. Durable Layer vs. Replaceable Layer

### Durable

- canonical creative content;
- storyworld graph;
- characters and relationships;
- events and chronology;
- world rules;
- author intent;
- provenance;
- permissions and rights;
- research and source relationships;
- branches and derivations;
- stable IDs and version history.

### Replaceable adapters

- AI models and providers;
- model harnesses;
- editors and UI layouts;
- databases and indexes;
- vector stores and embedding models;
- file formats;
- operating systems;
- hardware;
- publishing platforms;
- text / audio / video / AR / VR / spatial presentation layers.

### Architectural invariant

> **Models and harnesses operate on the user's intellectual artifacts; they do not own or define them.**

A useful design test:

> **If this technology disappears in five years, does the user's work still make sense?**

---

## 26. Integration with Rikuri Knowledge Architecture

The Writer system should use the same broader Rikuri principles already intended for knowledge and research:

- Knowledge-as-a-Service behind stable interfaces;
- information as evidence/data rather than permanent truth;
- provenance and supersession;
- open/portable schemas;
- stable IDs;
- versioned contracts;
- capability-based access;
- separation between durable content and model-specific behavior;
- migration and export paths.

Creative works should therefore be able to outlive any particular Rikuri implementation.

### 26.1 Shared Versioned Work Service

Versioned Work / Change Management should be a Rikuri platform service rather than a Writer-only feature. The same underlying change graph can support multiple domains while each workspace supplies its own validation and vocabulary:

- **Code:** branch -> test -> review -> merge;
- **Writing:** branch -> critique -> continuity check -> merge;
- **Research:** branch hypothesis or analysis -> compare -> revalidate -> promote;
- **Documents / business work:** draft -> comment -> revise -> approve.

The shared service should provide stable version nodes, branches, diffs, annotations, review proposals, merge operations, rollback, provenance, and domain hooks for semantic conflict detection.

For writing, a merge validator may detect narrative contradictions; for code, the validator may run tests; for research, it may check evidence and claim dependencies. The machinery is common, but the meaning of "safe to merge" is domain-specific.

Git repositories and GitHub workflows may connect through adapters when useful, but they remain replaceable infrastructure rather than dependencies of the durable work.

---

# Part VII - Scope Control & Roadmap

## 27. Complexity Control Rules

The richness of the design must not turn into implementation complexity for its own sake.

### 27.1 Progressive disclosure

Expose only the tools relevant to the user's current work. A poet should not see galactic political-economy controls unless requested. A short essay writer should not need a world graph.

### 27.2 Optional modules

World creation, game adaptation, multimedia, reader simulation, and adaptive storyworld functions should be independent modules around a stable core.

### 27.3 Compute on demand

Do not continuously analyze every possible dimension. Run deeper continuity, world, audience, or adaptation analysis when triggered by the user, project state, or meaningful changes.

### 27.4 No forced completeness

A writer may intentionally leave a culture, motive, timeline, or mechanism undefined. Rikuri can flag consequences without requiring encyclopedic worldbuilding.

### 27.5 Questions before generation

Creative-mode agents help the user surface possibilities and tensions, explain techniques and offer short labeled illustrations. They must not compose finished scenes, chapters or substantial manuscript prose for incorporation. Business-production generation is separately declared and governed. See the October 1 clarification and linked Writer IDE specification.

---

## 28. Roadmap Hierarchy

### Foundation - Durable Creative Model

Build first because every later feature depends on it.

- stable creative-object IDs;
- canonical versus derived content;
- provenance;
- version history;
- relationships among scenes, characters, places, events, themes, and sources;
- open/exportable representation;
- adapter boundary for models and editors;
- shared versioned-work graph with snapshots, branches, merge ancestry, rollback, and provenance;
- semantic-diff and domain-validation hooks.

### Phase 1 - Essential Writer's Studio

Goal: a genuinely useful writing environment without requiring advanced worldbuilding.

- manuscript/editor;
- fragments and scratchpad;
- outline and scene board;
- basic timeline;
- character / world notes;
- basic character tension and identity notes;
- linked windows;
- search;
- research pane;
- provenance-aware AI suggestions;
- author-controlled accept / reject;
- basic continuity checks;
- writer-friendly snapshots / checkpoints;
- alternate-version branches;
- draft comparison and rollback.

**Phase 1 success criterion:** A writer can complete a substantial work while remaining clearly the author.

### Phase 2 - Craft, Insight & Review

Add Pattern Mirror, perspective portfolios, fresh-eye question generation, and the Ending/Closure Lab using the shared Collaborative Intelligence evidence/cognitive-recipe substrate. These capabilities remain author-directed and question-first; they do not make autonomous canon changes.

- Writer's Technique Library;
- Style Lab;
- expressive mechanics;
- intentional-rule-breaking awareness;
- divergent / orthogonal ideation;
- subtext and thematic reflection;
- fresh-context critique;
- advanced continuity and character-knowledge checks;
- Character Tension Map and identity-before-function analysis;
- archetype-as-lens analysis;
- Reader Lenses;
- revision comparison;
- revision proposals and review workflow;
- semantic merge-conflict detection for continuity, chronology, character knowledge, and structure.

**Phase 2 success criterion:** Rikuri helps the writer see possibilities and problems they were too close to notice.

### Phase 3 - World Creation Engine

- world dimensions and selective-depth controls;
- regional / temporal variation;
- cross-system causality;
- belief, meaning, moral, and mythic systems;
- world-character co-evolution;
- species / creature design;
- reskinning diagnostic;
- civilizational dynamics;
- Scenario Foundry;
- political economy and power dynamics;
- progress / regression / recovery modeling.

**Phase 3 success criterion:** Worlds behave differently because their underlying conditions differ, not merely because their appearance changes.

### Phase 4 - Storyworld Extension & Genre Modules

- sequel / prequel / spin-off diagnostic;
- contingency and surprise analysis;
- canon / retcon tracking;
- screenplay and stage modules;
- poetry module;
- nonfiction / history / academic integration;
- marketing module;
- game narrative / mechanics alignment;
- collaboration with editors and beta readers.

**Phase 4 success criterion:** Rikuri can help expand or translate a work without automatically diluting the original.

### Phase 5 - Narrative Canvas / Multimedia

- image;
- sound;
- voice;
- video;
- maps;
- data;
- timing;
- interactive state;
- medium-aware adaptation;
- publishing and presentation adapters.

**Phase 5 success criterion:** The same canonical creative work can express itself coherently through more than one medium.

### Phase 6 - Adaptive Storyworlds / Future Media

- author-defined adaptation permissions;
- branchable canon;
- controlled reader agency;
- dynamic viewpoint / era / medium transformation;
- immersive and spatial experiences;
- future presentation adapters;
- storyworld experiences in media not yet invented.

**Phase 6 success criterion:** The creative work remains recognizably the author's even when audiences explore permitted transformations.

---

## 29. MVP Boundary

To prevent scope explosion, the initial Writer release should **not** require:

- full civilizational simulation;
- game engine functionality;
- automated film or 3D production;
- universal multimedia authoring;
- dynamic reader-controlled adaptation;
- complete market prediction;
- exhaustive world modeling.

Those are future modules.

The first release needs to excel at a smaller promise:

> **Help a human writer think, organize, preserve intent, see inconsistencies, explore craft, and revise a substantial work without surrendering authorship.**

That is sufficient to validate the architecture before adding deeper world and adaptation capabilities.

---

# Part VIII - Product Principles

## 30. Guardrails

1. **Human authorship first.** AI amplifies creativity; it should not silently replace the creator.
2. **No single correct process.** Rikuri adapts to planners, discovery writers, and hybrids.
3. **Technique over imitation.** Teach expressive mechanisms, not celebrity cloning.
4. **Intent before correction.** Nonstandard language may be deliberate craft.
5. **Possibilities, not prescriptions.** Surface alternatives and consequences; the author decides.
6. **Fresh eyes without false authority.** AI critique is a perspective, not a verdict.
7. **Audience matters.** Model reader questions but do not pretend to predict people precisely.
8. **Canon remains canonical.** Adaptations branch; they do not overwrite the source.
9. **Worldbuilding is systemic, not cosmetic.** Difference should create different behavior.
10. **History moves.** Worlds may progress, regress, fracture, recover, or develop unevenly.
11. **Abundance is not the same as equality.** Ownership, access, agency, and power remain separate variables.
12. **Surprise should remain believable.** Avoid both deterministic plotting and arbitrary twists.
13. **Selective depth beats encyclopedic burden.** Model what matters to the story.
14. **Identity before function.** A character's plot role should emerge from beliefs, desires, duties, relationships, contradictions, and choices.
15. **Archetypes are lenses, not molds.** Use recurring patterns to illuminate character, never to flatten individuality or culture.
16. **Belief is structural, not decorative.** Religion, philosophy, nonbelief, myth, and morality should affect behavior and institutions.
17. **Context creates tendencies, not destiny.** Worlds shape people while preserving variance, dissent, accident, and agency.
18. **World and character co-evolve.** The world shapes the person; the person can reshape the world.
19. **Content is durable; technology is replaceable.**
20. **Design for media we have not invented yet.**
21. **Borrow version-control primitives, not version-control complexity.** Shared snapshots, branches, diffs, review, merge, rollback, and provenance should feel native to each domain; Git/GitHub remain optional adapters.

---

## 31. Strategic Fit Within Rikuri Pi

The Writer / Creative Authoring track complements the Research & Evidence track.

- Research: **curiosity -> question -> experiment -> evidence -> understanding**
- Creative work: **intuition -> exploration -> form -> expression -> audience experience**

Both systems help users externalize complex thought, inspect it from multiple perspectives, detect contradictions, preserve provenance, and remain in control of final judgment. A shared Versioned Work service can support code, research, documents, and creative work while each domain supplies its own review and validation semantics.

The same deeper platform principle applies to the knowledge base, research artifacts, and creative works:

> **Knowledge and intellectual products remain agnostic from the platform, harness, model, runtime, and presentation technology. They persist.**

---

## 32. Research Signals Informing the Design

These are design signals rather than requirements to copy existing products.

1. **Scrivener** demonstrates strong demand for nonlinear drafting and multiple views of the same long-form project.
2. **Final Draft** combines script drafting with structural planning and beat-oriented views.
3. **ProWritingAid** shows value in craft-specific diagnostics rather than one generic writing-quality score.
4. **Authors Guild survey work** indicates that many authors value AI for grammar, brainstorming, organization, and marketing while remaining concerned about authorship, consent, and imitation.
5. **Twine** demonstrates nonlinear interactive storytelling with branches and state.
6. **Ren'Py** demonstrates established authoring that combines words, images, sound, and interaction.
7. **Knight Lab** demonstrates multimedia storytelling through timelines, maps, audio, data stories, and immersive media.
8. **Current reader research** continues to show substantial consumption across print, e-book, and audiobook formats, reinforcing the value of multiple audience and delivery lenses.

---

## Selected Sources / Research Notes

- Literature & Latte, *Integrating Scrivener's Binder, Corkboard, and Outliner* (2022; accessed 2026). https://www.literatureandlatte.com/blog/integrating-scriveners-binder-corkboard-and-outliner
- Final Draft, *Features* and current outline / Beat Board documentation (accessed 2026). https://www.finaldraft.com/products/features/
- ProWritingAid, *Product Features* / Writing Reports (accessed 2026). https://prowritingaid.com/features
- Authors Guild, *Survey Reveals 90 Percent of Writers Believe Authors Should Be Compensated...* (2023; accessed 2026). https://authorsguild.org/news/ai-survey-90-percent-of-writers-believe-authors-should-be-compensated-for-the-use-of-their-books-in-training-generative-ai/
- Twine, official project site (2026). https://twinery.org/
- Ren'Py, official visual novel engine site (2026). https://www.renpy.org/
- Knight Lab, storytelling tools and projects (accessed 2026). https://knightlab.northwestern.edu/projects/
- Pew Research Center, *Americans still opt for print books over digital or audio versions* (April 9, 2026). https://www.pewresearch.org/short-reads/2026/04/09/americans-still-opt-for-print-books-over-digital-or-audio-versions-few-are-in-book-clubs/

## September 30 — studio experience and Pi collaborators

The detailed creative architecture above remains controlling. Its depth is a differentiator, offered progressively rather than as a compulsory workflow. The studio accommodates a fragment-first writer, a planned outline and multiple concurrent storylines. Flexible Tauri-linked views share object identity and version history; opening another window never forks the underlying work accidentally. A return view shows the creator's current intent, last decisions, active alternatives, source changes and open questions, without requiring every fragment to be classified first.

StrawberryPi, ChocolatePi, PumpkinPi and ApplePi exemplify the collaborator family. Their flavors need not dictate capability or personality; creators can rename and configure roles. A Pi can serve as a research partner, continuity reader, world designer, character challenger or alternative-scene explorer. Identity and responsibilities are durable; model/tool selections are replaceable. Requested suggestions remain candidates and the creator controls canon. Ensemble disagreement is useful evidence, not a vote deciding literary truth. Avoid settings plumbing in ordinary creative flows: explain intended effects such as “explore wider alternatives” or “check consistency”; expose detailed recipes only on inspection.

A scenario packet links a stable candidate/branch ID, parent revision, creator goal, assumptions, events, affected characters/world rules, opportunities, risks, consequences, sources, unknowns and recipe/run evidence. Two or three alternatives are a useful default when requested and warranted, not a quota for arbitrary filler. Independent generation must preserve shared starting constraints; comparison identifies actual differences. Save a complete validated packet atomically through governed services, or mark partial results recoverable and incomplete. The creator may retain, reject, revisit or selectively merge packets. A merge proposes changes and conflicts across chronology, knowledge, motivations and world rules; it cannot just concatenate prose or quietly rewrite canon.

Science-backed worldbuilding starts with a sourced baseline and a stated question. A black-hole travel scene separates established relativistic effects, chosen parameters and calculation limits from an invented propulsion rule. A Mars scenario distinguishes evidence about human physiology from uncertain long-duration outcomes and invented adaptations. A domain model may explore consequences under explicit assumptions, without presenting a language-model expert persona as a scientific authority. The author can deliberately change physics, biology or social rules, then inspect the downstream constraints that change with them. Contradiction can be intentional; flag it for author interpretation rather than forbid it.

Game and animation packs develop branching narrative, levels, encounter constraints, character/visual references and world state into a versioned production specification. Actual playtesting, simulation and implementation validate their respective claims. External engines and coding ecosystems can consume that handoff. Marketing later adds optional launch/audience/research templates or creator-arranged workflows, product evidence and rights, message alternatives, taglines and asset revisions. Unsupported product claims cannot become verified facts; audience hypotheses remain hypotheses. Editable vectors/layers and supported illustration/design adapters matter more than promising that a raster generator can make precise logo edits.

**MVP boundary:** Stage 2 provides text-based shared objects and linked views, requested critique/alternatives, author-controlled revisions, resume/search/export and production briefs. Image attachments may be preserved as opaque source assets, but rich extraction, editable visual generation, science simulations and game engines are not implied. Advanced world engines build on this base at Stage 8. Advanced Pattern Mirror, structural learned retrieval and longitudinal reflection remain Stage 10 capabilities under CI prerequisites. The complete reflection/abstraction design is preserved in the Collaborative Intelligence document; none is removed to simplify the first studio.

## October 1 — Writer's IDE and learning-through-creation contract

The owner defines Rikuri as a writer's IDE: create, organize, revise, collaborate and learn in one human-authored workspace. The [Writer IDE Specification v0.1](Rikuri_Pi_Writer_IDE_Spec_v0.1.md) specifies WIDE-001–012, object/anchor contracts, agent permissions, recovery, rights agreements, learning records and future acceptance A01–A11. It refines STUDIO-001–010 without replacing this detailed creative architecture.

This dated clarification controls earlier references to replacement prose, alternative-scene exploration and scenario generation: creative agents explore non-prose structural alternatives, critique, research, explain craft and provide short labeled teaching examples; they do not write finished scenes/chapters or substantial manuscript passages for incorporation. Human authors write the work. Marketing/business generation is separately explicit and remains a later pack. Canon approval alone does not waive creative-mode composition limits.

Stable IDs identify sentences, paragraphs and story units independently of changing labels such as C10P4S2. Immutable revisions, split/merge/copy lineage and edition-specific page/line maps preserve references. Shared Versioned Work owns checkpoint/branch/workspace/proposal/merge and scoped deletion/restore; rights and current revocation still govern history. Reader experience and author intent guide optional craft questions. Contextual prompt/voice questions, saved lessons and flashcards support learning without mandatory tutoring or autonomous trait inference. Voice, full scholarly publishing and multi-human collaboration wait for their owning adapters/gates.

Product Stage 2 S2.C1–C4 owns the text-first solo slice; Stage 6 owns multi-human/agent collaboration, Stage 8 richer domains/publishing/voice, and Stage 10 learned longitudinal inference. All runtime claims remain Not yet proven. See [the Build Record](../docs/build-records/2026-10-01-writer-ide-spec.md).
