# p6, first attempt: rung selection by where-clause is refused

The first version of `p6` selected the native rung with a `where M: InRange<LO, HI>` bound on
otherwise-identical impls. rustc refuses it, and the refusal is a real result about rung
selection rather than a mistake in the probe: a where-clause does not disambiguate impls, so
two rungs of the same ladder overlap at the head constructor.

Reproduced by restoring the `rung!` macro form and compiling. Diagnostic:

```
error[E0119]: conflicting implementations of trait `Derive<Warm>` for type `(_, Warm)`
   |
58 | /         impl<M: Mag> Derive<Warm> for (M, Warm)
59 | |         where
60 | |             M: InRange<$lo, $hi>,
   | |                                 ^ first implementation here
   |                                     conflicting implementation for `(_, Warm)`
...
92 |   rung!(9..=16 => u16);
```

What it establishes: the rung has to be carried IN THE TYPE, not in a bound. That is consistent
with the closed panel's structural-keying result (`seed/SETTLED_container.md:74-89`), reached
here independently and from the opposite direction, by trying the bound form and being refused.

It does not establish anything about the arity question, which is what `p6` went on to check.
