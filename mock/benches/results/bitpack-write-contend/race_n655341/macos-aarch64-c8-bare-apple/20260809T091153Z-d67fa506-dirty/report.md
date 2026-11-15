# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 2440% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (8.70 us) leads bitpack-write-unsound (220.92 us) by 2440%, a clear separation rather than a photo finish. CV 14.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 27.0x slower than the field

bitpack-write-guarded (234.46 us) is 27.0x the fastest (8.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.87)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (8.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 27.0x the fastest

Fastest bitpack-write-dense (8.70 us) to slowest bitpack-write-guarded (234.46 us): 27.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-unsound is inconsistent: worst-20% is 1.6x its best-20%

bitpack-write-unsound's best 20% of batches run at 193.59 us but its worst 20% at 311.76 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 8696.2 ns median
- 2 variants significantly slower than baseline
- Spread: 26.96x (fastest 8696.2 ns, slowest 234464.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 9490ns | 8790ns | 8580ns | 9077ns | 11638ns | base |
| bitpack-write-guarded | 239426ns | 234903ns | 208122ns | 235507ns | 282487ns | +2422.98% |
| bitpack-write-unsound | 231655ns | 221220ns | 193895ns | 217320ns | 312419ns | +2341.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 9383ns | 8490ns | 11491ns | base | 6.984 |
| bitpack-write-guarded | 239010ns | 207831ns | 282037ns | +2447.28% | 0.274 |
| bitpack-write-unsound | 231261ns | 193585ns | 311757ns | +2364.69% | 0.283 |

## Performance model

- Peak throughput: **7.719 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 7.536 | 97.6% |
| bitpack-write-guarded | 0.280 | 3.6% |
| bitpack-write-unsound | 0.297 | 3.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 9490ns | 9490ns | base |
| bitpack-write-guarded | 239426ns | 239426ns | +2422.98% |
| bitpack-write-unsound | 231655ns | 231655ns | +2341.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 8696ns | base | --- | [8592, 9021] | --- | --- | --- | --- |
| bitpack-write-guarded | 234465ns | +225726.7ns (+2595.7%) | [+209391, +241061]ns | [217920, 249933] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 220915ns | +212139.5ns (+2439.5%) | [+188661, +225614]ns | [197161, 234649] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|
| 1 | 11240ns | +2436.1% | +2196.2% |
| 2 | 11220ns | +2520.7% | +3285.4% |
| 3 | 11382ns | +2390.2% | +4029.4% |
| 4 | 11351ns | +2408.8% | +2658.3% |
| 5 | 11244ns | +2355.6% | +2416.1% |
| 6 | 11392ns | +2244.0% | +2169.7% |
| 7 | 12834ns | +1954.8% | +2052.1% |
| 8 | 11238ns | +2384.2% | +2172.3% |
| 9 | 11245ns | +2423.6% | +2062.5% |
| 10 | 11228ns | +2304.1% | +2136.8% |
| 11 | 8983ns | +2544.6% | +2483.0% |
| 12 | 9065ns | +2558.8% | +2440.7% |
| 13 | 8990ns | +2850.0% | +2418.1% |
| 14 | 8972ns | +2710.4% | +2424.1% |
| 15 | 9072ns | +2598.3% | +2474.4% |
| 16 | 8691ns | +2793.2% | +2556.7% |
| 17 | 8671ns | +2710.6% | +2623.8% |
| 18 | 8630ns | +2842.8% | +2631.5% |
| 19 | 9052ns | +2644.2% | +2504.8% |
| 20 | 9669ns | +2426.3% | +2386.0% |
| 21 | 8562ns | +2457.5% | +2416.5% |
| 22 | 8484ns | +2581.2% | +2227.0% |
| 23 | 8493ns | +2624.1% | +2175.9% |
| 24 | 8490ns | +2362.1% | +2183.0% |
| 25 | 8492ns | +2548.7% | +2176.7% |
| 26 | 8485ns | +2578.8% | +2235.3% |
| 27 | 8498ns | +2406.8% | +2183.4% |
| 28 | 8512ns | +2391.1% | +2175.1% |
| 29 | 8516ns | +2368.5% | +2212.0% |
| 30 | 8496ns | +2452.6% | +2255.6% |
| 31 | 8485ns | +2486.9% | +2175.4% |
| 32 | 8497ns | +2374.7% | +2182.4% |
| 33 | 8522ns | +2326.1% | +2180.7% |
| 34 | 8549ns | +2315.1% | +2187.8% |
| 35 | 8622ns | +2294.4% | +2161.4% |
| 36 | 8668ns | +2318.9% | +2160.8% |
| 37 | 8720ns | +2316.1% | +2185.1% |
| 38 | 8701ns | +2379.3% | +2220.1% |
| 39 | 8633ns | +2295.6% | +2164.8% |
| 40 | 8722ns | +2276.1% | +2118.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.871 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.871 | HIGH+ (drift/warm-up) |
| bitpack-write-unsound | 0.771 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40
- **bitpack-write-unsound**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 3.3ns | 9383.0ns | 0.0% |  |
| bitpack-write-guarded | 10.8ns | 239010.3ns | 0.0% |  |
| bitpack-write-unsound | 15.5ns | 231260.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 8490.3-11490.6 ns)
   8490.3 |########################################
   8640.3 |##################
   8790.3 |
   8940.3 |##################
   9090.3 |
   9240.3 |
   9390.4 |
   9540.4 |###
   9690.4 |
   9840.4 |
   9990.4 |
  10140.4 |
  10290.5 |
  10440.5 |
  10590.5 |
  10740.5 |
  10890.5 |
  11040.5 |
  11190.5 |##################
  11340.6 |#########
  (4 below, 1 above range)

bitpack-write-guarded (n=40, range 207830.6-282036.7 ns)
  207830.6 |########################################
  211540.9 |################
  215251.2 |################
  218961.5 |################
  222671.8 |########
  226382.1 |################
  230092.4 |########
  233802.7 |
  237513.0 |################
  241223.3 |########################
  244933.6 |########
  248643.9 |################
  252354.2 |########
  256064.5 |
  259774.8 |
  263485.2 |########################
  267195.5 |########
  270905.8 |
  274616.1 |########
  278326.4 |########
  (5 below, 5 above range)

bitpack-write-unsound (n=40, range 193585.1-311756.9 ns)
  193585.1 |########################################
  199493.7 |######
  205402.3 |
  211310.9 |###
  217219.5 |
  223128.0 |######
  229036.6 |############
  234945.2 |############
  240853.8 |###
  246762.4 |###
  252671.0 |#########
  258579.6 |
  264488.2 |
  270396.7 |###
  276305.3 |
  282213.9 |###
  288122.5 |
  294031.1 |
  299939.7 |
  305848.3 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-write-guarded**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **bitpack-write-unsound**: CV=23.4% (high variance, measurements may be unstable)
- **bitpack-write-unsound**: autocorrelation=0.77 (measurement drift or warm-up artifact)
