# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.55)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 1.5x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 301 ns but its worst 20% at 460 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 303.2 ns median (-10.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.19x (fastest 303.2 ns, slowest 361.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 590ns | 589ns | 531ns | 582ns | 676ns | base |
| warm-clamp-accfit | 597ns | 593ns | 455ns | 599ns | 730ns | +1.07% |
| warm-clamp-accfit-dyn | 638ns | 638ns | 523ns | 625ns | 793ns | +8.10% |
| warm-clamp-head | 569ns | 586ns | 440ns | 568ns | 700ns | -3.64% |
| warm-clamp-min-lanes | 553ns | 535ns | 475ns | 543ns | 663ns | -6.29% |
| warm-clamp-minimum | 627ns | 589ns | 531ns | 606ns | 787ns | +6.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 332ns | 305ns | 356ns | base | 24.654 |
| warm-clamp-accfit | 333ns | 262ns | 385ns | +0.34% | 24.572 |
| warm-clamp-accfit-dyn | 365ns | 301ns | 460ns | +9.95% | 22.423 |
| warm-clamp-head | 317ns | 252ns | 380ns | -4.57% | 25.834 |
| warm-clamp-min-lanes | 305ns | 273ns | 343ns | -8.18% | 26.852 |
| warm-clamp-minimum | 322ns | 276ns | 371ns | -3.16% | 25.458 |

## Performance model

- Peak throughput: **32.497 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 24.212 | 74.5% |
| warm-clamp-accfit | 24.045 | 74.0% |
| warm-clamp-accfit-dyn | 22.633 | 69.6% |
| warm-clamp-head | 24.399 | 75.1% |
| warm-clamp-min-lanes | 27.018 | 83.1% |
| warm-clamp-minimum | 25.850 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 590ns | 590ns | base |
| warm-clamp-accfit | 597ns | 597ns | +1.07% |
| warm-clamp-accfit-dyn | 638ns | 638ns | +8.10% |
| warm-clamp-head | 569ns | 569ns | -3.64% |
| warm-clamp-min-lanes | 553ns | 553ns | -6.29% |
| warm-clamp-minimum | 627ns | 627ns | +6.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 338ns | base | --- | [338, 339] | --- | --- | --- | --- |
| warm-clamp-accfit | 341ns | +12.1ns (+3.6%) | [+1, +36]ns | [340, 342] | YES | 0.0161 | 0.0064 | 0 |
| warm-clamp-accfit-dyn | 362ns | no significant difference | [-1, +40]ns | [337, 379] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-head | 336ns | no significant difference | [-37, +31]ns | [303, 337] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-min-lanes | 303ns | -31.6ns (-9.4%) | [-35, -22]ns | [303, 304] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 317ns | no significant difference | [-35, +10]ns | [303, 337] | no | 0.4470 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 339ns | +13.0% | +11.7% | -10.6% | -0.7% | -5.8% |
| 1 | 2 | 338ns | -23.7% | +12.0% | -25.5% | -10.4% | -16.5% |
| 1 | 3 | 338ns | +0.7% | -0.4% | +11.7% | -10.4% | +5.5% |
| 1 | 4 | 305ns | +11.8% | -0.7% | +10.3% | -9.7% | +3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.074 | ok |
| warm-clamp-accfit | -0.536 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.267 | moderate+ |
| warm-clamp-head | -0.217 | moderate- |
| warm-clamp-min-lanes | -0.000 | ok |
| warm-clamp-minimum | -0.550 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 94.3ns | 332.3ns | 28.4% | HIGH |
| warm-clamp-accfit | 95.2ns | 333.4ns | 28.6% | HIGH |
| warm-clamp-accfit-dyn | 100.0ns | 365.3ns | 27.4% | HIGH |
| warm-clamp-head | 91.3ns | 317.1ns | 28.8% | HIGH |
| warm-clamp-min-lanes | 88.5ns | 305.1ns | 29.0% | HIGH |
| warm-clamp-minimum | 104.5ns | 321.8ns | 32.5% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 304.8-356.0 ns)
    304.8 |#########
    307.3 |
    309.9 |
    312.4 |
    315.0 |#
    317.6 |
    320.1 |###
    322.7 |
    325.2 |
    327.8 |
    330.4 |
    332.9 |
    335.5 |#####
    338.0 |########################################
    340.6 |#
    343.2 |
    345.7 |
    348.3 |
    350.8 |
    353.4 |
  (5 below, 2 above range)

warm-clamp-accfit (n=40, range 261.9-384.7 ns)
    261.9 |
    268.1 |
    274.2 |############
    280.3 |
    286.5 |
    292.6 |
    298.8 |
    304.9 |######
    311.0 |
    317.2 |
    323.3 |###
    329.5 |
    335.6 |########################################
    341.7 |###
    347.9 |
    354.0 |
    360.2 |###
    366.3 |
    372.4 |
    378.6 |#################################
  (6 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 300.5-459.7 ns)
    300.5 |#####################
    308.5 |
    316.4 |
    324.4 |
    332.3 |#####################
    340.3 |
    348.3 |##
    356.2 |
    364.2 |
    372.1 |########################################
    380.1 |
    388.1 |
    396.0 |
    404.0 |
    411.9 |
    419.9 |
    427.9 |
    435.8 |
    443.8 |
    451.7 |
  (1 below, 3 above range)

warm-clamp-head (n=40, range 252.1-380.5 ns)
    252.1 |####################
    258.5 |
    264.9 |
    271.3 |
    277.8 |
    284.2 |
    290.6 |
    297.0 |#######################
    303.5 |######
    309.9 |
    316.3 |
    322.7 |
    329.1 |
    335.6 |########################################
    342.0 |
    348.4 |###
    354.8 |
    361.2 |
    367.7 |
    374.1 |#######################
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 272.8-343.2 ns)
    272.8 |###############
    276.3 |####
    279.9 |##
    283.4 |
    286.9 |
    290.4 |
    293.9 |
    297.5 |
    301.0 |########################################
    304.5 |##
    308.0 |
    311.5 |
    315.1 |
    318.6 |
    322.1 |
    325.6 |
    329.1 |
    332.7 |
    336.2 |#################
    339.7 |##
  (1 below, 1 above range)

warm-clamp-minimum (n=40, range 276.3-371.2 ns)
    276.3 |
    281.1 |
    285.8 |################
    290.6 |####
    295.3 |
    300.1 |####################
    304.8 |
    309.5 |
    314.3 |########################
    319.0 |
    323.8 |
    328.5 |####
    333.3 |########################################
    338.0 |
    342.7 |########
    347.5 |
    352.2 |
    357.0 |
    361.7 |
    366.5 |
  (5 below, 6 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=28.6% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=28.6% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: CV=20.3% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: bridge=27.3% of algo (FFI overhead may distort results)
- **warm-clamp-head**: bridge=27.5% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=29.0% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=31.7% of algo (FFI overhead may distort results)
