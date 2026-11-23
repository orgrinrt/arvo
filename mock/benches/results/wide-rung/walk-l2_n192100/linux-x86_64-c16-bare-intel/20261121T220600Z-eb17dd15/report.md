# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (543.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround-alias at 358.79 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-wordround-alias beats baseline by 25% (significant)

wide-rung-wordround-alias is -134.33 us (25%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged, wide-rung-ragged-overread} vs {wide-rung-align16} (31% apart)

The field splits into a fast tier {wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged, wide-rung-ragged-overread} and a slow tier {wide-rung-align16} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged-overread is inconsistent: worst-20% is 1.7x its best-20%

wide-rung-ragged-overread's best 20% of batches run at 319.02 us but its worst 20% at 532.06 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 358787.5 ns median (-34.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.51x (fastest 358787.5 ns, slowest 543245.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112737653 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 525230ns | 543745ns | 449989ns | 531321ns | 582201ns | base |
| wide-rung-ragged | 409575ns | 383634ns | 343171ns | 397513ns | 512164ns | -22.02% |
| wide-rung-ragged-overread | 427290ns | 414617ns | 319479ns | 428180ns | 532432ns | -18.65% |
| wide-rung-wordround | 404140ns | 377649ns | 326757ns | 402323ns | 486973ns | -23.05% |
| wide-rung-wordround-alias | 378220ns | 359252ns | 341144ns | 359883ns | 470309ns | -27.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 524764ns | 449506ns | 581763ns | base | 0.874 |
| wide-rung-ragged | 409132ns | 342870ns | 511598ns | -22.04% | 1.121 |
| wide-rung-ragged-overread | 426919ns | 319020ns | 532055ns | -18.65% | 1.075 |
| wide-rung-wordround | 403810ns | 326453ns | 486560ns | -23.05% | 1.136 |
| wide-rung-wordround-alias | 377822ns | 340858ns | 469752ns | -28.00% | 1.214 |

## Performance model

- Peak throughput: **1.438 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.844 | 58.7% |
| wide-rung-ragged | 1.197 | 83.2% |
| wide-rung-ragged-overread | 1.107 | 77.0% |
| wide-rung-wordround | 1.216 | 84.5% |
| wide-rung-wordround-alias | 1.279 | 88.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 525230ns | 525230ns | base |
| wide-rung-ragged | 409575ns | 409575ns | -22.02% |
| wide-rung-ragged-overread | 427290ns | 427290ns | -18.65% |
| wide-rung-wordround | 404140ns | 404140ns | -23.05% |
| wide-rung-wordround-alias | 378220ns | 378220ns | -27.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 543245ns | base | --- | [505819, 561385] | --- | --- | --- | --- |
| wide-rung-ragged | 383209ns | -91942.7ns (-16.9%) | [-183900, -69613]ns | [370193, 436315] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 414294ns | -122501.7ns (-22.5%) | [-163745, -101052]ns | [405589, 455669] | YES | 0.0022 | 0.0022 | 0 |
| wide-rung-wordround | 377365ns | -110505.9ns (-20.3%) | [-160817, -93360]ns | [370822, 444553] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 358788ns | -134328.0ns (-24.7%) | [-201986, -104697]ns | [353890, 365053] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 558640ns | -38.6% | -18.6% | -33.9% | -39.2% |
| 1 | 2 | 577800ns | -36.0% | -29.6% | -44.4% | -37.8% |
| 1 | 3 | 452742ns | -15.3% | +6.1% | -0.1% | -21.9% |
| 1 | 4 | 522805ns | -16.2% | -39.3% | -28.2% | -30.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.201 | moderate- |
| wide-rung-ragged | 0.114 | ok |
| wide-rung-ragged-overread | -0.467 | moderate- |
| wide-rung-wordround | -0.447 | moderate- |
| wide-rung-wordround-alias | -0.225 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 76.5ns | 524764.0ns | 0.0% |  |
| wide-rung-ragged | 124.1ns | 409132.1ns | 0.0% |  |
| wide-rung-ragged-overread | 99.5ns | 426918.5ns | 0.0% |  |
| wide-rung-wordround | 139.4ns | 403809.5ns | 0.0% |  |
| wide-rung-wordround-alias | 86.2ns | 377822.2ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 449506.0-581763.4 ns)
  449506.0 |###################################
  456118.9 |
  462731.7 |#####
  469344.6 |##########
  475957.5 |
  482570.3 |
  489183.2 |#####
  495796.1 |
  502409.0 |
  509021.8 |
  515634.7 |#####
  522247.6 |##########
  528860.4 |###############
  535473.3 |
  542086.2 |
  548699.0 |#####
  555311.9 |#########################
  561924.8 |###############
  568537.6 |#####
  575150.5 |########################################
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 342869.9-511598.0 ns)
  342869.9 |####################
  351306.3 |
  359742.7 |
  368179.1 |########################################
  376615.5 |###################################
  385051.9 |
  393488.3 |#####
  401924.7 |
  410361.1 |
  418797.5 |
  427234.0 |##########
  435670.4 |########################################
  444106.8 |#####
  452543.2 |
  460979.6 |
  469416.0 |
  477852.4 |
  486288.8 |#####
  494725.2 |
  503161.6 |#####
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 319020.1-532055.4 ns)
  319020.1 |#####
  329671.9 |#####
  340323.7 |
  350975.4 |
  361627.2 |#################
  372279.0 |
  382930.7 |#####
  393582.5 |#####
  404234.3 |########################################
  414886.0 |#####
  425537.8 |#####
  436189.6 |#####
  446841.3 |######################
  457493.1 |
  468144.9 |
  478796.6 |########################################
  489448.4 |#####
  500100.1 |
  510751.9 |
  521403.7 |###########
  (6 below, 3 above range)

wide-rung-wordround (n=40, range 326452.7-486560.3 ns)
  326452.7 |
  334458.1 |###
  342463.5 |
  350468.9 |
  358474.3 |
  366479.6 |########################################
  374485.0 |##########
  382490.4 |
  390495.8 |###
  398501.1 |
  406506.5 |
  414511.9 |###
  422517.3 |#######
  430522.7 |###
  438528.0 |
  446533.4 |#########################
  454538.8 |###
  462544.2 |##############
  470549.6 |###
  478554.9 |
  (6 below, 1 above range)

wide-rung-wordround-alias (n=40, range 340858.1-469752.1 ns)
  340858.1 |###
  347302.8 |#####################
  353747.5 |########################################
  360192.2 |#####################
  366636.9 |##########
  373081.6 |
  379526.3 |
  385971.0 |
  392415.7 |
  398860.4 |
  405305.1 |
  411749.8 |
  418194.5 |
  424639.2 |###
  431083.9 |
  437528.6 |###
  443973.3 |
  450418.0 |
  456862.7 |###
  463307.4 |###
  (6 below, 3 above range)

```
