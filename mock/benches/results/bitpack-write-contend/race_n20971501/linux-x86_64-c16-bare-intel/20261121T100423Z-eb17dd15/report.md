# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1218% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (485.21 us) leads bitpack-write-unsound (6.40 ms) by 1218%, a clear separation rather than a photo finish. CV 11.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 15.1x slower than the field

bitpack-write-guarded (7.31 ms) is 15.1x the fastest (485.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 11.0%)

bitpack-write-dense wins on median (485.21 us) yet has the highest variance (CV 11.0%), while bitpack-write-unsound is the steadiest (CV 1.3%, 6.40 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-guarded shows alternating (throttle bounce) (autocorr -0.78)

bitpack-write-guarded's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (485.21 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.1x the fastest

Fastest bitpack-write-dense (485.21 us) to slowest bitpack-write-guarded (7.31 ms): 15.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 485210.3 ns median
- 2 variants significantly slower than baseline
- Spread: 15.08x (fastest 485210.3 ns, slowest 7314550.3 ns)

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
| bitpack-write-dense | 489282ns | 485496ns | 434399ns | 479376ns | 573885ns | base |
| bitpack-write-guarded | 7262507ns | 7314801ns | 7110189ns | 7262848ns | 7413804ns | +1384.32% |
| bitpack-write-unsound | 6376281ns | 6396916ns | 6263484ns | 6377490ns | 6485451ns | +1203.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 488951ns | 434085ns | 573378ns | base | 4.289 |
| bitpack-write-guarded | 7262157ns | 7109752ns | 7413425ns | +1385.25% | 0.289 |
| bitpack-write-unsound | 6375908ns | 6263112ns | 6485218ns | +1204.00% | 0.329 |

## Performance model

- Peak throughput: **4.831 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.322 | 89.5% |
| bitpack-write-guarded | 0.287 | 5.9% |
| bitpack-write-unsound | 0.328 | 6.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 489282ns | 489282ns | base |
| bitpack-write-guarded | 7262507ns | 7262507ns | +1384.32% |
| bitpack-write-unsound | 6376281ns | 6376281ns | +1203.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 485210ns | base | --- | [446781, 500129] | --- | --- | --- | --- |
| bitpack-write-guarded | 7314550ns | +6835306.6ns (+1408.7%) | [+6676763, +6882281]ns | [7124992, 7360070] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 6396471ns | +5860038.2ns (+1207.7%) | [+5831283, +5936590]ns | [6315839, 6430296] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 433817ns | +1591.8% | +1344.5% |
| 1 | 2 | 445126ns | +1500.0% | +1354.0% |
| 1 | 3 | 500047ns | +1380.7% | +1185.6% |
| 1 | 4 | 556379ns | +1177.9% | +1044.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.261 | moderate+ |
| bitpack-write-guarded | -0.779 | HIGH- (thermal bounce) |
| bitpack-write-unsound | -0.298 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 55.7ns | 488950.7ns | 0.0% |  |
| bitpack-write-guarded | 66.8ns | 7262156.7ns | 0.0% |  |
| bitpack-write-unsound | 67.1ns | 6375907.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 434084.6-573377.8 ns)
  434084.6 |#################
  441049.2 |##########################
  448013.9 |#############
  454978.6 |
  461943.2 |####
  468907.9 |####
  475872.5 |
  482837.2 |
  489801.9 |
  496766.5 |########################################
  503731.2 |####
  510695.8 |
  517660.5 |
  524625.1 |
  531589.8 |
  538554.5 |
  545519.1 |#################
  552483.8 |########
  559448.4 |
  566413.1 |####
  (5 below, 3 above range)

bitpack-write-guarded (n=40, range 7109751.6-7413425.3 ns)
  7109751.6 |########################################
  7124935.3 |#############
  7140119.0 |
  7155302.7 |
  7170486.4 |
  7185670.1 |
  7200853.7 |
  7216037.4 |
  7231221.1 |
  7246404.8 |
  7261588.5 |
  7276772.2 |########
  7291955.9 |
  7307139.5 |########
  7322323.2 |########
  7337506.9 |########
  7352690.6 |########
  7367874.3 |########
  7383058.0 |#################
  7398241.7 |#############
  (5 below, 4 above range)

bitpack-write-unsound (n=40, range 6263111.7-6485218.2 ns)
  6263111.7 |########################################
  6274217.0 |
  6285322.3 |####
  6296427.6 |
  6307533.0 |########
  6318638.3 |
  6329743.6 |
  6340848.9 |
  6351954.3 |####
  6363059.6 |
  6374164.9 |########
  6385270.2 |########
  6396375.6 |####
  6407480.9 |#############
  6418586.2 |########
  6429691.5 |########
  6440796.9 |#################
  6451902.2 |####
  6463007.5 |########
  6474112.8 |####
  (3 below, 4 above range)

```
