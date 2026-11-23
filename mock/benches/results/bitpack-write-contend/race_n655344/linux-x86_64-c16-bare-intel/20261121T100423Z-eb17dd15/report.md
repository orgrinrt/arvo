# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 480% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (29.78 us) leads bitpack-write-guarded (172.56 us) by 480%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.68)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (29.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.8x the fastest

Fastest bitpack-write-dense (29.78 us) to slowest bitpack-write-guarded (172.56 us): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 29777.1 ns median
- 1 variant significantly slower than baseline
- Spread: 5.80x (fastest 29777.1 ns, slowest 172558.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef2 |
| Counter frequency | 2112756716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 30104ns | 29882ns | 28171ns | 29667ns | 33344ns | base |
| bitpack-write-guarded | 168340ns | 172658ns | 137639ns | 173349ns | 184015ns | +459.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 30012ns | 28094ns | 33230ns | base | 2.184 |
| bitpack-write-guarded | 168234ns | 137545ns | 183906ns | +460.56% | 0.390 |

## Performance model

- Peak throughput: **2.333 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 2.201 | 94.3% |
| bitpack-write-guarded | 0.380 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 30104ns | 30104ns | base |
| bitpack-write-guarded | 168340ns | 168340ns | +459.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 29777ns | base | --- | [28437, 30143] | --- | --- | --- | --- |
| bitpack-write-guarded | 172558ns | +144009.4ns (+483.6%) | [+139351, +146403]ns | [172428, 177022] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 30097ns | +353.1% |
| 1 | 2 | 28299ns | +548.7% |
| 1 | 3 | 33228ns | +420.3% |
| 1 | 4 | 28353ns | +508.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.684 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.285 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 23.8ns | 30011.6ns | 0.1% |  |
| bitpack-write-guarded | 24.1ns | 168234.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 28093.5-33229.9 ns)
  28093.5 |##################################
  28350.3 |########################################
  28607.2 |
  28864.0 |
  29120.8 |#####
  29377.6 |
  29634.4 |#####
  29891.3 |##################################
  30148.1 |######################
  30404.9 |
  30661.7 |
  30918.5 |
  31175.4 |
  31432.2 |
  31689.0 |
  31945.8 |
  32202.6 |
  32459.5 |
  32716.3 |#####
  32973.1 |######################
  (5 below, 5 above range)

bitpack-write-guarded (n=40, range 137545.4-183905.7 ns)
  137545.4 |
  139863.4 |##
  142181.4 |
  144499.4 |#####
  146817.5 |
  149135.5 |
  151453.5 |##
  153771.5 |
  156089.5 |
  158407.5 |
  160725.5 |
  163043.5 |
  165361.6 |
  167679.6 |
  169997.6 |#####
  172315.6 |########################################
  174633.6 |
  176951.6 |
  179269.6 |##
  181587.6 |###############################
  (6 below, 2 above range)

```
