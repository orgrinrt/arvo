# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 dominates: 25% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-d64 (12.09 us) leads bitpack-carrier-d32 (15.10 us) by 25%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-d64 beats baseline by 41% (significant)

bitpack-carrier-d64 is -8.11 us (41%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d16-control's edge over baseline is significant but tiny (24 ns, 0.12%)

bitpack-carrier-d16-control differs from baseline bitpack-carrier-d16 by 24 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-carrier-d64** at 12094.8 ns median (-39.3% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.83x (fastest 12094.8 ns, slowest 22179.9 ns)

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
| Counter frequency | 2112721357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 21128ns | 20009ns | 19993ns | 20382ns | 24502ns | base |
| bitpack-carrier-d16-control | 22242ns | 22282ns | 20017ns | 22241ns | 24470ns | +5.27% |
| bitpack-carrier-d32 | 15175ns | 15189ns | 15074ns | 15184ns | 15249ns | -28.18% |
| bitpack-carrier-d64 | 12128ns | 12223ns | 11894ns | 12120ns | 12386ns | -42.60% |
| bitpack-carrier-packed | 17991ns | 18000ns | 16095ns | 17995ns | 19875ns | -14.85% |
| bitpack-carrier-packed-simd | 17920ns | 17879ns | 16010ns | 17905ns | 19874ns | -15.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 21053ns | 19926ns | 24415ns | base | 6.226 |
| bitpack-carrier-d16-control | 22158ns | 19944ns | 24371ns | +5.24% | 5.915 |
| bitpack-carrier-d32 | 15099ns | 14998ns | 15176ns | -28.28% | 8.681 |
| bitpack-carrier-d64 | 12011ns | 11780ns | 12277ns | -42.95% | 10.913 |
| bitpack-carrier-packed | 17905ns | 16018ns | 19776ns | -14.95% | 7.320 |
| bitpack-carrier-packed-simd | 17836ns | 15935ns | 19782ns | -15.28% | 7.349 |

## Performance model

- Peak throughput: **11.127 Gops/s** (bitpack-carrier-d64; best 20% batches)
- Ops per call: 131072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.575 | 59.1% |
| bitpack-carrier-d16-control | 5.909 | 53.1% |
| bitpack-carrier-d32 | 8.677 | 78.0% |
| bitpack-carrier-d64 | 10.837 | 97.4% |
| bitpack-carrier-packed | 7.324 | 65.8% |
| bitpack-carrier-packed-simd | 7.371 | 66.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 21128ns | 21128ns | base |
| bitpack-carrier-d16-control | 22242ns | 22242ns | +5.27% |
| bitpack-carrier-d32 | 15175ns | 15175ns | -28.18% |
| bitpack-carrier-d64 | 12128ns | 12128ns | -42.60% |
| bitpack-carrier-packed | 17991ns | 17991ns | -14.85% |
| bitpack-carrier-packed-simd | 17920ns | 17920ns | -15.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 19934ns | base | --- | [19928, 19950] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 22180ns | +23.8ns (+0.1%) | [+17, +26]ns | [19953, 24354] | YES | 0.0022 | 0.0022 | 0 |
| bitpack-carrier-d32 | 15105ns | -4866.6ns (-24.4%) | [-4922, -4818]ns | [15079, 15138] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 12095ns | -8114.2ns (-40.7%) | [-8154, -8078]ns | [11829, 12108] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 17897ns | -3831.9ns (-19.2%) | [-3911, -3730]ns | [16178, 19674] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 17782ns | -3896.3ns (-19.5%) | [-3977, -3837]ns | [16079, 19594] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 24394ns | -0.1% | -38.1% | -49.7% | -19.3% | -19.8% |
| 1 | 2 | 19950ns | +22.1% | -24.4% | -39.3% | -1.1% | -1.0% |
| 1 | 3 | 19928ns | +0.1% | -24.1% | -40.8% | -19.2% | -19.6% |
| 1 | 4 | 19927ns | +0.1% | -23.9% | -40.7% | -19.4% | -19.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.079 | ok |
| bitpack-carrier-d16-control | 0.250 | moderate+ |
| bitpack-carrier-d32 | 0.165 | ok |
| bitpack-carrier-d64 | 0.274 | moderate+ |
| bitpack-carrier-packed | 0.250 | moderate+ |
| bitpack-carrier-packed-simd | 0.234 | moderate+ |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 3/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 4/4, lost 0/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 21.8ns | 21053.5ns | 0.1% |  |
| bitpack-carrier-d16-control | 23.0ns | 22157.6ns | 0.1% |  |
| bitpack-carrier-d32 | 20.8ns | 15099.4ns | 0.1% |  |
| bitpack-carrier-d64 | 25.2ns | 12011.1ns | 0.2% |  |
| bitpack-carrier-packed | 23.5ns | 17905.5ns | 0.1% |  |
| bitpack-carrier-packed-simd | 23.0ns | 17835.6ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 19926.4-24415.4 ns)
  19926.4 |########################################
  20150.9 |
  20375.3 |
  20599.8 |
  20824.2 |
  21048.7 |
  21273.1 |
  21497.6 |
  21722.0 |
  21946.5 |
  22170.9 |
  22395.4 |
  22619.8 |
  22844.3 |
  23068.7 |
  23293.2 |
  23517.6 |
  23742.1 |
  23966.5 |
  24191.0 |#############
  (4 below, 1 above range)

bitpack-carrier-d16-control (n=40, range 19944.1-24371.1 ns)
  19944.1 |########################################
  20165.4 |
  20386.8 |
  20608.1 |
  20829.5 |
  21050.8 |
  21272.2 |
  21493.5 |
  21714.9 |
  21936.2 |
  22157.6 |
  22378.9 |
  22600.3 |
  22821.6 |
  23043.0 |
  23264.3 |
  23485.7 |
  23707.0 |
  23928.4 |
  24149.7 |########################################
  (3 below, 3 above range)

bitpack-carrier-d32 (n=40, range 14997.5-15176.3 ns)
  14997.5 |##########
  15006.5 |
  15015.4 |
  15024.3 |
  15033.3 |##############################
  15042.2 |##########
  15051.1 |####################
  15060.1 |
  15069.0 |####################
  15078.0 |##########
  15086.9 |####################
  15095.8 |##############################
  15104.8 |##############################
  15113.7 |
  15122.7 |####################
  15131.6 |##########
  15140.5 |##########
  15149.5 |##########
  15158.4 |##############################
  15167.3 |########################################
  (5 below, 5 above range)

bitpack-carrier-d64 (n=40, range 11780.0-12277.5 ns)
  11780.0 |####################
  11804.8 |########################################
  11829.7 |#############
  11854.6 |######
  11879.5 |
  11904.3 |######
  11929.2 |######
  11954.1 |
  11979.0 |
  12003.8 |
  12028.7 |
  12053.6 |
  12078.5 |#################################
  12103.4 |#################################
  12128.2 |######
  12153.1 |
  12178.0 |
  12202.9 |
  12227.7 |
  12252.6 |########################################
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 16018.0-19776.4 ns)
  16018.0 |################################
  16205.9 |#######
  16393.9 |
  16581.8 |
  16769.7 |
  16957.6 |
  17145.5 |
  17333.4 |
  17521.4 |
  17709.3 |
  17897.2 |
  18085.1 |
  18273.0 |
  18461.0 |
  18648.9 |
  18836.8 |
  19024.7 |
  19212.6 |
  19400.6 |##
  19588.5 |########################################
  (4 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 15934.6-19781.6 ns)
  15934.6 |########################################
  16126.9 |
  16319.3 |
  16511.6 |
  16704.0 |
  16896.3 |
  17088.7 |
  17281.0 |
  17473.4 |
  17665.7 |
  17858.1 |
  18050.4 |
  18242.8 |
  18435.1 |
  18627.5 |
  18819.8 |
  19012.2 |
  19204.6 |
  19396.9 |###############
  19589.3 |#########################
  (4 below, 4 above range)

```
