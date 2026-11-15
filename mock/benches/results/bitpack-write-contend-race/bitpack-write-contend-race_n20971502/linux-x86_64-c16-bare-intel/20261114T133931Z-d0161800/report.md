# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 897% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (437.30 us) leads bitpack-write-guarded (4.36 ms) by 897%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 9.4%)

bitpack-write-dense wins on median (437.30 us) yet has the highest variance (CV 9.4%), while bitpack-write-guarded is the steadiest (CV 4.7%, 4.36 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (437.30 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.0x the fastest

Fastest bitpack-write-dense (437.30 us) to slowest bitpack-write-guarded (4.36 ms): 10.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 437299.3 ns median
- 1 variant significantly slower than baseline
- Spread: 9.97x (fastest 437299.3 ns, slowest 4358102.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 435256ns | 437583ns | 377291ns | 436250ns | 490239ns | base |
| bitpack-write-guarded | 4428184ns | 4358537ns | 4307212ns | 4356169ns | 4765202ns | +917.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 434924ns | 376974ns | 489826ns | base | 4.822 |
| bitpack-write-guarded | 4427796ns | 4306893ns | 4764796ns | +918.06% | 0.474 |

## Performance model

- Peak throughput: **5.563 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.796 | 86.2% |
| bitpack-write-guarded | 0.481 | 8.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 435256ns | 435256ns | base |
| bitpack-write-guarded | 4428184ns | 4428184ns | +917.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 437299ns | base | --- | [436396, 442321] | --- | --- | --- | --- |
| bitpack-write-guarded | 4358102ns | +3920762.0ns (+896.6%) | [+3875301, +3965915]ns | [4323968, 4374003] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 376658ns | +1066.4% |
| 1 | 2 | 436866ns | +899.7% |
| 1 | 3 | 470466ns | +819.5% |
| 1 | 4 | 437500ns | +885.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.043 | ok |
| bitpack-write-guarded | 0.296 | moderate+ |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 82.1ns | 434924.4ns | 0.0% |  |
| bitpack-write-guarded | 84.0ns | 4427795.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 376973.9-489826.3 ns)
  376973.9 |##########
  382616.6 |##
  388259.2 |
  393901.8 |
  399544.4 |
  405187.0 |
  410829.7 |
  416472.3 |
  422114.9 |
  427757.5 |
  433400.1 |########################################
  439042.7 |##########
  444685.4 |##
  450328.0 |
  455970.6 |##
  461613.2 |
  467255.8 |#############
  472898.5 |
  478541.1 |##
  484183.7 |#####
  (5 below, 1 above range)

bitpack-write-guarded (n=40, range 4306892.8-4764795.7 ns)
  4306892.8 |########################################
  4329787.9 |###
  4352683.1 |########################################
  4375578.2 |##########
  4398473.4 |###
  4421368.5 |#######
  4444263.7 |###
  4467158.8 |
  4490053.9 |
  4512949.1 |
  4535844.2 |
  4558739.4 |
  4581634.5 |
  4604529.7 |
  4627424.8 |###
  4650320.0 |###
  4673215.1 |
  4696110.2 |
  4719005.4 |
  4741900.5 |
  (4 below, 4 above range)

```
