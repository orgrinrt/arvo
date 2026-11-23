# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 22% faster than the next best (warm-clamp-minimum)

warm-clamp-min-lanes (91 ns) leads warm-clamp-minimum (111 ns) by 22%, a clear separation rather than a photo finish. CV 12.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes beats baseline by 33% (significant)

warm-clamp-min-lanes is -45 ns (33%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-acc64 is inconsistent: worst-20% is 4.6x its best-20%

warm-clamp-acc64's best 20% of batches run at 56 ns but its worst 20% at 257 ns (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 91.0 ns median (-32.9% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.97x (fastest 91.0 ns, slowest 178.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112832026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 337ns | 310ns | 131ns | 322ns | 589ns | base |
| warm-clamp-accfit | 448ns | 448ns | 397ns | 438ns | 532ns | +33.00% |
| warm-clamp-accfit-dyn | 421ns | 401ns | 333ns | 418ns | 520ns | +24.94% |
| warm-clamp-head | 443ns | 401ns | 335ns | 413ns | 637ns | +31.27% |
| warm-clamp-min-lanes | 339ns | 318ns | 314ns | 326ns | 404ns | +0.60% |
| warm-clamp-minimum | 361ns | 368ns | 305ns | 356ns | 431ns | +6.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 145ns | 56ns | 257ns | base | 56.552 |
| warm-clamp-accfit | 191ns | 175ns | 222ns | +32.16% | 42.789 |
| warm-clamp-accfit-dyn | 178ns | 145ns | 218ns | +22.99% | 45.982 |
| warm-clamp-head | 189ns | 147ns | 276ns | +30.25% | 43.417 |
| warm-clamp-min-lanes | 95ns | 89ns | 108ns | -34.65% | 86.539 |
| warm-clamp-minimum | 114ns | 100ns | 126ns | -21.12% | 71.696 |

## Performance model

- Peak throughput: **145.668 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 60.435 | 41.5% |
| warm-clamp-accfit | 45.778 | 31.4% |
| warm-clamp-accfit-dyn | 46.986 | 32.3% |
| warm-clamp-head | 48.516 | 33.3% |
| warm-clamp-min-lanes | 90.071 | 61.8% |
| warm-clamp-minimum | 73.669 | 50.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 337ns | 337ns | base |
| warm-clamp-accfit | 448ns | 448ns | +33.00% |
| warm-clamp-accfit-dyn | 421ns | 421ns | +24.94% |
| warm-clamp-head | 443ns | 443ns | +31.27% |
| warm-clamp-min-lanes | 339ns | 339ns | +0.60% |
| warm-clamp-minimum | 361ns | 361ns | +6.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 136ns | base | --- | [135, 136] | --- | --- | --- | --- |
| warm-clamp-accfit | 179ns | +42.8ns (+31.6%) | [+37, +43]ns | [178, 198] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-accfit-dyn | 174ns | +46.2ns (+34.0%) | [+9, +83]ns | [174, 175] | YES | 0.0166 | 0.0166 | 0 |
| warm-clamp-head | 169ns | +56.7ns (+41.8%) | [+12, +102]ns | [160, 176] | YES | 0.0037 | 0.0022 | 0 |
| warm-clamp-min-lanes | 91ns | -45.2ns (-33.4%) | [-46, -44]ns | [90, 92] | YES | 0.0080 | 0.0064 | 0 |
| warm-clamp-minimum | 111ns | -19.1ns (-14.1%) | [-27, -12]ns | [110, 121] | YES | 0.0037 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 136ns | +31.2% | +7.0% | +85.1% | -33.3% | -18.9% |
| 1 | 2 | 59ns | +279.6% | +196.8% | +172.9% | +69.1% | +71.9% |
| 1 | 3 | 251ns | -20.9% | -30.5% | -29.7% | -63.9% | -51.6% |
| 1 | 4 | 135ns | +32.0% | +61.1% | +8.7% | -33.5% | -8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.498 | moderate- |
| warm-clamp-accfit | -0.309 | moderate- |
| warm-clamp-accfit-dyn | -0.004 | ok |
| warm-clamp-head | -0.173 | ok |
| warm-clamp-min-lanes | -0.360 | moderate- |
| warm-clamp-minimum | 0.066 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 69.5ns | 144.9ns | 48.0% | HIGH |
| warm-clamp-accfit | 91.6ns | 191.4ns | 47.8% | HIGH |
| warm-clamp-accfit-dyn | 89.0ns | 178.2ns | 49.9% | HIGH |
| warm-clamp-head | 90.7ns | 188.7ns | 48.1% | HIGH |
| warm-clamp-min-lanes | 86.8ns | 94.7ns | 91.7% | HIGH |
| warm-clamp-minimum | 94.6ns | 114.3ns | 82.8% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 56.2-256.7 ns)
     56.2 |#################
     66.3 |
     76.3 |
     86.3 |
     96.3 |
    106.4 |
    116.4 |
    126.4 |########################################
    136.4 |##
    146.5 |##
    156.5 |
    166.5 |
    176.5 |
    186.5 |
    196.6 |
    206.6 |
    216.6 |########
    226.6 |
    236.7 |
    246.7 |###########
  (2 below, 1 above range)

