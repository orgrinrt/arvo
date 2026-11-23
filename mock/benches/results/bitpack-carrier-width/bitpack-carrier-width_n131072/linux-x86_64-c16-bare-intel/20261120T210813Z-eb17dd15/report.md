# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 dominates: 22% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-d64 (12.29 us) leads bitpack-carrier-d32 (15.01 us) by 22%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d64 beats baseline by 38% (significant)

bitpack-carrier-d64 is -7.65 us (38%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-packed shows alternating (throttle bounce) (autocorr -0.76)

bitpack-carrier-packed's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-carrier-d16-control's edge over baseline is significant but tiny (5 ns, 0.02%)

bitpack-carrier-d16-control differs from baseline bitpack-carrier-d16 by 5 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-carrier-d64** at 12286.1 ns median (-38.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.62x (fastest 12286.1 ns, slowest 19944.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112762950 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 20012ns | 20007ns | 20005ns | 20007ns | 20033ns | base |
| bitpack-carrier-d16-control | 21158ns | 20017ns | 20007ns | 20436ns | 24474ns | +5.73% |
| bitpack-carrier-d32 | 15088ns | 15088ns | 15070ns | 15087ns | 15108ns | -24.61% |
| bitpack-carrier-d64 | 12257ns | 12385ns | 11787ns | 12340ns | 12476ns | -38.75% |
| bitpack-carrier-packed | 17931ns | 17972ns | 16027ns | 17948ns | 19786ns | -10.40% |
| bitpack-carrier-packed-simd | 17033ns | 16137ns | 16104ns | 16446ns | 19724ns | -14.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 19941ns | 19939ns | 19944ns | base | 6.573 |
| bitpack-carrier-d16-control | 21081ns | 19939ns | 24384ns | +5.72% | 6.217 |
| bitpack-carrier-d32 | 15014ns | 14998ns | 15023ns | -24.71% | 8.730 |
| bitpack-carrier-d64 | 12151ns | 11669ns | 12376ns | -39.06% | 10.787 |
| bitpack-carrier-packed | 17852ns | 15959ns | 19694ns | -10.47% | 7.342 |
| bitpack-carrier-packed-simd | 16954ns | 16031ns | 19631ns | -14.98% | 7.731 |

## Performance model

- Peak throughput: **11.233 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 131072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.573 | 58.5% |
| bitpack-carrier-d16-control | 6.572 | 58.5% |
| bitpack-carrier-d32 | 8.731 | 77.7% |
| bitpack-carrier-d64 | 10.668 | 95.0% |
| bitpack-carrier-packed | 7.332 | 65.3% |
| bitpack-carrier-packed-simd | 8.159 | 72.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 20012ns | 20012ns | base |
| bitpack-carrier-d16-control | 21158ns | 21158ns | +5.73% |
| bitpack-carrier-d32 | 15088ns | 15088ns | -24.61% |
| bitpack-carrier-d64 | 12257ns | 12257ns | -38.75% |
| bitpack-carrier-packed | 17931ns | 17931ns | -10.40% |
| bitpack-carrier-packed-simd | 17033ns | 17033ns | -14.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 19940ns | base | --- | [19940, 19940] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 19945ns | +5.0ns (+0.0%) | [+3, +12]ns | [19943, 19952] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d32 | 15013ns | -4926.6ns (-24.7%) | [-4927, -4920]ns | [15012, 15022] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 12286ns | -7653.1ns (-38.4%) | [-7706, -7618]ns | [12234, 12322] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 17878ns | -2061.3ns (-10.3%) | [-3853, -271]ns | [16087, 19670] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 16064ns | -3875.4ns (-19.4%) | [-3886, -3864]ns | [16053, 16079] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 19940ns | +0.0% | -24.7% | -38.0% | -19.3% | -19.4% |
| 1 | 2 | 19939ns | +0.0% | -24.7% | -38.6% | -1.3% | -19.6% |
| 1 | 3 | 19940ns | +0.0% | -24.7% | -38.2% | -20.0% | -19.5% |
| 1 | 4 | 19940ns | +22.3% | -24.7% | -41.4% | -1.3% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.035 | ok |
| bitpack-carrier-d16-control | -0.083 | ok |
| bitpack-carrier-d32 | -0.412 | moderate- |
| bitpack-carrier-d64 | -0.161 | ok |
| bitpack-carrier-packed | -0.758 | HIGH- (thermal bounce) |
| bitpack-carrier-packed-simd | -0.081 | ok |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 0/4, lost 1/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 4/4, lost 0/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 20.7ns | 19940.5ns | 0.1% |  |
| bitpack-carrier-d16-control | 22.0ns | 21081.2ns | 0.1% |  |
| bitpack-carrier-d32 | 20.9ns | 15013.9ns | 0.1% |  |
| bitpack-carrier-d64 | 27.0ns | 12151.3ns | 0.2% |  |
| bitpack-carrier-packed | 23.3ns | 17852.0ns | 0.1% |  |
| bitpack-carrier-packed-simd | 22.1ns | 16954.0ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 19938.8-19944.1 ns)
  19938.8 |####################
  19939.0 |#############
  19939.3 |##########################
  19939.6 |########################################
  19939.8 |##########################
  19940.1 |##########################
  19940.4 |####################
  19940.6 |
  19940.9 |######
  19941.2 |######
  19941.4 |######
  19941.7 |######
  19942.0 |
  19942.2 |
  19942.5 |
  19942.8 |#############
  19943.0 |
  19943.3 |
  19943.6 |
  19943.8 |
  (5 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 19939.5-24383.9 ns)
  19939.5 |########################################
  20161.7 |
  20383.9 |
  20606.1 |
  20828.4 |#
  21050.6 |
  21272.8 |
  21495.0 |
  21717.2 |
  21939.5 |
  22161.7 |
  22383.9 |
  22606.1 |
  22828.3 |
  23050.5 |
  23272.8 |
  23495.0 |
  23717.2 |
  23939.4 |
  24161.6 |#########
  (3 below, 4 above range)

bitpack-carrier-d32 (n=40, range 14998.1-15022.9 ns)
  14998.1 |
  14999.3 |
  15000.6 |
  15001.8 |
  15003.1 |
  15004.3 |
  15005.5 |
  15006.8 |
  15008.0 |
  15009.2 |
  15010.5 |############################
  15011.7 |####################################
  15013.0 |####
  15014.2 |####
  15015.4 |
  15016.7 |
  15017.9 |
  15019.1 |
  15020.4 |####################
  15021.6 |########################################
  (4 below, 3 above range)

bitpack-carrier-d64 (n=40, range 11668.9-12375.7 ns)
  11668.9 |#########################
  11704.2 |#####
  11739.6 |
  11774.9 |
  11810.2 |
  11845.6 |
  11880.9 |
  11916.3 |
  11951.6 |
  11986.9 |
  12022.3 |
  12057.6 |
  12093.0 |
  12128.3 |
  12163.6 |
  12199.0 |####################
  12234.3 |####################
  12269.7 |#########################
  12305.0 |########################################
  12340.3 |####################
  (4 below, 5 above range)

bitpack-carrier-packed (n=40, range 15958.8-19694.1 ns)
  15958.8 |#################################
  16145.6 |
  16332.4 |
  16519.1 |
  16705.9 |
  16892.6 |
  17079.4 |
  17266.2 |
  17452.9 |
  17639.7 |
  17826.5 |
  18013.2 |
  18200.0 |
  18386.7 |
  18573.5 |
  18760.3 |
  18947.0 |
  19133.8 |
  19320.6 |
  19507.3 |########################################
  (4 below, 1 above range)

bitpack-carrier-packed-simd (n=40, range 16031.0-19631.5 ns)
  16031.0 |########################################
  16211.0 |#
  16391.0 |
  16571.1 |
  16751.1 |
  16931.1 |
  17111.1 |
  17291.2 |
  17471.2 |
  17651.2 |
  17831.2 |
  18011.3 |
  18191.3 |
  18371.3 |
  18551.3 |
  18731.4 |
  18911.4 |
  19091.4 |
  19271.4 |
  19451.5 |##########
  (3 below, 3 above range)

```
