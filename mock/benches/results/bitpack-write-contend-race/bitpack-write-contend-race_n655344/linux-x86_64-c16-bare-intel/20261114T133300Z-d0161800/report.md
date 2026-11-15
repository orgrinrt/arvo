# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 443% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (28.93 us) leads bitpack-write-guarded (157.20 us) by 443%, a clear separation rather than a photo finish. CV 18.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 18.1%)

bitpack-write-dense wins on median (28.93 us) yet has the highest variance (CV 18.1%), while bitpack-write-guarded is the steadiest (CV 12.8%, 157.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-guarded shows alternating (throttle bounce) (autocorr -0.79)

bitpack-write-guarded's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (28.93 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.4x the fastest

Fastest bitpack-write-dense (28.93 us) to slowest bitpack-write-guarded (157.20 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 1.8x its best-20%

bitpack-write-dense's best 20% of batches run at 17.57 us but its worst 20% at 31.14 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 28925.1 ns median
- 1 variant significantly slower than baseline
- Spread: 5.43x (fastest 28925.1 ns, slowest 157200.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 26752ns | 29004ns | 17710ns | 28273ns | 31229ns | base |
| bitpack-write-guarded | 160875ns | 157317ns | 136309ns | 159606ns | 189246ns | +501.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 26650ns | 17567ns | 31136ns | base | 2.459 |
| bitpack-write-guarded | 160746ns | 136190ns | 189112ns | +503.17% | 0.408 |

## Performance model

- Peak throughput: **3.731 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 2.266 | 60.7% |
| bitpack-write-guarded | 0.417 | 11.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 26752ns | 26752ns | base |
| bitpack-write-guarded | 160875ns | 160875ns | +501.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 28925ns | base | --- | [28254, 29673] | --- | --- | --- | --- |
| bitpack-write-guarded | 157200ns | +132141.0ns (+456.8%) | [+129232, +142254]ns | [149570, 171934] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 29817ns | +476.3% |
| 1 | 2 | 17311ns | +764.0% |
| 1 | 3 | 29768ns | +520.0% |
| 1 | 4 | 28290ns | +434.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.512 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.789 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 30.1ns | 26650.1ns | 0.1% |  |
| bitpack-write-guarded | 30.4ns | 160745.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 17567.0-31135.7 ns)
  17567.0 |
  18245.4 |############
  18923.8 |####
  19602.3 |
  20280.7 |
  20959.1 |
  21637.6 |
  22316.0 |
  22994.4 |
  23672.9 |
  24351.3 |
  25029.7 |
  25708.2 |
  26386.6 |
  27065.0 |
  27743.5 |########################################
  28421.9 |
  29100.3 |####################################
  29778.8 |########################
  30457.2 |########
  (6 below, 3 above range)

bitpack-write-guarded (n=40, range 136190.0-189111.8 ns)
  136190.0 |##########################
  138836.1 |####################
  141482.2 |
  144128.2 |#############
  146774.3 |######
  149420.4 |########################################
  152066.5 |
  154712.6 |######
  157358.7 |##########################
  160004.8 |
  162650.9 |
  165297.0 |
  167943.1 |
  170589.2 |##########################
  173235.3 |
  175881.3 |
  178527.4 |######
  181173.5 |######
  183819.6 |########################################
  186465.7 |
  (3 below, 4 above range)

```
