# 83. Op: the distinction collapses to whatever is available at const time

Op, in this panel, on 2026-08-13, answering checkpoint `81`'s Q-C. His words, verbatim and complete:

> Let me just add there that the above collapses to whatever is available at const time: Making the
> predicates const expressions for example, allows using const functions and pipe in some data that is
> outside the typestate. However, being const time expressions, typestate is usable there too

## What he was choosing among

The checkpoint put Q-C to him as a two-way fork, and named the cost of each side:

1. **A value may gate an arm.** Then `79`'s P4 is an arm that runs where the law holds and falls back
   where it does not, at the price `80` measured in section 5.1: the value-gated form materialises both
   lowerings and selects with a `csel`, 13 instructions against 6 and 3 for the two static arms, so it is
   worse than either rather than worse than the better one.
2. **A value may not gate an arm.** Then P4 is a characterisation and not an arm, and so is `42`'s
   reachability condition in its value-level form, and every other trajectory region this panel has
   measured, unless somebody lifts its conditions into declarations. `80` reported nobody had tried to
   construct such a lifting.

**He took neither, and rejected the fork.** That is the content of the answer and it is why this file
exists rather than a line in the register.

## What follows, stated only as far as his words reach

The panel's working distinction was **typestate predicate against trajectory predicate**: a function of
the type against a function of the values flowing through. `80` built that distinction and this unit has
been reasoning inside it since.

Op's sentence says the axis is not type against value. **It is const-available against not.** A predicate
is a const expression, so it may call const functions, and it may take in data from outside the typestate,
provided that data is available at const time. And because the expression is evaluated at const time, the
typestate remains usable inside it: the typestate is one source of const-available data rather than the
only admissible source.

So "typestate predicate" was too narrow a name for the licensed category, and "trajectory predicate" was
carrying two different things under one name: conditions over data that happens to be const-available at a
given call site, and conditions over data that genuinely is not available until the program runs. The first
is admissible; his sentence does not reach the second.

**What his words do not settle**, and none of it should be read into them:

- Whether a condition over data that is genuinely not const-available may gate an arm at all. He said what
  a const predicate may reach; he did not say what happens beyond it.
- Which of the panel's measured trajectory conditions have a const-available form. That is a construction
  question and it is what `82` is in flight on.
- Anything about the binding-time axis of Q-A. A const predicate is evaluated at const time by
  construction, but Q-A asks about the verb "validate" across a three by two grid, and this sentence
  addresses the arm-selection question rather than the validation one.

## Rung

**This is op's own statement about the meaning of his own ratified entry**, made in this panel, in his
voice. It is recorded at that weight and no higher. He did not mark it as a separate ratification and it
is not entered as one. I13 is unchanged; what this file records is what he says its phrase "const
predicates" reaches.

Required reading for every file in this unit after it. `82` was dispatched before he said it and was told
of it mid-flight, by a message pointing here rather than paraphrasing.
