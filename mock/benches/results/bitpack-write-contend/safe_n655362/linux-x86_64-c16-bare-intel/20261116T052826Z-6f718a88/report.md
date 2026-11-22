# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (169.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 15.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 752% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (15.76 us) leads bitpack-write-windowed (134.28 us) by 752%, a clear separation rather than a photo finish. CV 38.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 86% (significant)

bitpack-write-dense is -146.12 us (86%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 10.8x slower than the field

bitpack-write-aligned (169.90 us) is 10.8x the fastest (15.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 38.1%)

bitpack-write-dense wins on median (15.76 us) yet has the highest variance (CV 38.1%), while bitpack-write-aligned is the steadiest (CV 5.9%, 169.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-aligned shows alternating (throttle bounce) (autocorr -0.64)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.8x the fastest

Fastest bitpack-write-dense (15.76 us) to slowest bitpack-write-aligned (169.90 us): 10.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 2.3x its best-20%

bitpack-write-dense's best 20% of batches run at 11.49 us but its worst 20% at 25.86 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 15761.8 ns median (-90.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 10.78x (fastest 15761.8 ns, slowest 169905.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112697486 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 162328ns | 170010ns | 149700ns | 162818ns | 173483ns | base |
| bitpack-write-dense | 17281ns | 15856ns | 11570ns | 16297ns | 25944ns | -89.35% |
| bitpack-write-windowed | 129347ns | 134452ns | 108656ns | 132555ns | 140416ns | -20.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 162222ns | 149590ns | 173368ns | base | 0.404 |
| bitpack-write-dense | 17196ns | 11488ns | 25864ns | -89.40% | 3.811 |
| bitpack-write-windowed | 129262ns | 108585ns | 140319ns | -20.32% | 0.507 |

## Performance model

- Peak throughput: **5.705 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.386 | 6.8% |
| bitpack-write-dense | 4.158 | 72.9% |
| bitpack-write-windowed | 0.488 | 8.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 162328ns | 162328ns | base |
| bitpack-write-dense | 17281ns | 17281ns | -89.35% |
| bitpack-write-windowed | 129347ns | 129347ns | -20.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 169905ns | base | --- | [153790, 170212] | --- | --- | --- | --- |
| bitpack-write-dense | 15762ns | -146117.9ns (-86.0%) | [-147265, -138034]ns | [11546, 19998] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 134282ns | -35457.9ns (-20.9%) | [-41325, -30288]ns | [128956, 139213] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 172442ns | -85.1% | -25.2% |
| 1 | 2 | 153790ns | -87.0% | -9.4% |
| 1 | 3 | 170101ns | -93.2% | -18.0% |
| 1 | 4 | 149816ns | -92.3% | -27.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.645 | HIGH- (thermal bounce) |
| bitpack-write-dense | 0.277 | moderate+ |
| bitpack-write-windowed | -0.169 | ok |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 22.5ns | 162221.7ns | 0.0% |  |
| bitpack-write-dense | 20.5ns | 17195.9ns | 0.1% |  |
| bitpack-write-windowed | 21.7ns | 129261.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 149590.0-173367.8 ns)
  149590.0 |####################
  150778.9 |
  151967.8 |
  153156.7 |########################################
  154345.6 |
  155534.5 |
  156723.4 |
  157912.2 |
  159101.1 |
  160290.0 |
  161478.9 |
  162667.8 |
  163856.7 |
  165045.6 |
  166234.5 |
  167423.4 |
  168612.3 |
  169801.1 |########################################
  170990.0 |####
  172178.9 |############################
  (4 below, 3 above range)

bitpack-write-dense (n=40, range 11488.0-25863.5 ns)
  11488.0 |########################################
  12206.8 |
  12925.6 |
  13644.3 |
  14363.1 |
  15081.9 |
  15800.7 |
  16519.4 |
  17238.2 |
  17957.0 |
  18675.8 |
  19394.5 |####################
  20113.3 |#####
  20832.1 |
  21550.9 |
  22269.6 |
  22988.4 |
  23707.2 |
  24426.0 |##
  25144.7 |############
  (4 below, 4 above range)

bitpack-write-windowed (n=40, range 108585.5-140318.8 ns)
  108585.5 |##############
  110172.1 |
  111758.8 |
  113345.5 |
  114932.1 |
  116518.8 |
  118105.4 |
  119692.1 |
  121278.8 |
  122865.4 |
  124452.1 |
  126038.8 |
  127625.4 |################
  129212.1 |#######
  130798.8 |
  132385.4 |
  133972.1 |
  135558.8 |
  137145.4 |
  138732.1 |########################################
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=35.0% (high variance, measurements may be unstable)
