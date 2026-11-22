# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 765% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (467.19 us) leads bitpack-write-guarded (4.04 ms) by 765%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded shows alternating (throttle bounce) (autocorr -0.76)

bitpack-write-guarded's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (467.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.6x the fastest

Fastest bitpack-write-dense (467.19 us) to slowest bitpack-write-guarded (4.04 ms): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 467190.1 ns median
- 1 variant significantly slower than baseline
- Spread: 8.65x (fastest 467190.1 ns, slowest 4039947.1 ns)

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
| Counter frequency | 2112761742 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 447300ns | 467487ns | 364039ns | 461382ns | 488312ns | base |
| bitpack-write-guarded | 4228014ns | 4040408ns | 3632832ns | 4101714ns | 5202096ns | +845.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 446999ns | 363742ns | 487984ns | base | 4.692 |
| bitpack-write-guarded | 4227602ns | 3632409ns | 5201674ns | +845.77% | 0.496 |

## Performance model

- Peak throughput: **5.765 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.489 | 77.9% |
| bitpack-write-guarded | 0.519 | 9.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 447300ns | 447300ns | base |
| bitpack-write-guarded | 4228014ns | 4228014ns | +845.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 467190ns | base | --- | [440862, 477768] | --- | --- | --- | --- |
| bitpack-write-guarded | 4039947ns | +3558086.9ns (+761.6%) | [+3191450, +4155562]ns | [3634226, 4536466] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 361255ns | +1183.2% |
| 1 | 2 | 467587ns | +677.2% |
| 1 | 3 | 486711ns | +857.1% |
| 1 | 4 | 446550ns | +713.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.067 | ok |
| bitpack-write-guarded | -0.755 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 73.4ns | 446999.1ns | 0.0% |  |
| bitpack-write-guarded | 52.9ns | 4227601.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 363741.5-487983.6 ns)
  363741.5 |
  369953.6 |
  376165.7 |####
  382377.8 |####
  388589.9 |
  394802.0 |
  401014.2 |
  407226.3 |
  413438.4 |
  419650.5 |
  425862.6 |
  432074.7 |####
  438286.8 |####################
  444498.9 |
  450711.0 |####
  456923.1 |####
  463135.2 |############################
  469347.3 |
  475559.4 |############
  481771.5 |########################################
  (7 below, 3 above range)

bitpack-write-guarded (n=40, range 3632408.8-5201673.9 ns)
  3632408.8 |########################################
  3710872.1 |
  3789335.3 |
  3867798.6 |
  3946261.8 |
  4024725.1 |
  4103188.3 |
  4181651.6 |
  4260114.8 |
  4338578.1 |##
  4417041.4 |
  4495504.6 |################
  4573967.9 |#######
  4652431.1 |###########
  4730894.4 |####
  4809357.6 |##
  4887820.9 |
  4966284.1 |
  5044747.4 |
  5123210.6 |
  (3 below, 1 above range)

```
