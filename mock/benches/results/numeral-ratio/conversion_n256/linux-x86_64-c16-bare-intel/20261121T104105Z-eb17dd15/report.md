# A 16-bit fixed point at exponent -8 onto exponent -4, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_conversion_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_conversion_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_conversion_hand dominates: 107% faster than the next best (numeral_ratio_conversion_room)

numeral_ratio_conversion_hand (446 ns) leads numeral_ratio_conversion_room (921 ns) by 107%, a clear separation rather than a photo finish. CV 35.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_conversion_wide is an outlier: 18.7x slower than the field

numeral_ratio_conversion_wide (8.36 us) is 18.7x the fastest (446 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### numeral_ratio_conversion_hand is fastest but the noisiest (CV 35.2%)

numeral_ratio_conversion_hand wins on median (446 ns) yet has the highest variance (CV 35.2%), while numeral_ratio_conversion_wide is the steadiest (CV 8.6%, 8.36 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (numeral_ratio_conversion_hand)

The baseline numeral_ratio_conversion_hand is the fastest (446 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 18.7x the fastest

Fastest numeral_ratio_conversion_hand (446 ns) to slowest numeral_ratio_conversion_wide (8.36 us): 18.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### numeral_ratio_conversion_hand is inconsistent: worst-20% is 2.0x its best-20%

numeral_ratio_conversion_hand's best 20% of batches run at 370 ns but its worst 20% at 743 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (numeral_ratio_conversion_hand) is the fastest** at 445.8 ns median
- 2 variants significantly slower than baseline
- Spread: 18.74x (fastest 445.8 ns, slowest 8355.5 ns)

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
| Counter frequency | 2112734798 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 793ns | 699ns | 578ns | 752ns | 1131ns | base |
| numeral_ratio_conversion_room | 1226ns | 1203ns | 986ns | 1193ns | 1566ns | +54.60% |
| numeral_ratio_conversion_wide | 7974ns | 8421ns | 7103ns | 7979ns | 8832ns | +905.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 498ns | 370ns | 743ns | base | 0.514 |
| numeral_ratio_conversion_room | 946ns | 772ns | 1189ns | +90.01% | 0.271 |
| numeral_ratio_conversion_wide | 7902ns | 7049ns | 8738ns | +1487.33% | 0.032 |

## Performance model

- Peak throughput: **0.693 Gops/s** (numeral_ratio_conversion_hand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_conversion_hand | 0.574 | 82.9% |
| numeral_ratio_conversion_room | 0.278 | 40.1% |
| numeral_ratio_conversion_wide | 0.031 | 4.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_conversion_hand | 793ns | 793ns | base |
| numeral_ratio_conversion_room | 1226ns | 1226ns | +54.60% |
| numeral_ratio_conversion_wide | 7974ns | 7974ns | +905.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_conversion_hand | 446ns | base | --- | [402, 447] | --- | --- | --- | --- |
| numeral_ratio_conversion_room | 921ns | +408.3ns (+91.6%) | [+403, +479]ns | [849, 1057] | YES | 0.0000 | 0.0000 | 0 |
| numeral_ratio_conversion_wide | 8355ns | +7622.1ns (+1709.8%) | [+6834, +7776]ns | [7211, 8421] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_conversion_hand | numeral_ratio_conversion_room | numeral_ratio_conversion_wide |
|---|---|---|---|---|
| 1 | 1 | 667ns | +70.7% | +1160.0% |
| 1 | 2 | 364ns | +112.0% | +1876.7% |
| 1 | 3 | 447ns | +90.0% | +1472.1% |
| 1 | 4 | 402ns | +160.3% | +2043.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_conversion_hand | -0.304 | moderate- |
| numeral_ratio_conversion_room | -0.287 | moderate- |
| numeral_ratio_conversion_wide | -0.259 | moderate- |

**Consistency summary:**

- **numeral_ratio_conversion_room**: won 0/4, lost 4/4
- **numeral_ratio_conversion_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_conversion_hand | 98.2ns | 497.8ns | 19.7% | HIGH |
| numeral_ratio_conversion_room | 93.9ns | 946.0ns | 9.9% | HIGH |
| numeral_ratio_conversion_wide | 21.0ns | 7902.4ns | 0.3% |  |

## Distribution (algo ns)

```
numeral_ratio_conversion_hand (n=40, range 369.6-743.2 ns)
    369.6 |###
    388.3 |########################################
    407.0 |
    425.7 |###
    444.3 |#############################
    463.0 |
    481.7 |
    500.4 |###
    519.0 |
    537.7 |
    556.4 |###
    575.1 |
    593.7 |###
    612.4 |
    631.1 |###
    649.8 |##############
    668.5 |#######
    687.1 |###
    705.8 |
    724.5 |
  (6 below, 2 above range)

numeral_ratio_conversion_room (n=40, range 771.9-1188.7 ns)
    771.9 |########################################
    792.8 |
    813.6 |
    834.5 |###############################
    855.3 |
    876.1 |
    897.0 |####
    917.8 |
    938.7 |#################
    959.5 |
    980.3 |
   1001.2 |
   1022.0 |####
   1042.8 |###################################
   1063.7 |
   1084.5 |
   1105.4 |####
   1126.2 |
   1147.0 |####
   1167.9 |
  (3 below, 5 above range)

numeral_ratio_conversion_wide (n=40, range 7049.2-8738.2 ns)
   7049.2 |
   7133.6 |########################################
   7218.1 |#############
   7302.5 |
   7387.0 |
   7471.4 |
   7555.9 |
   7640.3 |
   7724.8 |
   7809.2 |
   7893.7 |####
   7978.2 |
   8062.6 |
   8147.1 |
   8231.5 |
   8316.0 |######################
   8400.4 |######################
   8484.9 |####
   8569.3 |##########################
   8653.8 |#############
  (6 below, 1 above range)

```

## Diagnostics

- **numeral_ratio_conversion_hand**: CV=31.5% (high variance, measurements may be unstable)
- **numeral_ratio_conversion_hand**: bridge=20.1% of algo (FFI overhead may distort results)
