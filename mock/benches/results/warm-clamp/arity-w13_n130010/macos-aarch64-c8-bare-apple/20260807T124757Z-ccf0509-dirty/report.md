# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 69% (significant)

warm-clamp-minimum is -574 ns (69%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 17.0x slower than the field

warm-clamp-accfit-dyn (4.33 us) is 17.0x the fastest (255 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (423% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-min-lanes, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 423% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 17.0x the fastest

Fastest warm-clamp-minimum (255 ns) to slowest warm-clamp-accfit-dyn (4.33 us): 17.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-minimum** at 254.8 ns median (-69.2% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 16.99x (fastest 254.8 ns, slowest 4329.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 925ns | 888ns | 884ns | 891ns | 1068ns | base |
| warm-clamp-accfit | 341ns | 318ns | 313ns | 325ns | 414ns | -63.16% |
| warm-clamp-accfit-dyn | 4765ns | 4389ns | 4310ns | 4605ns | 5701ns | +415.07% |
| warm-clamp-head | 638ns | 632ns | 557ns | 631ns | 738ns | -31.08% |
| warm-clamp-min-lanes | 390ns | 371ns | 354ns | 375ns | 470ns | -57.86% |
| warm-clamp-minimum | 341ns | 316ns | 311ns | 326ns | 414ns | -63.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 861ns | 824ns | 991ns | base | 9.511 |
| warm-clamp-accfit | 275ns | 253ns | 333ns | -68.06% | 29.774 |
| warm-clamp-accfit-dyn | 4702ns | 4255ns | 5623ns | +445.87% | 1.742 |
| warm-clamp-head | 569ns | 498ns | 660ns | -33.95% | 14.399 |
| warm-clamp-min-lanes | 323ns | 294ns | 388ns | -62.50% | 25.364 |
| warm-clamp-minimum | 274ns | 252ns | 333ns | -68.14% | 29.852 |

## Performance model

- Peak throughput: **32.531 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.890 | 30.4% |
| warm-clamp-accfit | 31.814 | 97.8% |
| warm-clamp-accfit-dyn | 1.892 | 5.8% |
| warm-clamp-head | 14.516 | 44.6% |
| warm-clamp-min-lanes | 26.511 | 81.5% |
| warm-clamp-minimum | 32.151 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 925ns | 925ns | base |
| warm-clamp-accfit | 341ns | 341ns | -63.16% |
| warm-clamp-accfit-dyn | 4765ns | 4765ns | +415.07% |
| warm-clamp-head | 638ns | 638ns | -31.08% |
| warm-clamp-min-lanes | 390ns | 390ns | -57.86% |
| warm-clamp-minimum | 341ns | 341ns | -63.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 828ns | base | --- | [827, 835] | --- | --- | --- | --- |
| warm-clamp-accfit | 258ns | -571.5ns (-69.0%) | [-575, -568]ns | [256, 259] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4329ns | +3461.4ns (+417.9%) | [+3429, +3620]ns | [4287, 4478] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 564ns | -267.5ns (-32.3%) | [-321, -225]ns | [504, 603] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 309ns | -531.5ns (-64.2%) | [-532, -525]ns | [296, 311] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 255ns | -574.5ns (-69.4%) | [-576, -573]ns | [253, 259] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 821ns | -69.2% | +418.6% | -20.3% | -62.3% | -59.7% |
| 2 | 837ns | -69.8% | +408.8% | -22.0% | -62.7% | -60.3% |
| 3 | 829ns | -69.5% | +413.4% | -21.0% | -62.6% | -60.0% |
| 4 | 827ns | -69.5% | +414.7% | -21.0% | -63.2% | -59.9% |
| 5 | 827ns | -69.2% | +414.0% | -20.7% | -63.6% | -59.8% |
| 6 | 830ns | -69.4% | +413.2% | -21.0% | -64.5% | -59.9% |
| 7 | 827ns | -69.5% | +436.0% | -20.4% | -64.4% | -59.6% |
| 8 | 826ns | -69.2% | +416.3% | -20.8% | -64.3% | -59.7% |
| 9 | 826ns | -69.2% | +414.5% | -20.6% | -64.4% | -59.9% |
| 10 | 827ns | -69.7% | +414.4% | -20.7% | -64.3% | -59.8% |
| 11 | 828ns | -59.7% | +580.1% | -27.2% | -53.5% | -69.3% |
| 12 | 825ns | -59.6% | +578.9% | -27.0% | -53.7% | -69.5% |
| 13 | 826ns | -59.8% | +585.6% | -27.1% | -52.8% | -69.3% |
| 14 | 828ns | -59.8% | +576.7% | -27.2% | -52.8% | -69.4% |
| 15 | 828ns | -59.8% | +584.1% | -27.2% | -53.4% | -69.7% |
| 16 | 833ns | -60.0% | +572.5% | -27.3% | -53.8% | -69.6% |
| 17 | 839ns | -60.3% | +568.0% | -18.0% | -53.6% | -69.8% |
| 18 | 837ns | -60.2% | +569.9% | -28.1% | -53.5% | -69.7% |
| 19 | 827ns | -59.6% | +576.8% | -27.4% | -53.1% | -69.5% |
| 20 | 836ns | -60.3% | +570.0% | -27.7% | -53.8% | -69.8% |
| 21 | 825ns | -68.8% | +579.4% | -36.9% | -64.3% | -69.3% |
| 22 | 828ns | -69.1% | +560.2% | -36.2% | -64.3% | -69.5% |
| 23 | 826ns | -69.0% | +415.2% | -36.7% | -64.4% | -69.4% |
| 24 | 827ns | -68.9% | +414.5% | -37.6% | -64.5% | -69.5% |
| 25 | 826ns | -68.8% | +415.1% | -39.1% | -64.3% | -69.5% |
| 26 | 825ns | -69.2% | +420.3% | -38.7% | -64.4% | -69.4% |
| 27 | 826ns | -68.9% | +418.6% | -39.0% | -64.4% | -69.3% |
| 28 | 829ns | -68.9% | +423.4% | -39.2% | -64.3% | -69.5% |
| 29 | 822ns | -68.5% | +425.5% | -38.9% | -64.4% | -69.0% |
| 30 | 825ns | -68.6% | +428.3% | -39.0% | -64.4% | -69.5% |
| 31 | 1002ns | -74.2% | +325.4% | -50.3% | -69.0% | -74.6% |
| 32 | 1002ns | -74.7% | +328.5% | -50.1% | -69.1% | -74.4% |
| 33 | 1010ns | -74.5% | +327.0% | -50.6% | -69.3% | -74.5% |
| 34 | 1010ns | -74.4% | +326.9% | -50.8% | -69.2% | -74.5% |
| 35 | 1011ns | -74.5% | +327.9% | -50.7% | -69.3% | -74.6% |
| 36 | 1002ns | -74.3% | +332.4% | -50.4% | -69.3% | -73.3% |
| 37 | 1003ns | -74.4% | +341.7% | -50.3% | -69.0% | -74.0% |
| 38 | 888ns | -71.1% | +404.1% | -44.0% | -65.2% | -70.9% |
| 39 | 838ns | -69.5% | +435.5% | -40.3% | -63.5% | -68.8% |
| 40 | 842ns | -69.3% | +432.0% | -41.1% | -63.2% | -68.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.863 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.851 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.868 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.899 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.845 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.893 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.2ns | 861.3ns | 0.4% |  |
| warm-clamp-accfit | 2.4ns | 275.1ns | 0.9% |  |
| warm-clamp-accfit-dyn | 3.2ns | 4701.7ns | 0.1% |  |
| warm-clamp-head | 2.8ns | 568.9ns | 0.5% |  |
| warm-clamp-min-lanes | 2.5ns | 323.0ns | 0.8% |  |
| warm-clamp-minimum | 2.6ns | 274.4ns | 0.9% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 824.3-991.1 ns)
    824.3 |########################################
    832.7 |##########
    841.0 |#
    849.3 |
    857.7 |
    866.0 |
    874.4 |
    882.7 |#
    891.0 |
    899.4 |
    907.7 |
    916.1 |
    924.4 |
    932.7 |
    941.1 |
    949.4 |
    957.8 |
    966.1 |
    974.5 |
    982.8 |
  (2 below, 7 above range)

warm-clamp-accfit (n=40, range 252.8-333.2 ns)
    252.8 |#####################################
    256.9 |########################################
    260.9 |
    264.9 |
    268.9 |
    272.9 |
    277.0 |
    281.0 |
    285.0 |
    289.0 |
    293.0 |
    297.1 |
    301.1 |
    305.1 |
    309.1 |
    313.1 |
    317.1 |
    321.2 |
    325.2 |
    329.2 |#################
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 4254.9-5623.4 ns)
   4254.9 |########################################
   4323.3 |#########
   4391.7 |####
   4460.2 |#######
   4528.6 |
   4597.0 |
   4665.4 |
   4733.9 |
   4802.3 |
   4870.7 |
   4939.1 |
   5007.6 |
   5076.0 |
   5144.4 |
   5212.8 |
   5281.3 |
   5349.7 |
   5418.1 |##
   5486.5 |
   5555.0 |##################
  (2 below, 3 above range)

warm-clamp-head (n=40, range 497.7-659.8 ns)
    497.7 |########################################
    505.8 |
    513.9 |######
    522.0 |######
    530.1 |
    538.2 |
    546.3 |
    554.4 |
    562.5 |
    570.6 |
    578.7 |
    586.8 |
    595.0 |####################
    603.1 |##########
    611.2 |
    619.3 |
    627.4 |
    635.5 |
    643.6 |
    651.7 |#################################
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 293.7-388.1 ns)
    293.7 |########################################
    298.4 |###
    303.1 |######
    307.8 |########################################
    312.6 |
    317.3 |
    322.0 |
    326.7 |
    331.4 |
    336.2 |
    340.9 |
    345.6 |
    350.3 |
    355.1 |
    359.8 |
    364.5 |
    369.2 |
    374.0 |
    378.7 |###
    383.4 |################
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 251.8-332.7 ns)
    251.8 |########################################
    255.9 |###########
    259.9 |####
    264.0 |####
    268.0 |
    272.0 |
    276.1 |
    280.1 |
    284.2 |
    288.2 |
    292.2 |
    296.3 |
    300.3 |
    304.4 |
    308.4 |
    312.5 |
    316.5 |
    320.5 |
    324.6 |
    328.6 |###########
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.89 (measurement drift or warm-up artifact)
