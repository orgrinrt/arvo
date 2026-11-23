# The quotient of a 16-bit fixed point by a value read at run time, 64 records

3 variants, 40 samples per variant.
Baseline: **numeral_ratio_quotient_hand**

## Highlights

Baseline for all deltas below: **numeral_ratio_quotient_hand**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### numeral_ratio_quotient_hand dominates: 761% faster than the next best (numeral_ratio_quotient_room)

numeral_ratio_quotient_hand (597 ns) leads numeral_ratio_quotient_room (5.14 us) by 761%, a clear separation rather than a photo finish. CV 15.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### numeral_ratio_quotient_wide is an outlier: 17.8x slower than the field

numeral_ratio_quotient_wide (10.61 us) is 17.8x the fastest (597 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (numeral_ratio_quotient_hand)

The baseline numeral_ratio_quotient_hand is the fastest (597 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 17.8x the fastest

Fastest numeral_ratio_quotient_hand (597 ns) to slowest numeral_ratio_quotient_wide (10.61 us): 17.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### numeral_ratio_quotient_room is inconsistent: worst-20% is 4.2x its best-20%

numeral_ratio_quotient_room's best 20% of batches run at 1.61 us but its worst 20% at 6.68 us (4.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (numeral_ratio_quotient_hand) is the fastest** at 597.3 ns median
- 2 variants significantly slower than baseline
- Spread: 17.77x (fastest 597.3 ns, slowest 10614.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The quotient of a 16-bit fixed point by a value read at run time, 64 records |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112774483 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| numeral_ratio_quotient_hand | 884ns | 876ns | 736ns | 867ns | 1082ns | base |
| numeral_ratio_quotient_room | 4701ns | 5333ns | 1665ns | 4956ns | 6975ns | +431.99% |
| numeral_ratio_quotient_wide | 10642ns | 10678ns | 10243ns | 10663ns | 10976ns | +1104.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| numeral_ratio_quotient_hand | 618ns | 518ns | 751ns | base | 0.414 |
| numeral_ratio_quotient_room | 4522ns | 1607ns | 6683ns | +631.79% | 0.057 |
| numeral_ratio_quotient_wide | 10579ns | 10177ns | 10916ns | +1611.77% | 0.024 |

## Performance model

- Peak throughput: **0.494 Gops/s** (numeral_ratio_quotient_hand; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| numeral_ratio_quotient_hand | 0.429 | 86.8% |
| numeral_ratio_quotient_room | 0.050 | 10.1% |
| numeral_ratio_quotient_wide | 0.024 | 4.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| numeral_ratio_quotient_hand | 884ns | 884ns | base |
| numeral_ratio_quotient_room | 4701ns | 4701ns | +431.99% |
| numeral_ratio_quotient_wide | 10642ns | 10642ns | +1104.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| numeral_ratio_quotient_hand | 597ns | base | --- | [573, 644] | --- | --- | --- | --- |
| numeral_ratio_quotient_room | 5143ns | +4525.3ns (+757.6%) | [+3724, +4980]ns | [4358, 5616] | YES | 0.0000 | 0.0000 | 0 |
| numeral_ratio_quotient_wide | 10614ns | +9948.0ns (+1665.5%) | [+9892, +10061]ns | [10462, 10752] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | numeral_ratio_quotient_hand | numeral_ratio_quotient_room | numeral_ratio_quotient_wide |
|---|---|---|---|---|
| 1 | 1 | 538ns | +857.4% | +1846.0% |
| 1 | 2 | 626ns | +620.9% | +1528.1% |
| 1 | 3 | 697ns | +733.1% | +1437.8% |
| 1 | 4 | 574ns | +849.0% | +1799.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| numeral_ratio_quotient_hand | -0.194 | ok |
| numeral_ratio_quotient_room | -0.260 | moderate- |
| numeral_ratio_quotient_wide | 0.102 | ok |

**Consistency summary:**

- **numeral_ratio_quotient_room**: won 0/4, lost 4/4
- **numeral_ratio_quotient_wide**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| numeral_ratio_quotient_hand | 97.1ns | 618.0ns | 15.7% | HIGH |
| numeral_ratio_quotient_room | 57.9ns | 4522.5ns | 1.3% |  |
| numeral_ratio_quotient_wide | 19.0ns | 10578.7ns | 0.2% |  |

## Distribution (algo ns)

```
numeral_ratio_quotient_hand (n=40, range 518.4-750.5 ns)
    518.4 |#####
    530.0 |#####
    541.6 |#####
    553.2 |######################
    564.8 |########################################
    576.4 |#####
    588.1 |
    599.7 |
    611.3 |#####
    622.9 |######################
    634.5 |#####
    646.1 |#################
    657.7 |#####
    669.3 |#####
    680.9 |
    692.5 |##################################
    704.1 |
    715.7 |#####
    727.3 |
    738.9 |
  (5 below, 2 above range)

numeral_ratio_quotient_room (n=40, range 1606.7-6683.0 ns)
   1606.7 |########
   1860.5 |########################
   2114.3 |########
   2368.1 |
   2622.0 |
   2875.8 |
   3129.6 |
   3383.4 |
   3637.2 |
   3891.0 |########
   4144.9 |################
   4398.7 |################
   4652.5 |################
   4906.3 |################
   5160.1 |########################################
   5413.9 |################
   5667.8 |########
   5921.6 |########################################
   6175.4 |########
   6429.2 |################
  (6 below, 4 above range)

numeral_ratio_quotient_wide (n=40, range 10177.2-10916.4 ns)
  10177.2 |##########
  10214.1 |##########
  10251.1 |####################
  10288.1 |
  10325.0 |
  10362.0 |####################
  10398.9 |##########
  10435.9 |####################
  10472.9 |####################
  10509.8 |##########
  10546.8 |##########
  10583.8 |####################
  10620.7 |####################
  10657.7 |##########
  10694.6 |##########
  10731.6 |########################################
  10768.6 |##############################
  10805.5 |##########
  10842.5 |####################
  10879.5 |####################
  (5 below, 4 above range)

```

## Diagnostics

- **numeral_ratio_quotient_hand**: bridge=14.3% of algo (FFI overhead may distort results)
- **numeral_ratio_quotient_room**: CV=41.7% (high variance, measurements may be unstable)
- **numeral_ratio_quotient_room**: worst_20/best_20 = 4.2x (possible bimodal distribution)
