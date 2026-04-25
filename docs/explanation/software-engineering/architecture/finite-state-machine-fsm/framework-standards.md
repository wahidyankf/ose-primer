---
title: "FSM Framework Standards"
description: Spring State Machine, XState configuration standards for demo
category: explanation
subcategory: architecture
tags:
  - fsm
  - spring-state-machine
  - xstate
principles:
  - automation-over-manual
---

# FSM Framework Standards

## Prerequisite Knowledge

## Spring State Machine (Java)

**REQUIRED for Java applications**.

**Configuration**:

- MUST use `@Configuration` with `@EnableStateMachine`
- MUST use configuration DSL, NOT XML
- MUST persist state for long-running workflows

**Example**:

```java
@Configuration
@EnableStateMachine
public class ZakatStateMachineConfig extends StateMachineConfigurerAdapter<
    ZakatState, ZakatEvent> {
    // Configuration here
}
```

## XState (TypeScript)

**REQUIRED for TypeScript applications**.

**Configuration**:

- MUST use `createMachine` with TypeScript types
- MUST visualize with XState Viz
- MUST define guards as pure functions

**Example**:

```typescript
const machine = createMachine({
  id: "campaign",
  initial: "planning",
  states: {
    /* ... */
  },
});
```

## Go

**OPTIONAL**: Use `looplab/fsm` OR hand-rolled.

**MUST**: Use explicit state type (not strings).

---
