# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 63% (significant)

warm-clamp-accfit is -330 ns (63%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 51.5x slower than the field

warm-clamp-minimum (9.79 us) is 51.5x the fastest (190 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows warm-up / thermal drift (autocorr +0.77)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation +0.77, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes, warm-clamp-acc64} vs {warm-clamp-minimum} (1782% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes, warm-clamp-acc64} and a slow tier {warm-clamp-minimum} with a 1782% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 51.5x the fastest

Fastest warm-clamp-accfit (190 ns) to slowest warm-clamp-minimum (9.79 us): 51.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 190.0 ns median (-63.5% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 51.52x (fastest 190.0 ns, slowest 9789.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 587ns | 582ns | 578ns | 586ns | 603ns | base |
| warm-clamp-accfit | 255ns | 253ns | 248ns | 254ns | 264ns | -56.64% |
| warm-clamp-accfit-dyn | 259ns | 257ns | 251ns | 258ns | 269ns | -55.98% |
| warm-clamp-head | 284ns | 283ns | 277ns | 283ns | 291ns | -51.72% |
| warm-clamp-min-lanes | 531ns | 528ns | 516ns | 530ns | 552ns | -9.55% |
| warm-clamp-minimum | 10104ns | 9854ns | 9713ns | 9924ns | 11034ns | +1620.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 525ns | 517ns | 538ns | base | 15.606 |
| warm-clamp-accfit | 191ns | 186ns | 199ns | -63.54% | 42.800 |
| warm-clamp-accfit-dyn | 196ns | 191ns | 203ns | -62.67% | 41.807 |
| warm-clamp-head | 223ns | 219ns | 228ns | -57.43% | 36.656 |
| warm-clamp-min-lanes | 444ns | 432ns | 459ns | -15.42% | 18.451 |
| warm-clamp-minimum | 10033ns | 9646ns | 10952ns | +1811.35% | 0.816 |

## Performance model

- Peak throughput: **44.099 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 15.748 | 35.7% |
| warm-clamp-accfit | 43.116 | 97.8% |
| warm-clamp-accfit-dyn | 41.967 | 95.2% |
| warm-clamp-head | 36.686 | 83.2% |
| warm-clamp-min-lanes | 18.513 | 42.0% |
| warm-clamp-minimum | 0.837 | 1.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 587ns | 587ns | base |
| warm-clamp-accfit | 255ns | 255ns | -56.64% |
| warm-clamp-accfit-dyn | 259ns | 259ns | -55.98% |
| warm-clamp-head | 284ns | 284ns | -51.72% |
| warm-clamp-min-lanes | 531ns | 531ns | -9.55% |
| warm-clamp-minimum | 10104ns | 10104ns | +1620.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 520ns | base | --- | [519, 524] | --- | --- | --- | --- |
| warm-clamp-accfit | 190ns | -332.5ns (-63.9%) | [-336, -329]ns | [189, 192] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 195ns | -327.3ns (-62.9%) | [-332, -325]ns | [193, 197] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 223ns | -298.8ns (-57.4%) | [-305, -295]ns | [222, 225] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 442ns | -79.8ns (-15.3%) | [-86, -76]ns | [440, 446] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 9789ns | +9268.8ns (+1781.8%) | [+9229, +9448]ns | [9758, 9975] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 539ns | -63.0% | -62.7% | -58.3% | -15.4% | +1730.5% |
| 2 | 538ns | -63.4% | -62.6% | -58.8% | -15.0% | +1773.8% |
| 3 | 536ns | -63.1% | -62.0% | -58.0% | -14.1% | +1740.4% |
| 4 | 536ns | -63.2% | -62.8% | -58.2% | -14.5% | +1719.7% |
| 5 | 536ns | -63.5% | -62.4% | -57.7% | -15.7% | +1764.7% |
| 6 | 537ns | -63.1% | -62.1% | -57.4% | -17.6% | +1717.7% |
| 7 | 535ns | -62.7% | -62.2% | -57.9% | -17.2% | +1740.8% |
| 8 | 538ns | -62.6% | -62.7% | -57.5% | -16.7% | +1774.3% |
| 9 | 539ns | -62.4% | -62.2% | -58.0% | -17.3% | +1768.3% |
| 10 | 541ns | -63.7% | -62.6% | -58.1% | -18.2% | +1803.3% |
| 11 | 519ns | -63.7% | -62.0% | -56.0% | -14.6% | +1776.0% |
| 12 | 517ns | -63.2% | -62.7% | -56.2% | -14.5% | +1782.9% |
| 13 | 517ns | -63.5% | -62.6% | -56.7% | -14.8% | +1788.9% |
| 14 | 515ns | -63.5% | -61.5% | -56.3% | -14.2% | +1786.6% |
| 15 | 518ns | -63.2% | -62.7% | -56.9% | -13.8% | +1779.9% |
| 16 | 519ns | -62.9% | -62.8% | -56.2% | -15.2% | +1787.7% |
| 17 | 518ns | -63.5% | -62.3% | -55.6% | -15.3% | +1780.8% |
| 18 | 519ns | -63.1% | -63.0% | -56.5% | -14.4% | +1781.6% |
| 19 | 517ns | -63.3% | -62.5% | -56.7% | -14.8% | +1788.3% |
| 20 | 517ns | -63.7% | -62.2% | -56.8% | -14.5% | +1782.5% |
| 21 | 523ns | -62.9% | -60.3% | -57.6% | -12.4% | +1771.5% |
| 22 | 520ns | -64.4% | -62.0% | -57.3% | -13.0% | +1775.2% |
| 23 | 518ns | -64.6% | -62.2% | -56.9% | -12.2% | +1781.8% |
| 24 | 523ns | -62.7% | -62.6% | -57.4% | -10.7% | +1790.1% |
| 25 | 518ns | -63.6% | -62.3% | -56.9% | -10.6% | +1787.5% |
| 26 | 518ns | -63.0% | -62.2% | -56.9% | -13.7% | +1822.9% |
| 27 | 519ns | -63.0% | -63.3% | -57.2% | -13.9% | +1829.2% |
| 28 | 517ns | -63.2% | -62.0% | -56.2% | -13.8% | +1854.7% |
| 29 | 522ns | -63.7% | -62.8% | -57.4% | -16.7% | +1837.5% |
| 30 | 519ns | -63.2% | -62.6% | -56.7% | -15.6% | +1834.3% |
| 31 | 525ns | -64.1% | -63.3% | -58.1% | -17.2% | +2313.5% |
| 32 | 520ns | -64.1% | -63.0% | -57.9% | -16.8% | +2080.5% |
| 33 | 533ns | -64.9% | -64.2% | -59.1% | -18.9% | +2079.1% |
| 34 | 519ns | -64.2% | -63.6% | -57.5% | -15.6% | +1954.2% |
| 35 | 524ns | -63.9% | -63.7% | -57.8% | -17.5% | +1751.7% |
| 36 | 539ns | -64.6% | -64.2% | -59.3% | -19.3% | +1676.5% |
| 37 | 523ns | -64.4% | -63.5% | -58.3% | -17.4% | +1729.0% |
| 38 | 518ns | -64.1% | -62.9% | -57.6% | -16.4% | +1749.8% |
| 39 | 535ns | -65.2% | -64.7% | -59.2% | -19.2% | +1690.7% |
| 40 | 518ns | -63.7% | -62.9% | -57.3% | -17.5% | +1992.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.586 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.673 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.673 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.699 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.771 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.536 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.7ns | 524.9ns | 0.5% |  |
| warm-clamp-accfit | 2.7ns | 191.4ns | 1.4% |  |
| warm-clamp-accfit-dyn | 2.7ns | 195.9ns | 1.4% |  |
| warm-clamp-head | 2.3ns | 223.5ns | 1.0% |  |
| warm-clamp-min-lanes | 2.4ns | 444.0ns | 0.5% |  |
| warm-clamp-minimum | 2.5ns | 10033.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 516.8-538.3 ns)
    516.8 |########################################
    517.9 |#########################
    519.0 |###############
    520.1 |#####
    521.1 |#####
    522.2 |##########
    523.3 |##########
    524.3 |#####
    525.4 |
    526.5 |
    527.6 |
    528.6 |
    529.7 |
    530.8 |
    531.9 |#####
    532.9 |
    534.0 |##########
    535.1 |##########
    536.1 |##########
    537.2 |##########
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 185.8-199.2 ns)
    185.8 |################################
    186.4 |################
    187.1 |
    187.8 |########################################
    188.4 |########################
    189.1 |################
    189.8 |################################
    190.5 |########
    191.1 |########################
    191.8 |########
    192.5 |########
    193.1 |########
    193.8 |
    194.5 |########
    195.1 |########
    195.8 |########
    196.5 |########
    197.2 |########
    197.8 |################
    198.5 |
  (2 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 190.5-203.2 ns)
    190.5 |########################
    191.2 |
    191.8 |########################
    192.4 |########################################
    193.1 |################
    193.7 |########################
    194.3 |
    195.0 |################################
    195.6 |########################
    196.2 |
    196.9 |################
    197.5 |
    198.1 |########
    198.8 |
    199.4 |########
    200.0 |########
    200.7 |################
    201.3 |########
    201.9 |################
    202.6 |
  (3 below, 4 above range)

warm-clamp-head (n=40, range 219.1-227.7 ns)
    219.1 |########
    219.5 |########
    219.9 |########
    220.4 |########
    220.8 |########################
    221.2 |
    221.7 |########
    222.1 |########################
    222.5 |########
    223.0 |########################################
    223.4 |################
    223.8 |########
    224.3 |########
    224.7 |################################
    225.1 |########
    225.5 |
    226.0 |########################
    226.4 |################
    226.8 |########
    227.3 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 431.9-459.3 ns)
    431.9 |########################################
    433.3 |################
    434.6 |########
    436.0 |
    437.4 |########################
    438.7 |########
    440.1 |################
    441.5 |################################
    442.9 |################################
    444.2 |########
    445.6 |################
    447.0 |########################
    448.3 |
    449.7 |
    451.1 |########
    452.5 |########
    453.8 |########
    455.2 |########
    456.6 |################
    457.9 |########
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 9646.5-10951.7 ns)
   9646.5 |###
   9711.7 |########################################
   9777.0 |############
   9842.2 |#########
   9907.5 |###
   9972.8 |######
  10038.0 |###############
  10103.3 |###
  10168.6 |
  10233.8 |
  10299.1 |###
  10364.3 |
  10429.6 |
  10494.9 |
  10560.1 |
  10625.4 |###
  10690.7 |
  10755.9 |
  10821.2 |###
  10886.4 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.54 (measurement drift or warm-up artifact)
