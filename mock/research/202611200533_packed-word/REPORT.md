# A Packed Word of Named Fields, Asked by Hilavitkutin

This is the record of the job `arvo-packed-word`, which put row A of hilavitkutin's port plan through arvo's flow: the need as an `obligation` row, then the word in `arvo-bits` through design round `202611200533-wwxt`. The plan is `mock/research/202611200345_arvo-port-plan/README.md` on hilavitkutin `dev`, section "The rows arvo still owes, in hilavitkutin's terms", and its job 2, identity and strings, waits on this row.

The sections below are written as the job reaches them. The demand side is first, because the obligation row cites it.

## The demand hilavitkutin states

Read at hilavitkutin `dev` `ad6da29e`.

Hilavitkutin's identity handle, `Sym`, is one 32-bit word. Its default shape lays out a 28-bit `id` at bits 0 to 27, a 3-bit `kind` at 28 to 30 and a 1-bit `flag` at 31 (`mock/crates/hilavitkutin-sym/DESIGN.md.tmpl`, the table under "How the 32 bits divide is a shape"). A second shape, the wide-tag one, divides the same word as 26, 5 and 1 (`hilavitkutin-sym/src/handle.rs`, the second layout declaration, at line 73). The sites written today with the removed tree's field macro are `hilavitkutin-sym/src/handle.rs:19` and `:73`; the field width projection that reads a field's type back is `hilavitkutin-sym/src/shape.rs:185`.

What the consumer asks of the word, each in its own document:

The fields are read and replaced by name: "`SymLayout` generates per-field accessors (`kind()`, `id()`, `flag()`), setters, and per-field mask constants."

The layout is a wire fact fixed when the consumer builds: "A shape is a wire format. Two `Sym`s built under different shapes divide the same 32 bits differently, so they are not comparable and must not meet in one store." (the same design, emphasis dropped).

A field one bit too narrow, or two fields overlapping, is the failure its tests exist to catch: "The maximum is the case that finds a field one bit too narrow", and "Writing one field disturbs no other. This is the bitfield failure every law here was previously blind to, because a law that reads one field at a time cannot see an overlap between two."

The handle is compared and persisted as its raw bits: "`to_bits()` (the raw `Bits<32, Hot>` view for tests and persistence)", and "Handle equality is raw-bit equality".

The flag is read as a bit, not a number: `fn get_flag(self) -> Bit<Hot>` in `handle.rs`, beside `type Id = Bits<28, Hot>` and `type Kind = Bits<3, Hot>`.

So the need, stated without a mechanism: a word of a declared total width divided into named fields of declared widths, each field read and written in a constant, a one-bit field with its own type, the widths summing to the word checked at build, and the whole convertible to and from the exact-width container at no cost. The nearest row, `obligation::an_exact_width_container_a_consumer_can_alias_and_pin`, asks for the container only.
