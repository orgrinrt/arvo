# Clamping fold at 8 bits, arity 2 / 4 / 8 / 16: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 83% (significant)

warm-clamp-min-lanes is -615 ns (83%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 36.6x slower than the field

warm-clamp-accfit-dyn (4.64 us) is 36.6x the fastest (127 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-min-lanes, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-min-lanes (127 ns) and warm-clamp-minimum (127 ns) differ by 0.32%, inside the noise, even though the wider field spreads 3558.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.83)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (525% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 525% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 36.6x the fastest

Fastest warm-clamp-min-lanes (127 ns) to slowest warm-clamp-accfit-dyn (4.64 us): 36.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 126.9 ns median (-82.9% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 36.58x (fastest 126.9 ns, slowest 4642.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 809ns | 804ns | 800ns | 805ns | 830ns | base |
| warm-clamp-accfit | 288ns | 295ns | 245ns | 290ns | 323ns | -64.45% |
| warm-clamp-accfit-dyn | 4804ns | 4700ns | 4631ns | 4689ns | 5324ns | +493.87% |
| warm-clamp-head | 357ns | 352ns | 325ns | 353ns | 399ns | -55.88% |
| warm-clamp-min-lanes | 186ns | 186ns | 182ns | 185ns | 191ns | -77.03% |
| warm-clamp-minimum | 189ns | 186ns | 182ns | 186ns | 205ns | -76.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 746ns | 739ns | 758ns | base | 10.983 |
| warm-clamp-accfit | 219ns | 187ns | 247ns | -70.60% | 37.359 |
| warm-clamp-accfit-dyn | 4743ns | 4574ns | 5256ns | +535.92% | 1.727 |
| warm-clamp-head | 292ns | 267ns | 326ns | -60.79% | 28.013 |
| warm-clamp-min-lanes | 128ns | 126ns | 132ns | -82.90% | 64.217 |
| warm-clamp-minimum | 130ns | 125ns | 144ns | -82.51% | 62.804 |

## Performance model

- Peak throughput: **65.275 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 11.024 | 16.9% |
| warm-clamp-accfit | 36.280 | 55.6% |
| warm-clamp-accfit-dyn | 1.765 | 2.7% |
| warm-clamp-head | 28.326 | 43.4% |
| warm-clamp-min-lanes | 64.555 | 98.9% |
| warm-clamp-minimum | 64.352 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 809ns | 809ns | base |
| warm-clamp-accfit | 288ns | 288ns | -64.45% |
| warm-clamp-accfit-dyn | 4804ns | 4804ns | +493.87% |
| warm-clamp-head | 357ns | 357ns | -55.88% |
| warm-clamp-min-lanes | 186ns | 186ns | -77.03% |
| warm-clamp-minimum | 189ns | 189ns | -76.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 743ns | base | --- | [742, 744] | --- | --- | --- | --- |
| warm-clamp-accfit | 226ns | -517.1ns (-69.6%) | [-529, -513]ns | [213, 227] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4642ns | +3891.9ns (+523.7%) | [+3836, +3906]ns | [4577, 4653] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 289ns | -460.0ns (-61.9%) | [-471, -435]ns | [270, 307] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 127ns | -616.3ns (-82.9%) | [-617, -614]ns | [127, 127] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 127ns | -616.1ns (-82.9%) | [-618, -615]ns | [126, 129] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 742ns | -71.0% | +520.2% | -63.5% | -83.1% | -82.9% |
| 2 | 742ns | -71.3% | +516.1% | -63.6% | -82.9% | -82.9% |
| 3 | 742ns | -71.5% | +516.0% | -63.0% | -83.1% | -83.0% |
| 4 | 740ns | -71.3% | +518.1% | -64.0% | -82.9% | -82.9% |
| 5 | 743ns | -71.2% | +515.6% | -63.8% | -83.1% | -83.1% |
| 6 | 740ns | -71.2% | +518.6% | -63.7% | -82.8% | -82.8% |
| 7 | 740ns | -71.3% | +526.8% | -63.6% | -82.9% | -83.0% |
| 8 | 741ns | -67.8% | +526.7% | -63.9% | -82.9% | -83.1% |
| 9 | 745ns | -71.3% | +534.1% | -64.0% | -83.1% | -82.7% |
| 10 | 741ns | -71.2% | +527.6% | -63.3% | -83.0% | -82.7% |
| 11 | 743ns | -69.6% | +515.2% | -58.6% | -82.7% | -83.0% |
| 12 | 743ns | -69.5% | +527.6% | -58.5% | -82.3% | -83.2% |
| 13 | 740ns | -69.3% | +518.6% | -58.6% | -82.1% | -83.0% |
| 14 | 741ns | -69.5% | +517.9% | -58.6% | -81.8% | -83.0% |
| 15 | 742ns | -69.1% | +516.7% | -58.7% | -81.7% | -83.0% |
| 16 | 740ns | -69.5% | +526.7% | -58.7% | -82.4% | -83.0% |
| 17 | 743ns | -69.4% | +515.8% | -58.6% | -83.0% | -83.1% |
| 18 | 737ns | -69.4% | +520.8% | -58.4% | -82.8% | -82.9% |
| 19 | 744ns | -69.5% | +545.2% | -59.2% | -82.9% | -83.1% |
| 20 | 740ns | -69.3% | +649.4% | -58.4% | -82.8% | -83.0% |
| 21 | 741ns | -66.6% | +520.2% | -63.4% | -82.8% | -82.8% |
| 22 | 739ns | -66.5% | +519.1% | -63.7% | -82.6% | -83.0% |
| 23 | 744ns | -66.9% | +514.9% | -64.1% | -82.9% | -83.0% |
| 24 | 742ns | -66.8% | +515.9% | -63.7% | -82.4% | -70.0% |
| 25 | 760ns | -67.5% | +502.2% | -64.9% | -83.3% | -82.9% |
| 26 | 750ns | -66.9% | +509.6% | -64.2% | -83.1% | -82.6% |
| 27 | 744ns | -67.0% | +640.6% | -63.7% | -82.9% | -82.9% |
| 28 | 739ns | -66.7% | +650.9% | -63.6% | -82.8% | -82.8% |
| 29 | 740ns | -66.6% | +649.5% | -63.8% | -83.0% | -82.7% |
| 30 | 744ns | -66.8% | +645.8% | -64.5% | -82.9% | -82.7% |
| 31 | 780ns | -76.1% | +497.1% | -58.0% | -83.5% | -83.0% |
| 32 | 762ns | -75.3% | +527.2% | -57.2% | -83.6% | -82.7% |
| 33 | 754ns | -74.5% | +520.5% | -57.2% | -83.1% | -82.4% |
| 34 | 752ns | -75.5% | +520.6% | -56.6% | -83.2% | -82.0% |
| 35 | 752ns | -75.0% | +528.9% | -56.8% | -83.3% | -82.4% |
| 36 | 753ns | -75.4% | +516.5% | -57.1% | -83.2% | -82.6% |
| 37 | 752ns | -75.2% | +518.1% | -56.5% | -83.3% | -83.0% |
| 38 | 752ns | -75.1% | +517.9% | -56.8% | -83.1% | -82.8% |
| 39 | 753ns | -75.0% | +533.5% | -56.9% | -83.2% | -82.5% |
| 40 | 751ns | -74.7% | +519.2% | -56.8% | -83.3% | -82.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.534 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.827 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.559 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.824 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.661 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.004 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.4ns | 745.8ns | 0.3% |  |
| warm-clamp-accfit | 2.4ns | 219.3ns | 1.1% |  |
| warm-clamp-accfit-dyn | 2.6ns | 4743.0ns | 0.1% |  |
| warm-clamp-head | 2.4ns | 292.4ns | 0.8% |  |
| warm-clamp-min-lanes | 2.5ns | 127.6ns | 2.0% |  |
| warm-clamp-minimum | 2.4ns | 130.4ns | 1.8% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 739.3-758.1 ns)
    739.3 |###############
    740.3 |########################################
    741.2 |###############
    742.2 |###############
    743.1 |###################################
    744.0 |
    745.0 |#####
    745.9 |
    746.9 |
    747.8 |
    748.7 |
    749.7 |#####
    750.6 |#####
    751.6 |####################
    752.5 |##########
    753.4 |#####
    754.4 |
    755.3 |
    756.3 |
    757.2 |
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 186.8-247.1 ns)
    186.8 |####################
    189.8 |##########
    192.9 |
    195.9 |
    198.9 |
    201.9 |
    204.9 |
    207.9 |
    211.0 |########################################
    214.0 |#####
    217.0 |
    220.0 |
    223.0 |##########
    226.0 |###################################
    229.0 |#####
    232.1 |
    235.1 |
    238.1 |#####
    241.1 |
    244.1 |###################################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 4573.5-5255.9 ns)
   4573.5 |########################################
   4607.6 |#####
   4641.7 |########################
   4675.9 |##
   4710.0 |#####
   4744.1 |#####
   4778.2 |##
   4812.3 |
   4846.5 |
   4880.6 |
   4914.7 |
   4948.8 |
   4982.9 |
   5017.1 |
   5051.2 |
   5085.3 |
   5119.4 |
   5153.5 |
   5187.6 |
   5221.8 |
  (3 below, 5 above range)

warm-clamp-head (n=40, range 267.0-325.5 ns)
    267.0 |########################################
    270.0 |##################
    272.9 |###
    275.8 |
    278.7 |
    281.7 |
    284.6 |
    287.5 |
    290.4 |
    293.4 |
    296.3 |
    299.2 |
    302.1 |###
    305.1 |#########################
    308.0 |#######
    310.9 |
    313.8 |
    316.8 |
    319.7 |
    322.6 |#####################
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 125.5-131.5 ns)
    125.5 |######################
    125.8 |
    126.1 |############################
    126.4 |########################################
    126.7 |
    127.0 |##################################
    127.3 |############################
    127.6 |
    127.9 |
    128.2 |###########
    128.5 |#####
    128.8 |
    129.1 |
    129.4 |
    129.7 |
    130.0 |
    130.3 |#####
    130.6 |#####
    130.9 |#####
    131.2 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 125.5-143.9 ns)
    125.5 |########################################
    126.4 |####################
    127.3 |################
    128.3 |############
    129.2 |############
    130.1 |########
    131.0 |########
    131.9 |########
    132.9 |########
    133.8 |
    134.7 |####
    135.6 |
    136.5 |
    137.5 |
    138.4 |
    139.3 |
    140.2 |
    141.1 |
    142.1 |
    143.0 |
  (5 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.66 (measurement drift or warm-up artifact)
