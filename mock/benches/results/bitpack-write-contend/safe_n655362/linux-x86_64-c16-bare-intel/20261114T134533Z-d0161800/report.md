# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (170.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 21.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 471% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (21.35 us) leads bitpack-write-windowed (121.82 us) by 471%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 85% (significant)

bitpack-write-dense is -144.44 us (85%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 8.0x slower than the field

bitpack-write-aligned (170.25 us) is 8.0x the fastest (21.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-windowed shows alternating (throttle bounce) (autocorr -0.65)

bitpack-write-windowed's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.0x the fastest

Fastest bitpack-write-dense (21.35 us) to slowest bitpack-write-aligned (170.25 us): 8.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-windowed is inconsistent: worst-20% is 1.8x its best-20%

bitpack-write-windowed's best 20% of batches run at 108.05 us but its worst 20% at 195.54 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 21352.0 ns median (-87.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.97x (fastest 21352.0 ns, slowest 170249.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 167504ns | 170336ns | 152457ns | 166656ns | 185096ns | base |
| bitpack-write-dense | 22103ns | 21433ns | 19862ns | 21639ns | 25738ns | -86.80% |
| bitpack-write-windowed | 134365ns | 121910ns | 108126ns | 122676ns | 195673ns | -19.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 167413ns | 152369ns | 184990ns | base | 0.391 |
| bitpack-write-dense | 22020ns | 19778ns | 25654ns | -86.85% | 2.976 |
| bitpack-write-windowed | 134274ns | 108049ns | 195538ns | -19.79% | 0.488 |

## Performance model

- Peak throughput: **3.314 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.385 | 11.6% |
| bitpack-write-dense | 3.069 | 92.6% |
| bitpack-write-windowed | 0.538 | 16.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 167504ns | 167504ns | base |
| bitpack-write-dense | 22103ns | 22103ns | -86.80% |
| bitpack-write-windowed | 134365ns | 134365ns | -19.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 170249ns | base | --- | [160342, 170540] | --- | --- | --- | --- |
| bitpack-write-dense | 21352ns | -144438.5ns (-84.8%) | [-145050, -137971]ns | [20757, 22147] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 121824ns | -44936.6ns (-26.4%) | [-50980, -43999]ns | [119371, 128827] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 152492ns | -86.8% | -29.0% |
| 1 | 2 | 160388ns | -86.2% | -7.8% |
| 1 | 3 | 170437ns | -85.0% | -29.9% |
| 1 | 4 | 184948ns | -88.8% | -30.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.230 | moderate+ |
| bitpack-write-dense | -0.258 | moderate- |
| bitpack-write-windowed | -0.653 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 21.5ns | 167412.6ns | 0.0% |  |
| bitpack-write-dense | 20.5ns | 22019.6ns | 0.1% |  |
| bitpack-write-windowed | 25.2ns | 134273.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 152369.2-184990.3 ns)
  152369.2 |##############################
  154000.2 |
  155631.3 |
  157262.3 |
  158893.4 |########################################
  160524.5 |#####
  162155.5 |
  163786.6 |
  165417.6 |
  167048.7 |
  168679.7 |##########
  170310.8 |########################################
  171941.9 |
  173572.9 |#####
  175204.0 |
  176835.0 |
  178466.1 |
  180097.1 |
  181728.2 |
  183359.3 |##############################
  (4 below, 4 above range)

bitpack-write-dense (n=40, range 19778.4-25654.4 ns)
  19778.4 |########################################
  20072.2 |####################
  20366.0 |#############
  20659.8 |#################################
  20953.6 |####################
  21247.4 |######
  21541.2 |######
  21835.0 |####################
  22128.8 |#################################
  22422.6 |
  22716.4 |######
  23010.2 |
  23304.0 |
  23597.8 |
  23891.6 |
  24185.4 |
  24479.2 |
  24773.0 |######
  25066.8 |
  25360.6 |#################################
  (1 below, 3 above range)

bitpack-write-windowed (n=40, range 108049.4-195537.7 ns)
  108049.4 |############################
  112423.9 |
  116798.3 |########################################
  121172.7 |############
  125547.1 |####################################
  129921.5 |
  134295.9 |####
  138670.3 |####
  143044.7 |
  147419.2 |
  151793.6 |
  156168.0 |####
  160542.4 |
  164916.8 |
  169291.2 |
  173665.6 |####
  178040.1 |
  182414.5 |
  186788.9 |
  191163.3 |
  (3 below, 4 above range)

```

## Diagnostics

- **bitpack-write-windowed**: CV=28.8% (high variance, measurements may be unstable)
