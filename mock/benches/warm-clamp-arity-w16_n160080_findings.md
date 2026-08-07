# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 63% (significant)

warm-clamp-accfit is -326 ns (63%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 51.7x slower than the field

warm-clamp-minimum (9.75 us) is 51.7x the fastest (189 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes, warm-clamp-acc64} vs {warm-clamp-minimum} (1795% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-head, warm-clamp-min-lanes, warm-clamp-acc64} and a slow tier {warm-clamp-minimum} with a 1795% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 51.7x the fastest

Fastest warm-clamp-accfit (189 ns) to slowest warm-clamp-minimum (9.75 us): 51.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 188.6 ns median (-63.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 51.71x (fastest 188.6 ns, slowest 9750.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 581ns | 578ns | 569ns | 579ns | 601ns | base |
| warm-clamp-accfit | 252ns | 250ns | 246ns | 252ns | 261ns | -56.59% |
| warm-clamp-accfit-dyn | 260ns | 259ns | 255ns | 259ns | 269ns | -55.27% |
| warm-clamp-head | 299ns | 283ns | 276ns | 287ns | 358ns | -48.56% |
| warm-clamp-min-lanes | 308ns | 306ns | 301ns | 306ns | 320ns | -47.03% |
| warm-clamp-minimum | 9888ns | 9819ns | 9687ns | 9826ns | 10272ns | +1601.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 519ns | 509ns | 538ns | base | 15.774 |
| warm-clamp-accfit | 191ns | 186ns | 198ns | -63.32% | 43.000 |
| warm-clamp-accfit-dyn | 198ns | 193ns | 205ns | -61.82% | 41.318 |
| warm-clamp-head | 237ns | 219ns | 285ns | -54.42% | 34.606 |
| warm-clamp-min-lanes | 238ns | 232ns | 242ns | -54.19% | 34.431 |
| warm-clamp-minimum | 9819ns | 9621ns | 10200ns | +1790.70% | 0.834 |

## Performance model

- Peak throughput: **44.162 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 15.921 | 36.1% |
| warm-clamp-accfit | 43.447 | 98.4% |
| warm-clamp-accfit-dyn | 41.478 | 93.9% |
| warm-clamp-head | 36.645 | 83.0% |
| warm-clamp-min-lanes | 34.435 | 78.0% |
| warm-clamp-minimum | 0.840 | 1.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 581ns | 581ns | base |
| warm-clamp-accfit | 252ns | 252ns | -56.59% |
| warm-clamp-accfit-dyn | 260ns | 260ns | -55.27% |
| warm-clamp-head | 299ns | 299ns | -48.56% |
| warm-clamp-min-lanes | 308ns | 308ns | -47.03% |
| warm-clamp-minimum | 9888ns | 9888ns | +1601.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 515ns | base | --- | [510, 521] | --- | --- | --- | --- |
| warm-clamp-accfit | 189ns | -325.0ns (-63.2%) | [-329, -324]ns | [187, 192] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 198ns | -318.1ns (-61.8%) | [-321, -316]ns | [196, 200] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 224ns | -289.8ns (-56.3%) | [-291, -289]ns | [221, 228] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 238ns | -278.0ns (-54.0%) | [-281, -276]ns | [237, 240] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 9751ns | +9235.0ns (+1794.8%) | [+9222, +9251]ns | [9743, 9771] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 510ns | -63.2% | -61.5% | -57.1% | -53.7% | +1815.8% |
| 2 | 510ns | -63.3% | -61.5% | -56.9% | -53.3% | +1813.6% |
| 3 | 510ns | -63.4% | -62.0% | -56.8% | -53.7% | +1812.6% |
| 4 | 511ns | -63.6% | -62.1% | -57.3% | -54.8% | +1807.9% |
| 5 | 508ns | -63.3% | -61.7% | -57.2% | -53.8% | +1819.1% |
| 6 | 510ns | -63.5% | -62.3% | -57.1% | -53.5% | +1819.1% |
| 7 | 509ns | -63.9% | -61.5% | -56.8% | -55.3% | +1827.5% |
| 8 | 510ns | -63.5% | -62.0% | -56.9% | -55.0% | +2028.9% |
| 9 | 509ns | -63.0% | -61.3% | -57.0% | -53.7% | +1851.2% |
| 10 | 510ns | -63.4% | -61.8% | -57.1% | -54.2% | +1806.1% |
| 11 | 510ns | -63.8% | -61.0% | -56.3% | -53.5% | +1825.2% |
| 12 | 511ns | -63.4% | -61.3% | -56.4% | -53.9% | +1780.6% |
| 13 | 510ns | -63.6% | -62.4% | -54.7% | -53.0% | +1776.7% |
| 14 | 508ns | -63.2% | -60.7% | +3.8% | -53.2% | +1793.2% |
| 15 | 510ns | -63.3% | -61.8% | -56.3% | -53.5% | +2026.7% |
| 16 | 513ns | -63.2% | -61.5% | -56.9% | -53.8% | +1936.5% |
| 17 | 509ns | -63.1% | -61.6% | -56.8% | -54.5% | +1791.4% |
| 18 | 512ns | -63.4% | -62.0% | -57.1% | -53.7% | +1782.5% |
| 19 | 510ns | -63.7% | -61.8% | -56.8% | -53.4% | +1785.9% |
| 20 | 512ns | -63.4% | -62.4% | -56.4% | -54.0% | +1769.2% |
| 21 | 537ns | -63.1% | -62.0% | -53.3% | -55.0% | +1760.0% |
| 22 | 540ns | -63.6% | -63.0% | -53.9% | -55.5% | +1707.7% |
| 23 | 538ns | -63.3% | -63.2% | -53.6% | -55.7% | +1717.4% |
| 24 | 538ns | -63.4% | -62.8% | -53.5% | -54.9% | +1709.0% |
| 25 | 539ns | -63.6% | -62.8% | -53.6% | -55.5% | +1733.1% |
| 26 | 537ns | -63.8% | -62.1% | -53.8% | -55.1% | +1713.4% |
| 27 | 537ns | -63.3% | -61.8% | -53.5% | -54.9% | +1729.0% |
| 28 | 538ns | -63.7% | -61.7% | -53.4% | -55.2% | +1709.3% |
| 29 | 537ns | -62.6% | -61.4% | -53.3% | -55.2% | +1735.4% |
| 30 | 536ns | -62.3% | -61.2% | -53.8% | -55.2% | +1714.5% |
| 31 | 522ns | -63.3% | -62.3% | -56.8% | -53.3% | +1765.1% |
| 32 | 521ns | -63.2% | -61.8% | -56.6% | -53.4% | +1776.0% |
| 33 | 520ns | -63.1% | -62.3% | -56.0% | -54.0% | +1773.0% |
| 34 | 518ns | -63.9% | -61.2% | -55.9% | -52.8% | +1783.2% |
| 35 | 521ns | -63.4% | -61.3% | -57.1% | -54.0% | +1770.7% |
| 36 | 516ns | -63.3% | -60.8% | -55.9% | -53.4% | +1794.2% |
| 37 | 520ns | -63.2% | -61.8% | -57.4% | -54.4% | +1779.9% |
| 38 | 522ns | -62.1% | -61.6% | -56.9% | -53.9% | +1768.3% |
| 39 | 520ns | -63.1% | -61.3% | -57.0% | -53.7% | +1777.9% |
| 40 | 518ns | -63.4% | -61.9% | -57.2% | -54.5% | +1780.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.897 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.764 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.575 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.013 | ok |
| warm-clamp-min-lanes | 0.551 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.210 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 39/40, lost 1/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.4ns | 519.3ns | 0.5% |  |
| warm-clamp-accfit | 2.5ns | 190.5ns | 1.3% |  |
| warm-clamp-accfit-dyn | 2.4ns | 198.3ns | 1.2% |  |
| warm-clamp-head | 2.5ns | 236.7ns | 1.1% |  |
| warm-clamp-min-lanes | 2.6ns | 237.9ns | 1.1% |  |
| warm-clamp-minimum | 2.2ns | 9819.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 508.9-537.8 ns)
    508.9 |########################################
    510.3 |##############################
    511.8 |##########
    513.2 |
    514.7 |
    516.1 |#####
    517.6 |##########
    519.0 |###############
    520.5 |####################
    521.9 |
    523.4 |
    524.8 |
    526.2 |
    527.7 |
    529.1 |
    530.6 |
    532.0 |
    533.5 |
    534.9 |#####
    536.4 |#########################
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 185.5-198.3 ns)
    185.5 |###########
    186.1 |########################################
    186.8 |#################
    187.4 |######################
    188.1 |#####
    188.7 |###########
    189.3 |#####
    190.0 |
    190.6 |###########
    191.3 |###########
    191.9 |###########
    192.5 |
    193.2 |
    193.8 |
    194.5 |###########
    195.1 |
    195.7 |#####
    196.4 |#####
    197.0 |######################
    197.6 |#####
  (3 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 193.4-204.6 ns)
    193.4 |########################
    194.0 |
    194.5 |########################################
    195.1 |########
    195.7 |########################
    196.2 |########################
    196.8 |
    197.3 |########################
    197.9 |################
    198.5 |################
    199.0 |
    199.6 |########################
    200.1 |################
    200.7 |################
    201.3 |########
    201.8 |
    202.4 |########
    202.9 |########
    203.5 |########
    204.1 |
  (3 below, 4 above range)

warm-clamp-head (n=40, range 218.8-284.7 ns)
    218.8 |########################################
    222.1 |#############################
    225.4 |##########
    228.7 |#######
    232.0 |
    235.3 |
    238.6 |
    241.9 |
    245.2 |#######
    248.5 |#############################
    251.8 |
    255.0 |
    258.3 |
    261.6 |
    264.9 |
    268.2 |
    271.5 |
    274.8 |
    278.1 |
    281.4 |
  (5 below, 1 above range)

warm-clamp-min-lanes (n=40, range 232.4-242.4 ns)
    232.4 |
    232.9 |
    233.4 |######
    233.9 |
    234.4 |######
    234.9 |
    235.4 |#################################
    235.9 |######
    236.4 |
    236.9 |########################################
    237.4 |######
    237.9 |####################
    238.4 |
    238.9 |
    239.4 |####################
    239.9 |########################################
    240.4 |######
    240.9 |#############
    241.4 |######
    241.9 |######
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 9621.0-10200.4 ns)
   9621.0 |##########
   9649.9 |
   9678.9 |
   9707.9 |##################
   9736.9 |########################################
   9765.8 |#####################
   9794.8 |##############
   9823.8 |###
   9852.7 |###
   9881.7 |
   9910.7 |###
   9939.7 |
   9968.6 |###
   9997.6 |
  10026.6 |
  10055.5 |
  10084.5 |
  10113.5 |
  10142.5 |
  10171.4 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **warm-clamp-head**: CV=20.3% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: autocorrelation=0.55 (measurement drift or warm-up artifact)
