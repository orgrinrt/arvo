# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 890% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (459.94 us) leads bitpack-write-guarded (4.55 ms) by 890%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (459.94 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.9x the fastest

Fastest bitpack-write-dense (459.94 us) to slowest bitpack-write-guarded (4.55 ms): 9.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 459935.9 ns median
- 1 variant significantly slower than baseline
- Spread: 9.90x (fastest 459935.9 ns, slowest 4552116.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 458497ns | 460223ns | 407521ns | 460681ns | 502923ns | base |
| bitpack-write-guarded | 4514455ns | 4552528ns | 3632554ns | 4503034ns | 5430621ns | +884.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 458188ns | 407180ns | 502571ns | base | 4.577 |
| bitpack-write-guarded | 4514034ns | 3632055ns | 5430218ns | +885.19% | 0.465 |

## Performance model

- Peak throughput: **5.150 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.560 | 88.5% |
| bitpack-write-guarded | 0.461 | 8.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 458497ns | 458497ns | base |
| bitpack-write-guarded | 4514455ns | 4514455ns | +884.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 459936ns | base | --- | [457970, 461860] | --- | --- | --- | --- |
| bitpack-write-guarded | 4552116ns | +4111321.8ns (+893.9%) | [+4031879, +4179882]ns | [4514437, 4661364] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 482641ns | +851.7% |
| 1 | 2 | 460079ns | +689.5% |
| 1 | 3 | 411817ns | +1021.7% |
| 1 | 4 | 458175ns | +925.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.109 | ok |
| bitpack-write-guarded | -0.340 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 80.3ns | 458187.6ns | 0.0% |  |
| bitpack-write-guarded | 66.4ns | 4514034.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 407179.7-502570.5 ns)
  407179.7 |####
  411949.2 |####
  416718.8 |
  421488.3 |########
  426257.9 |####
  431027.4 |####
  435796.9 |
  440566.5 |
  445336.0 |####
  450105.6 |
  454875.1 |###############################
  459644.6 |########################################
  464414.2 |
  469183.7 |####
  473953.3 |####
  478722.8 |###################################
  483492.3 |####
  488261.9 |
  493031.4 |
  497801.0 |
  (4 below, 2 above range)

bitpack-write-guarded (n=40, range 3632054.9-5430218.4 ns)
  3632054.9 |########################################
  3721963.1 |
  3811871.3 |
  3901779.4 |
  3991687.6 |
  4081595.8 |
  4171504.0 |
  4261412.1 |
  4351320.3 |#####
  4441228.5 |##################################
  4531136.7 |########################################
  4621044.8 |##################################
  4710953.0 |######################
  4800861.2 |#################
  4890769.4 |#####
  4980677.5 |#####
  5070585.7 |
  5160493.9 |
  5250402.1 |
  5340310.2 |
  (3 below, 1 above range)

```

## Diagnostics

- **bitpack-write-guarded**: CV=20.1% (high variance, measurements may be unstable)