warm-clamp-accfit (n=40, range 174.6-222.3 ns)
    174.6 |
    177.0 |########################################
    179.4 |
    181.8 |
    184.1 |
    186.5 |
    188.9 |
    191.3 |
    193.7 |
    196.1 |############
    198.5 |########
    200.8 |
    203.2 |
    205.6 |
    208.0 |
    210.4 |
    212.8 |
    215.2 |##
    217.5 |
    219.9 |##
  (2 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 145.4-218.2 ns)
    145.4 |#################
    149.0 |
    152.7 |
    156.3 |
    159.9 |
    163.6 |
    167.2 |
    170.9 |########################################
    174.5 |#################
    178.2 |
    181.8 |
    185.5 |
    189.1 |
    192.7 |
    196.4 |
    200.0 |
    203.7 |
    207.3 |
    211.0 |
    214.6 |######################
  (4 below, 2 above range)

warm-clamp-head (n=40, range 147.0-275.9 ns)
    147.0 |############################
    153.4 |########
    159.9 |################################
    166.3 |
    172.8 |########################################
    179.2 |
    185.7 |
    192.1 |
    198.5 |
    205.0 |
    211.4 |
    217.9 |
    224.3 |
    230.8 |
    237.2 |
    243.6 |
    250.1 |############################
    256.5 |
    263.0 |
    269.4 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 89.2-107.9 ns)
     89.2 |########################################
     90.1 |####################
     91.0 |#############
     92.0 |######
     92.9 |
     93.9 |######
     94.8 |
     95.7 |
     96.7 |
     97.6 |
     98.5 |####################
     99.5 |######
    100.4 |
    101.3 |######
    102.3 |
    103.2 |
    104.2 |
    105.1 |
    106.0 |
    107.0 |
  (3 below, 1 above range)

warm-clamp-minimum (n=40, range 100.1-125.8 ns)
    100.1 |########################################
    101.4 |
    102.7 |
    103.9 |
    105.2 |#############
    106.5 |
    107.8 |######
    109.1 |##########################
    110.4 |########################################
    111.7 |
    112.9 |
    114.2 |
    115.5 |
    116.8 |
    118.1 |######
    119.4 |
    120.6 |#################################
    121.9 |#################################
    123.2 |#################################
    124.5 |######
  (2 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=49.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **warm-clamp-acc64**: bridge=48.6% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=48.0% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=48.8% of algo (FFI overhead may distort results)
- **warm-clamp-head**: CV=27.2% (high variance, measurements may be unstable)
- **warm-clamp-head**: bridge=48.6% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=93.2% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=81.5% of algo (FFI overhead may distort results)
