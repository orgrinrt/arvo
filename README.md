# `arvo`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/arvo.svg)](https://github.com/orgrinrt/arvo/issues)
![License](https://img.shields.io/github/license/orgrinrt/arvo?color=%23009689)

> Numeric foundations where the consumer picks the behaviour instead of the container the machine happens to offer. `no_std`, no alloc, no platform deps.

</div>

`arvo` is about numbers whose behaviour you choose rather than inherit. Ordinary integer types hand you whatever the hardware has lying around, a `u16` when you wanted thirteen bits, and then the wrapping, the rounding and the width are all decided for you by that accident. Here the width is exactly the width you name, and what happens at the edges is a thing you select at the call site, so two values of the same nominal shape can behave differently because their consumers wanted different things from them.

Everything is `#![no_std]` with no alloc and no platform dependency, and every size is const at the type level, so the choosing happens at compile time and the branch that picks it is gone by the time anything runs.

## Where this is right now

There's no shipping code in the tree. The design is being written from the ground up at the moment, and the crate layout is deliberately not there while that happens, so if you clone this expecting to build something, that's why it's empty.

I'd caution against depending on this for anything yet. Not just the api, the shape itself is still moving: how many crates there are, what they're called, and which layer owns what are all open questions right now, and answering them is the work in progress.

## What has settled

A format is identified by two things, the domain it lives in and the set of values it can actually represent, and that set is a constant of the type rather than something that depends on the data. Membership in it turns out to be one predicate over one parameterisation, with a slot function, a quantum per magnitude and a phase, and integers, fixed point, scaled integers and floats all fall out as points of that same shape rather than as separate families needing separate treatment.

Arithmetic then factors into two halves that are worth keeping apart: the exact operation in the ambient domain, and a named adaptation that puts the result back onto the representable set. The adaptation is a real object with its own laws, not a footnote about rounding, and that's what makes it possible to say which laws survive a given composition and which don't.

Rounding carries six explicit names, `toward_zero`, `floor`, `ceil`, `half_up`, `half_even` and `stochastic`. The familiar `trunc` is deliberately not among them, because on a signed domain it names two different operations and people reliably mean the wrong one of the two.

The concept is closed and the inventory isn't. A new number system earns its place by supplying what the concept asks of it, so admitting one doesn't mean amending anything.

## What it's for

`arvo` is a library and the value is in what composes on top of it, so it stays usable by anyone rather than being shaped around one caller. The known consumers are [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin), a pipeline execution engine, and [`vehje`](https://github.com/orgrinrt/vehje), a language toolchain, and they're what drive which surfaces eventually exist. It builds on [`notko`](https://github.com/orgrinrt/notko) for its foundations.

If you want general-purpose numerics today, this probably isn't the right thing, and there are established crates in the ecosystem that will serve you better and are available right now. The reason to want this one is the exact widths and the behaviour being a choice you make, which is a fairly specific thing to want.

## A note on coding agents

We do not recommend using coding agents with this codebase.

The design here is unusual enough that current models tend to reach for the familiar shape instead of the one that's written down, and the tree being empty makes that worse, because there's nothing in front of them to correct the guess.

If you still choose to use a coding agent:

- Be aware of the environmental and social impact of large-scale model inference. Minimise agent use where it is not needed. Be responsible.
- Only use an agent if you yourself understand the architecture. Do not use an agent because you do not understand; you will waste time and energy, both yours and the planet's.
- This repository provides agent instructions and skills that help, but they do not eliminate the problem. You will still need to correct the agent frequently.

The recommendation stands: do this work yourself unless you know what you are doing and why.

## Contributing

Feel free to contribute, though do mind that the design is still moving under everything right now, so a large pull request has a real chance of landing against something that changed last week. Throwing in an issue first is the cheaper path if you're unsure. Forks are always a valid choice too and I'd encourage anyone to have their own take on this, just mind the license when you do.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/main/LICENSE)
