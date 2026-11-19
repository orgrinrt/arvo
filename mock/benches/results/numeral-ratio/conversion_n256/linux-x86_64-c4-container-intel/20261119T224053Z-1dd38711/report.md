# A 16-bit fixed point at exponent -8 onto exponent -4, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_conversion_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_conversion_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_conversion_hand dominates: 58% faster than the next best (numeral_ratio_conversion_room)

numeral_ratio_conversion_hand (112 ns) leads numeral_ratio_conversion_room (177 ns) by 58%, a clear separation rather than a photo finish. CV 17.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_conversion_wide is an outlier: 74.6x slower than the field

numeral_ratio_conversion_wide (8.36 us) is 74.6x the fastest (112 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (numeral_ratio_conversion_hand)

The baseline numeral_ratio_conversion_hand is the fastest (112 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 74.6x the fastest

Fastest numeral_ratio_conversion_hand (112 ns) to slowest numeral_ratio_conversion_wide (8.36 us): 74.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (numeral_ratio_conversion_hand) is the fastest** at 112.0 ns median
- 2 variants significantly slower than baseline
- Spread: 74.64x (fastest 112.0 ns, slowest 8356.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A 16-bit fixed point at exponent -8 onto exponent -4, 64 records |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2300262086 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 227ns | 200ns | 195ns | 211ns | 305ns | base |
| numeral_ratio_conversion_room | 276ns | 265ns | 259ns | 265ns | 329ns | +22.03% |
| numeral_ratio_conversion_wide | 8853ns | 8453ns | 8193ns | 8435ns | 10764ns | +3807.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 122ns | 109ns | 159ns | base | 2.104 |
| numeral_ratio_conversion_room | 177ns | 174ns | 181ns | +45.59% | 1.445 |
| numeral_ratio_conversion_wide | 8740ns | 8089ns | 10621ns | +7081.83% | 0.029 |

## Performance model

- Peak throughput: **2.352 Gops/s** (numeral_ratio_conversion_hand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_conversion_hand | 2.287 | 97.2% |
| numeral_ratio_conversion_room | 1.444 | 61.4% |
| numeral_ratio_conversion_wide | 0.031 | 1.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_conversion_hand | 227ns | 227ns | base |
| numeral_ratio_conversion_room | 276ns | 276ns | +22.03% |
| numeral_ratio_conversion_wide | 8853ns | 8853ns | +3807.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 112ns | base | --- | [111, 112] | --- | --- | --- | --- |
| numeral_ratio_conversion_room | 177ns | +64.9ns (+58.0%) | [+65, +65]ns | [176, 178] | YES | 0.0000 | 0.0000 | 0 |
| numeral_ratio_conversion_wide | 8356ns | +8240.3ns (+7360.7%) | [+8134, +8268]ns | [8272, 8390] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_conversion_hand | numeral_ratio_conversion_room | numeral_ratio_conversion_wide |
|---|---|---|---|---|
| 1 | 1 | 111ns | +58.2% | +7257.5% |
| 1 | 2 | 109ns | +59.7% | +7438.2% |
| 1 | 3 | 159ns | +11.6% | +5159.6% |
| 1 | 4 | 112ns | +59.3% | +7402.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_conversion_hand | -0.419 | moderate- |
| numeral_ratio_conversion_room | 0.040 | ok |
| numeral_ratio_conversion_wide | 0.301 | moderate+ |

**Consistency summary:**

- **numeral_ratio_conversion_room**: won 0/4, lost 4/4
- **numeral_ratio_conversion_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_conversion_hand | 37.2ns | 121.7ns | 30.6% | HIGH |
| numeral_ratio_conversion_room | 35.7ns | 177.2ns | 20.1% | HIGH |
| numeral_ratio_conversion_wide | 45.6ns | 8739.8ns | 0.5% |  |

## Distribution (algo ns)

```
numeral_ratio_conversion_hand (n=40, range 108.9-158.8 ns)
    108.9 |###############################
    111.4 |########################################
    113.9 |##
    116.4 |
    118.9 |
    121.3 |
    123.8 |##
    126.3 |
    128.8 |
    131.3 |
    133.8 |
    136.3 |
    138.8 |
    141.3 |
    143.8 |##
    146.3 |
    148.8 |
    151.3 |##
    153.8 |##
    156.3 |##
  (4 below, 5 above range)

numeral_ratio_conversion_room (n=40, range 173.9-181.1 ns)
    173.9 |###################################
    174.3 |
    174.6 |
    175.0 |
    175.3 |
    175.7 |##########
    176.1 |###################################
    176.4 |
    176.8 |
    177.2 |########################################
    177.5 |##########
    177.9 |#####
    178.2 |##############################
    178.6 |###############
    179.0 |
    179.3 |
    179.7 |#####
    180.1 |
    180.4 |
    180.8 |
  (2 below, 1 above range)

numeral_ratio_conversion_wide (n=40, range 8089.3-10620.8 ns)
   8089.3 |#########################
   8215.9 |###############
   8342.4 |########################################
   8469.0 |#####
   8595.6 |#####
   8722.2 |
   8848.7 |
   8975.3 |
   9101.9 |
   9228.5 |
   9355.0 |
   9481.6 |
   9608.2 |
   9734.8 |
   9861.3 |
   9987.9 |
  10114.5 |
  10241.1 |
  10367.6 |
  10494.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **numeral_ratio_conversion_hand**: bridge=32.0% of algo (FFI overhead may distort results)
- **numeral_ratio_conversion_room**: bridge=20.2% of algo (FFI overhead may distort results)
- **numeral_ratio_conversion_wide**: CV=24.8% (high variance, measurements may be unstable)
