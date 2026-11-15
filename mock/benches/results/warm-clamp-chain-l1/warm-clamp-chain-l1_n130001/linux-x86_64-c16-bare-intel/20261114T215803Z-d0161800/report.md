# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (237 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 88 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-minimum dominates: 25% faster than the next best (warm-clamp-min-lanes)

warm-clamp-minimum (88 ns) leads warm-clamp-min-lanes (110 ns) by 25%, a clear separation rather than a photo finish. CV 43.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 57% (significant)

warm-clamp-minimum is -134 ns (57%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is an outlier: 2.7x slower than the field

warm-clamp-acc64 (237 ns) is 2.7x the fastest (88 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is fastest but the noisiest (CV 43.4%)

warm-clamp-minimum wins on median (88 ns) yet has the highest variance (CV 43.4%), while warm-clamp-min-lanes is the steadiest (CV 9.3%, 110 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-head, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64} (59% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-head, warm-clamp-accfit, warm-clamp-accfit-dyn, warm-clamp-acc64} with a 59% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-minimum is inconsistent: worst-20% is 1.7x its best-20%

warm-clamp-minimum's best 20% of batches run at 83 ns but its worst 20% at 140 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 88.0 ns median (-62.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 2.69x (fastest 88.0 ns, slowest 236.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 586ns | 574ns | 446ns | 565ns | 787ns | base |
| warm-clamp-accfit | 437ns | 407ns | 370ns | 422ns | 547ns | -25.40% |
| warm-clamp-accfit-dyn | 472ns | 447ns | 406ns | 460ns | 577ns | -19.37% |
| warm-clamp-head | 412ns | 400ns | 334ns | 398ns | 531ns | -29.69% |
| warm-clamp-min-lanes | 397ns | 362ns | 322ns | 372ns | 544ns | -32.29% |
| warm-clamp-minimum | 354ns | 315ns | 298ns | 334ns | 469ns | -39.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 242ns | 196ns | 295ns | base | 33.880 |
| warm-clamp-accfit | 186ns | 163ns | 211ns | -23.14% | 44.079 |
| warm-clamp-accfit-dyn | 200ns | 177ns | 233ns | -17.31% | 40.971 |
| warm-clamp-head | 178ns | 145ns | 228ns | -26.48% | 46.081 |
| warm-clamp-min-lanes | 110ns | 97ns | 126ns | -54.48% | 74.425 |
| warm-clamp-minimum | 99ns | 83ns | 140ns | -59.07% | 82.775 |

## Performance model

- Peak throughput: **98.373 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 34.617 | 35.2% |
| warm-clamp-accfit | 45.855 | 46.6% |
| warm-clamp-accfit-dyn | 42.107 | 42.8% |
| warm-clamp-head | 46.999 | 47.8% |
| warm-clamp-min-lanes | 74.745 | 76.0% |
| warm-clamp-minimum | 93.091 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 586ns | 586ns | base |
| warm-clamp-accfit | 437ns | 437ns | -25.40% |
| warm-clamp-accfit-dyn | 472ns | 472ns | -19.37% |
| warm-clamp-head | 412ns | 412ns | -29.69% |
| warm-clamp-min-lanes | 397ns | 397ns | -32.29% |
| warm-clamp-minimum | 354ns | 354ns | -39.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 237ns | base | --- | [206, 263] | --- | --- | --- | --- |
| warm-clamp-accfit | 179ns | -52.1ns (-22.0%) | [-89, -18]ns | [178, 197] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 195ns | -41.7ns (-17.6%) | [-85, -2]ns | [194, 195] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 174ns | -75.0ns (-31.7%) | [-78, -51]ns | [174, 174] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 110ns | -132.1ns (-55.8%) | [-154, -100]ns | [106, 112] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 88ns | -133.8ns (-56.5%) | [-156, -109]ns | [88, 88] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 196ns | +0.9% | +11.5% | -26.0% | -39.5% | -55.2% |
| 1 | 2 | 270ns | -28.2% | -28.1% | -35.4% | -59.7% | -59.2% |
| 1 | 3 | 294ns | -44.9% | -34.0% | -40.7% | -61.3% | -70.9% |
| 1 | 4 | 196ns | -9.2% | -0.9% | -11.0% | -50.4% | -55.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.260 | moderate- |
| warm-clamp-accfit | 0.052 | ok |
| warm-clamp-accfit-dyn | -0.080 | ok |
| warm-clamp-head | -0.082 | ok |
| warm-clamp-min-lanes | -0.252 | moderate- |
| warm-clamp-minimum | -0.430 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 123.6ns | 241.8ns | 51.1% | HIGH |
| warm-clamp-accfit | 91.8ns | 185.8ns | 49.4% | HIGH |
| warm-clamp-accfit-dyn | 98.0ns | 199.9ns | 49.0% | HIGH |
| warm-clamp-head | 84.9ns | 177.8ns | 47.8% | HIGH |
| warm-clamp-min-lanes | 97.7ns | 110.1ns | 88.7% | HIGH |
| warm-clamp-minimum | 102.5ns | 99.0ns | 103.6% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 195.8-295.1 ns)
    195.8 |########################################
    200.8 |
    205.8 |
    210.7 |###
    215.7 |##########
    220.7 |#######
    225.6 |
    230.6 |
    235.6 |
    240.5 |
    245.5 |
    250.4 |#####################
    255.4 |
    260.4 |
    265.3 |
    270.3 |###
    275.3 |
    280.2 |
    285.2 |###
    290.2 |#########################
  (3 below, 5 above range)

warm-clamp-accfit (n=40, range 162.7-211.2 ns)
    162.7 |##
    165.2 |
    167.6 |##
    170.0 |
    172.4 |
    174.9 |##
    177.3 |########################################
    179.7 |
    182.1 |
    184.6 |
    187.0 |
    189.4 |##
    191.8 |
    194.3 |##
    196.7 |###############################
    199.1 |##
    201.5 |##
    204.0 |
    206.4 |
    208.8 |
  (6 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 177.1-233.0 ns)
    177.1 |
    179.9 |#
    182.7 |#
    185.5 |
    188.3 |
    191.1 |#
    193.9 |########################################
    196.7 |
    199.5 |
    202.3 |
    205.1 |
    207.9 |
    210.7 |
    213.5 |
    216.3 |#########
    219.1 |#######
    221.8 |
    224.6 |
    227.4 |
    230.2 |
  (6 below, 1 above range)

warm-clamp-head (n=40, range 145.4-227.5 ns)
    145.4 |############
    149.5 |
    153.6 |
    157.7 |
    161.8 |
    165.9 |
    170.0 |############
    174.1 |########################################
    178.2 |
    182.3 |##
    186.4 |
    190.5 |####
    194.7 |##
    198.8 |
    202.9 |
    207.0 |
    211.1 |
    215.2 |
    219.3 |
    223.4 |
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 96.9-125.9 ns)
     96.9 |########################################
     98.4 |
     99.8 |#############
    101.3 |
    102.7 |######
    104.2 |#############
    105.6 |######
    107.1 |##########################
    108.5 |######
    110.0 |#################################
    111.4 |#############
    112.8 |#############
    114.3 |
    115.7 |######
    117.2 |
    118.6 |######
    120.1 |
    121.5 |######
    123.0 |####################
    124.4 |#############
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 83.3-139.5 ns)
     83.3 |#
     86.1 |########################################
     88.9 |
     91.7 |
     94.5 |
     97.3 |
    100.1 |
    103.0 |
    105.8 |
    108.6 |#############
    111.4 |#
    114.2 |
    117.0 |
    119.8 |
    122.6 |
    125.5 |#
    128.3 |
    131.1 |
    133.9 |
    136.7 |
  (4 below, 1 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=52.7% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=48.8% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=49.7% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=32.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=50.3% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=87.3% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: CV=38.6% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: bridge=97.5% of algo (FFI overhead may distort results)
