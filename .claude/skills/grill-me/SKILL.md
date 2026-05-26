---
name: grill-me
description: >
  Interview the user relentlessly about a plan or design, presenting choices one at a time
  until shared understanding is reached. Resolves every branch of the decision tree. Use
  when the user wants to stress-test a plan, get grilled on their design, or mentions
  "grill me".
---

# Grill Me

Stress-test plans and designs through relentless, structured questioning before implementation
begins.

## When to activate

Activate when:

- User says "grill me", "challenge my plan", "stress-test this", "interrogate my design",
  or any close variant
- A new plan is being created and design decisions remain open
- A design review is requested before committing to implementation

## Process

Interview the user about every aspect of the plan until shared understanding is reached. Walk
down each branch of the decision tree, resolving dependencies one-by-one.

**Rules (HARD — every question must follow all five):**

1. Ask questions **one at a time** — never bundle multiple questions in one message
2. Present **2–4 concrete options** with trade-off descriptions per question — **no open-ended
   questions allowed**; every question must offer discrete, actionable choices
3. **Mark the recommended option** clearly with `**(Recommended)**`
4. **Explore the codebase first** — if a question can be answered by reading existing files,
   read them instead of asking
5. Continue until all branches are resolved — do not stop early

**Violation of Rule 2 (asking without options) is the most common failure mode.** If you catch
yourself writing a question without listing concrete options, rewrite it with options before
sending.

## Question format

Structure **every** question exactly like this:

> **[Question]**
>
> - **Option A**: [description] — [trade-off]
> - **Option B**: [description] — [trade-off] **(Recommended)**
> - **Option C**: [description] — [trade-off]
>
> **Recommendation**: Option B because [specific reason grounded in this context].

No bare "What do you think about X?" questions. No yes/no questions without an options list.
Present the choices; let the user pick or override.

## After the grilling

When all decision tree branches are resolved:

1. Summarize every decision made and its rationale
2. Confirm shared understanding explicitly
3. Signal readiness to proceed to plan writing or implementation
