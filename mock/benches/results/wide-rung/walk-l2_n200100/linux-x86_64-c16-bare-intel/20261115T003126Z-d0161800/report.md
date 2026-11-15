# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (715.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 553.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread beats baseline by 23% (significant)

wide-rung-ragged-overread is -167.95 us (23%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-wordround-alias shows alternating (throttle bounce) (autocorr -0.79)

wide-rung-wordround-alias's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 553845.7 ns median (-22.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.29x (fastest 553845.7 ns, slowest 715572.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 709914ns | 716009ns | 651960ns | 711178ns | 764079ns | base |
| wide-rung-ragged | 597048ns | 587943ns | 563010ns | 594555ns | 638567ns | -15.90% |
| wide-rung-ragged-overread | 551059ns | 554364ns | 510928ns | 553061ns | 585184ns | -22.38% |
| wide-rung-wordround | 688546ns | 688147ns | 649528ns | 686108ns | 734875ns | -3.01% |
| wide-rung-wordround-alias | 701694ns | 706410ns | 634514ns | 704554ns | 760295ns | -1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 709353ns | 651334ns | 763531ns | base | 0.647 |
| wide-rung-ragged | 596611ns | 562639ns | 638096ns | -15.89% | 0.769 |
| wide-rung-ragged-overread | 550586ns | 510483ns | 584656ns | -22.38% | 0.833 |
| wide-rung-wordround | 687936ns | 649047ns | 734210ns | -3.02% | 0.667 |
| wide-rung-wordround-alias | 701050ns | 633786ns | 759678ns | -1.17% | 0.654 |

## Performance model

- Peak throughput: **0.899 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.641 | 71.3% |
| wide-rung-ragged | 0.781 | 86.9% |
| wide-rung-ragged-overread | 0.828 | 92.2% |
| wide-rung-wordround | 0.667 | 74.2% |
| wide-rung-wordround-alias | 0.650 | 72.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 709914ns | 709914ns | base |
| wide-rung-ragged | 597048ns | 597048ns | -15.90% |
| wide-rung-ragged-overread | 551059ns | 551059ns | -22.38% |
| wide-rung-wordround | 688546ns | 688546ns | -3.01% |
| wide-rung-wordround-alias | 701694ns | 701694ns | -1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 715572ns | base | --- | [689320, 732527] | --- | --- | --- | --- |
| wide-rung-ragged | 587398ns | -116961.6ns (-16.3%) | [-150330, -102951]ns | [581539, 608552] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 553846ns | -167949.0ns (-23.5%) | [-177619, -138945]ns | [546607, 564034] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 687527ns | -27538.0ns (-3.8%) | [-46643, -9782]ns | [687044, 689528] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 705769ns | no significant difference | [-6864, +7058]ns | [666645, 741644] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 689549ns | -15.7% | -19.8% | -2.7% | +7.5% |
| 1 | 2 | 716287ns | -21.2% | -23.0% | -4.0% | -8.9% |
| 1 | 3 | 746688ns | -21.4% | -24.2% | -7.6% | -0.4% |
| 1 | 4 | 672042ns | -7.6% | -23.6% | +2.2% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.358 | moderate- |
| wide-rung-ragged | 0.089 | ok |
| wide-rung-ragged-overread | -0.329 | moderate- |
| wide-rung-wordround | -0.017 | ok |
| wide-rung-wordround-alias | -0.787 | HIGH- (thermal bounce) |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 154.5ns | 709353.4ns | 0.0% |  |
| wide-rung-ragged | 119.5ns | 596611.3ns | 0.0% |  |
| wide-rung-ragged-overread | 169.3ns | 550586.5ns | 0.0% |  |
| wide-rung-wordround | 118.6ns | 687936.0ns | 0.0% |  |
| wide-rung-wordround-alias | 140.8ns | 701049.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 651334.0-763531.1 ns)
  651334.0 |####
  656943.8 |####
  662553.7 |
  668163.6 |########
  673773.4 |####
  679383.3 |
  684993.1 |########################################
  690603.0 |
  696212.8 |
  701822.7 |
  707432.5 |
  713042.4 |####################
  718652.3 |########
  724262.1 |
  729872.0 |
  735481.8 |
  741091.7 |####################
  746701.5 |########################
  752311.4 |
  757921.2 |
  (4 below, 3 above range)

wide-rung-ragged (n=40, range 562639.0-638096.3 ns)
  562639.0 |##########################
  566411.8 |####
  570184.7 |
  573957.6 |########
  577730.4 |#################
  581503.3 |#############
  585276.2 |#################
  589049.0 |########
  592821.9 |####
  596594.8 |########
  600367.7 |
  604140.5 |
  607913.4 |
  611686.3 |
  615459.1 |####
  619232.0 |########################################
  623004.9 |
  626777.7 |
  630550.6 |
  634323.5 |
  (1 below, 4 above range)

wide-rung-ragged-overread (n=40, range 510483.3-584656.1 ns)
  510483.3 |###########
  514191.9 |#################
  517900.6 |
  521609.2 |
  525317.9 |
  529026.5 |
  532735.1 |#####
  536443.8 |#####
  540152.4 |###########
  543861.0 |#####
  547569.7 |###########
  551278.3 |########################################
  554987.0 |#####
  558695.6 |#####
  562404.2 |############################
  566112.9 |############################
  569821.5 |
  573530.1 |
  577238.8 |###########
  580947.4 |
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 649047.0-734210.0 ns)
  649047.0 |#####
  653305.2 |########
  657563.3 |##
  661821.5 |##
  666079.6 |#####
  670337.8 |##
  674595.9 |##
  678854.1 |
  683112.2 |#################
  687370.4 |########################################
  691628.5 |#####
  695886.7 |
  700144.8 |
  704403.0 |
  708661.1 |##
  712919.3 |
  717177.4 |
  721435.6 |##
  725693.7 |
  729951.9 |
  (1 below, 4 above range)

wide-rung-wordround-alias (n=40, range 633785.7-759678.0 ns)
  633785.7 |
  640080.4 |##
  646375.0 |##
  652669.6 |##########
  658964.2 |##
  665258.8 |#########################
  671553.4 |
  677848.0 |
  684142.7 |
  690437.3 |
  696731.9 |
  703026.5 |
  709321.1 |
  715615.7 |
  721910.3 |
  728205.0 |
  734499.6 |#####
  740794.2 |########################################
  747088.8 |##
  753383.4 |
  (3 below, 1 above range)

```
