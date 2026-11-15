# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1305% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (446.50 us) leads bitpack-write-unsound (6.27 ms) by 1305%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 15.9x slower than the field

bitpack-write-guarded (7.11 ms) is 15.9x the fastest (446.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 7.0%)

bitpack-write-dense wins on median (446.50 us) yet has the highest variance (CV 7.0%), while bitpack-write-unsound is the steadiest (CV 0.2%, 6.27 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-unsound shows alternating (throttle bounce) (autocorr -0.71)

bitpack-write-unsound's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (446.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.9x the fastest

Fastest bitpack-write-dense (446.50 us) to slowest bitpack-write-guarded (7.11 ms): 15.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 446495.8 ns median
- 2 variants significantly slower than baseline
- Spread: 15.93x (fastest 446495.8 ns, slowest 7113230.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 453194ns | 446782ns | 420645ns | 448249ns | 500577ns | base |
| bitpack-write-guarded | 7118980ns | 7113546ns | 7102431ns | 7114206ns | 7149850ns | +1470.85% |
| bitpack-write-unsound | 6275018ns | 6274271ns | 6259479ns | 6273874ns | 6293991ns | +1284.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 452932ns | 420414ns | 500350ns | base | 4.630 |
| bitpack-write-guarded | 7118659ns | 7101996ns | 7149549ns | +1471.69% | 0.295 |
| bitpack-write-unsound | 6274723ns | 6259120ns | 6293697ns | +1285.36% | 0.334 |

## Performance model

- Peak throughput: **4.988 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.697 | 94.2% |
| bitpack-write-guarded | 0.295 | 5.9% |
| bitpack-write-unsound | 0.334 | 6.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 453194ns | 453194ns | base |
| bitpack-write-guarded | 7118980ns | 7118980ns | +1470.85% |
| bitpack-write-unsound | 6275018ns | 6275018ns | +1284.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 446496ns | base | --- | [427737, 452145] | --- | --- | --- | --- |
| bitpack-write-guarded | 7113230ns | +6665876.4ns (+1492.9%) | [+6649585, +6687119]ns | [7109252, 7116567] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 6273933ns | +5827490.2ns (+1305.2%) | [+5823386, +5835000]ns | [6270767, 6276752] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 426402ns | +1567.7% | +1370.1% |
| 1 | 2 | 499590ns | +1323.9% | +1156.1% |
| 1 | 3 | 423073ns | +1584.1% | +1379.9% |
| 1 | 4 | 452045ns | +1471.4% | +1289.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.685 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.469 | moderate- |
| bitpack-write-unsound | -0.712 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 52.5ns | 452931.6ns | 0.0% |  |
| bitpack-write-guarded | 92.4ns | 7118658.7ns | 0.0% |  |
| bitpack-write-unsound | 57.1ns | 6274723.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 420414.2-500349.8 ns)
  420414.2 |##########################
  424411.0 |##########################
  428407.7 |#############
  432404.5 |
  436401.3 |########
  440398.1 |####
  444394.9 |
  448391.6 |###############################
  452388.4 |####
  456385.2 |
  460382.0 |
  464378.7 |
  468375.5 |
  472372.3 |
  476369.1 |
  480365.9 |####
  484362.6 |####
  488359.4 |
  492356.2 |
  496353.0 |########################################
  (2 below, 1 above range)

bitpack-write-guarded (n=40, range 7101996.4-7149548.5 ns)
  7101996.4 |####################
  7104374.0 |##########################
  7106751.6 |#############
  7109129.2 |########################################
  7111506.8 |####################
  7113884.4 |####################
  7116262.0 |####################
  7118639.7 |#############
  7121017.3 |####################
  7123394.9 |#############
  7125772.5 |#############
  7128150.1 |#############
  7130527.7 |######
  7132905.3 |
  7135282.9 |
  7137660.5 |
  7140038.1 |
  7142415.7 |
  7144793.3 |
  7147170.9 |
  (3 below, 1 above range)

bitpack-write-unsound (n=40, range 6259120.5-6293696.8 ns)
  6259120.5 |
  6260849.3 |
  6262578.1 |
  6264307.0 |################
  6266035.8 |########################################
  6267764.6 |################
  6269493.4 |
  6271222.2 |################
  6272951.0 |########################################
  6274679.9 |########################################
  6276408.7 |################
  6278137.5 |########################################
  6279866.3 |########
  6281595.1 |########
  6283323.9 |########
  6285052.8 |########
  6286781.6 |
  6288510.4 |########
  6290239.2 |
  6291968.0 |
  (5 below, 2 above range)

```
