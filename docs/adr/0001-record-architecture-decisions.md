# ADR-0001: Record architecture decisions

- **Status:** Accepted
- **Date:** 2026-07-23
- **Deciders:** Project owner

## Context

`stevedore` moves secret material between stores — a domain where a quietly
wrong decision (how a value is read, where it's written, what's logged) has real
consequences. We want the reasoning behind such decisions to be legible later,
to us and to anyone the audience grows to include.

## Decision

We will record significant design and process decisions as
[Architecture Decision Records](https://adr.github.io/) under `docs/adr/`, one
Markdown file per decision, numbered sequentially with no gaps
(`NNNN-short-title.md`). Each starts from
[`0000-adr-template.md`](0000-adr-template.md) and is listed in
[`README.md`](README.md).

ADRs are **append-only decision history**: we don't delete or gut an accepted
ADR: we supersede it with a new one that references it and flip the old one's
status. Typo and link fixes in place are fine.

## Consequences

- The "why" behind a choice survives past the conversation that produced it.
- A small, standing documentation habit: a non-trivial decision isn't done until
  its ADR is written.
- Superseding rather than editing keeps the record honest but means the index
  grows monotonically.

## Alternatives considered

- **No formal record** — rely on commit messages and memory. Rejected: neither
  survives as browsable, first-class decision history.
- **A single design doc** — one file, edited in place. Rejected: it loses the
  chronology and the explicit supersession trail that make ADRs auditable.
