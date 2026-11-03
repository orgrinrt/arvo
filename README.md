# `arvo`

> Numeric foundations where the consumer picks the behaviour, `no_std`, no alloc and no platform dependencies.

`arvo` is a set of crates for numbers whose width and edge behaviour you choose, instead of inheriting them from whatever integer type the hardware happens to have. A value of thirteen bits is thirteen bits, and what happens at its edges, rounding, overflow, saturation, is picked at the call site, so two values of the same nominal shape can act differently because their users wanted different things. Every size is a constant at the type level, which means the choosing is done at compile time and nothing of it is left to run.

## Status

The bottom of the stack is in the tree and builds. `arvo-bits` is an exact-width bit container, `arvo-format` declares a number's representable set and the adaptation that puts an exact result back onto it, with six rounding modes (`toward_zero`, `floor`, `ceil`, `half_up`, `half_even` and `stochastic`) and the overflow policies, `arvo-placement` works out where a result's bits sit, and `arvo-strategy` binds the presets `Hot`, `Cold`, `Warm` and `Precise` to a placement and an adaptation. MATLAB `fi` and `fimath` declarations can be written as themselves and resolve to a format the crates already have.

There is no `UFixed` or `Uint` to reach for yet, the layers above these are still being designed, and the api will move. A git dependency on the `dev` branch gets the current state sooner than a release will. If you want general-purpose numerics today there are established crates that will serve you better, and this one is for the case where exact widths and a chosen behaviour are the point.

## Used by

[`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin), a pipeline execution engine, and [`vehje`](https://github.com/orgrinrt/vehje), a language toolchain, are the two consumers, and it builds on [`notko`](https://github.com/orgrinrt/notko).

## Coding agents

We don't recommend using coding agents on this codebase. The design is unusual enough that current models reach for the familiar shape instead of the one that is written down, and there is not enough code yet to correct the guess for them. If you use one anyway, understand the architecture first, keep the use to what is needed, and expect to correct it often.

## Contributing

The design is still moving, so a large pull request can land against something that changed last week, and opening an issue first is cheaper. Forks are welcome, mind the licence.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/arvo/blob/main/LICENSE)
