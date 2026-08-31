# 181. Op: the canon becomes a registry, the contracts hold, and this runs to a finished canon

**This file is out of order and the number is the only thing wrong with it.** Op said all of it at the
opening of the session that produced `179` and `180`, so it precedes both. Those two numbers were
already minted and handed to seats by the time this was written down, and a minted number is not
reused. Read this before either of them.

Verbatim, in full, as he sent it. The archivist's capture of the same words is in the workspace
repository at `.data/op-responses/20260830_2320_arvo-panel-steering-opening.md`, which a reader here
cannot open; this file is the citable copy.

---

> Alright, get your bearings. Your work is to continue the ingoing arvo panel and drive it hard to
> convergence. We want a finished canon by the end of this, and you need to really whip the experts to
> actually agree and find the solutions now.
>
> A few notes:
>
> - Mockspace has, while this was on pause, gained a first-class registry concept that is the new
>   convention for a canon to form. The arvo canon will have to be converted there. See
>   ~/Dev/clause-work/kamu-canon/kamu for a live example of how one is built and how the panels end up
>   with all the canon stuff
> - Mockspace has, also, gained a first-class tools concept in addition to lints. See the same repo for
>   how they use it, and perhaps you might want to fork from theirs in fact. Or extract to a shared
>   crate, which you both can then depend on in the mockspace.toml. Your call either way.
> - Homma is the new workspace harness, and you will work within this one, which is yours. It's yours,
>   and you only work within it. Keep it clean, tidy, don't collect stale worktrees please, get things
>   into branches on remotes and merged too preferably ultimately, and prune out the worktrees after
>   they are not needed. There are plenty of rules on how to work in this new way, so follow them.
> - Homma comes with some useful tools and commands of itself, and the homma side tool concept is a bit
>   of a wip, but workable. Should see if any of it is useful for you
> - Preceding the tool concept on homma side, there's also .shared/scripts which may contain useful
>   tools, or maybe not. Worth knowing about though.
> - The stack you work in extends beyond these initial three (notko, arvo, hilavituktin) which you may
>   and probably want to loan from. See my github, as well as hiisi-digital org's github, for any mostly
>   rust based projects, those are most of them likely useful or reusable, for something at some point.
> - The trait contract based structure is a primary paradigm we uphold in future too. This allows for
>   the shapes to stick even when the impls are wip, like hilavitkutin currently has
> - Do not hurry or fuck the canon and panel work up by rushing it too much. You want to steer
>   aggressively towards convergence and real finished canon, even if not exactly perfect, but don't be
>   lazy or naive about it. Good quality and real work, sophisticated, ergonomic solutions, efficient and
>   optimal. But it's been literal months ongoing by now, and there seems to be no end to how much the
>   theoretical side can be argued on all directions and we just want to settle somewhere within the
>   constraints we already kind of have formed.
>
> Godspeed. Also, did you remember to call the little archivist on my calls or responses? Have you been
> following the rules? All of them? Did you remember to pull and push the homma workspace content repo
> (clause-dev here)? And are you really achieving results without being naive and dumb about it, or are
> you treading water with the experts because they naturally seem unable to settle and would argue
> eternally if nobody pushes them towards convergence and agreement? And have you been careful,
> remembering to write down all the manual checks and tests as actual tests in the source? Even the
> negative ones that prove something that shouldn't compile, won't compile? Controls for both? Man, you
> need to be extra careful about all this.
>
> You will work autonomously here, because this work is, for the most part, already steered and
> commented on by me on length. Now the first thing is to port all the current results and agreements
> and convergences and settled things to the registry, shape the registry, its meta, so it works, then go
> ahead and comb all the rulings and proposals and statements etc. See kamu for an example of how this
> can work when done well.
>
> We want to get arvo canon settled in full. After that, I want to review it. Before sending it out to
> me, please make sure that the canon is exhaustive enough that a full design and then a full impl of
> everything can be done based on it, as explained on the various rules about how this concept works.
> Collect things to ask from me to a batch, and work autonomously until I respond to you or ask you to
> ask the questions. Before I show I'm present and ready to answer, keep them catalogued in full, don't
> lose track of them, and work around them as much as you can.

---

## Naming the intent, which is a separate act from quoting him

Three of these are design statements and belong in the registry as rulings. The rest are process, and
two of them are questions about the coordinator's conduct rather than instructions.

**The trait contract based structure is a primary paradigm arvo upholds.** His only new design
statement, and it is a general one rather than an answer to anything the panel asked. His reason is
stated with it: **it lets the shapes stick while the implementations are still work in progress**, and
he names hilavitkutin as the live example of that condition. It bears directly on what a canon has to
carry, since a contract is a shape and a shape is expressible before anything implements it.

**The canon has to be exhaustive enough that a full design and then a full impl can be done from it.**
A bar on the finished thing rather than a design statement, and the sharpest thing in the message,
because it is checkable: it is what the `obligation` namespace exists to answer.

**A finished canon is the end of this arc, and he reviews it after.** Which fixes what "done" means.

**Process, binding but not design.** The canon converts to a mockspace registry, which is now the
convention. The tools question is his to hand over and mine to answer, either way. The workspace is
homma and this session works only inside its own. Worktrees are pruned rather than collected. The stack
to borrow from is wider than the three repositories.

**And one instruction about pace that pulls both ways on purpose.** Steer aggressively to convergence
and a real finished canon, even if not exactly perfect; and do not be lazy or naive about it, and do not
rush it into a mess. He is explicit that the theoretical side has no natural end and that settling
inside the constraints already formed is the goal. **That is a licence to close things, not a licence to
skip them.**

## What is not an intent here, and must not be recorded as one

His four questions in the "Godspeed" paragraph are checks on whether the standing rules have been
followed. They are not new rules and they create no new obligation on arvo's design. They are recorded
here because they are his words, and they answer to the workspace's own discipline rather than to the
canon.
