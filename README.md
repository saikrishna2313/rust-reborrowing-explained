# Understanding Reborrowing in Rust

Reborrowing is one of the most confusing — and most important — ideas in Rust’s
borrow checker.

At first, it feels like Rust is breaking its own rules:

> How can a mutable reference be reborrowed as immutable?  
> How can it even be reborrowed as mutable again?

This document explains **only reborrowing** — what it is, why it exists,
and the different cases Rust allows — using a simple mental model.

---

## What Is Reborrowing?

**Reborrowing** means creating a new reference **from an existing reference**,
instead of borrowing directly from the owner.

Nothing moves.  
Nothing is copied.  
Ownership does not change.

Only **access permissions** are temporarily redistributed.

Rust references are not just pointers — they also carry permissions.

---

## The Rule Rust Never Breaks

At any moment in time, Rust guarantees:

- **many readers**, or
- **exactly one writer**

Never both at the same time.

Reborrowing does **not** weaken this rule — it enforces it more precisely.

---

## Reborrowing a Mutable Reference as Immutable

A mutable reference (`&mut T`) has **read + write** permission.

Rust allows it to temporarily lend out a **read-only view** (`&T`).

Conceptually:

- the immutable reborrow becomes **active**
- the original mutable reference becomes **frozen**
- mutation is forbidden while the read-only reference exists
- after the last use of the reborrow, mutation resumes

The mutable reference does **not** become immutable — it is only
temporarily unusable.

---

## Reborrowing a Mutable Reference as Mutable

Rust also allows reborrowing a mutable reference **as mutable again**.

This works because:

- only **one mutable reference is active**
- the new mutable reference temporarily replaces the old one
- the original mutable reference is frozen
- once the reborrow ends, the original regains access

The “one writer” rule is never violated.

---

## Reborrowing from an Immutable Reference

An immutable reference (`&T`) has **read-only** permission.

From it, you may create:
- another immutable reference

You may **never** create:
- a mutable reference

Rust never allows gaining permissions you didn’t already have.

---

## Permission Flow (Mental Model)

Permissions flow **downward**, never upward:

&mut T (read + write)  
↓  
&T (read only)

You may **reduce** permissions.  
You may **never increase** them.

This single rule explains every valid and invalid reborrow.

---

## Frozen vs Immutable (Important Distinction)

A **frozen mutable reference** is *not* the same as an immutable reference.

A frozen `&mut T`:
- keeps its type
- keeps write permission
- is just temporarily disallowed from being used

An immutable reference (`&T`) can never write.

Freezing affects **when** a reference can be used — not **what** it is.

---

## Active Time, Not Scope

Rust uses **Non-Lexical Lifetimes (NLL)**.

This means:
- a reborrow lasts **until its last use**
- not until the end of a block

As soon as Rust can prove a reborrow is no longer used,
the original reference is automatically unfrozen.

---

## What Reborrowing Is Not

Reborrowing is **not**:
- ownership transfer
- pointer aliasing in the C/C++ sense
- related to stack vs heap memory
- a runtime mechanism

It is **compile-time permission management**.

---

## Why Reborrowing Exists

Without reborrowing:

- `&mut` values could not be passed to read-only APIs
- methods taking `&self` would not work on `&mut self`
- many common Rust patterns would be impossible

Reborrowing is what lets Rust remain **expressive**
without sacrificing **safety**.

---

## Final Mental Model

> Reborrowing temporarily redistributes access permissions
> while ownership stays fixed.

Or even simpler:

> **Pause mutation, allow access, then resume mutation.**

Once this clicks, the borrow checker stops feeling mysterious
and starts feeling mechanical.
