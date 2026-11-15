# Saturating accumulation of a W-bit column into a 64-bit accumulator, with and without the theorem that the saturation cannot occur (8192 elements)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.53)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (26 ns) is smaller than the fastest variant's own run-to-run std-dev (162 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-container-native is inconsistent: worst-20% is 12.1x its best-20%

warm-container-native's best 20% of batches run at 5.27 us but its worst 20% at 63.82 us (12.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.5% of the fastest

All 6 variants sit between 5.48 us and 5.50 us - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### warm-container-kernel's edge over baseline is significant but tiny (-18 ns, 0.33%)

warm-container-kernel differs from baseline warm-container-headroom by -18 ns (0.33%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-lanes-deferred** at 5476.2 ns median (-0.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 5476.2 ns, slowest 5502.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 6960ns | 5603ns | 5414ns | 5718ns | 12233ns | base |
| warm-container-kernel | 5547ns | 5561ns | 5294ns | 5541ns | 5821ns | -20.30% |
| warm-container-lanes-deferred | 5544ns | 5550ns | 5346ns | 5526ns | 5795ns | -20.36% |
| warm-container-minimum | 8545ns | 5577ns | 5324ns | 5620ns | 20543ns | +22.77% |
| warm-container-native | 17207ns | 5564ns | 5329ns | 5560ns | 64027ns | +147.22% |
| warm-container-plusone | 5527ns | 5565ns | 5312ns | 5541ns | 5701ns | -20.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 6660ns | 5314ns | 11153ns | base | 2.460 |
| warm-container-kernel | 5474ns | 5229ns | 5734ns | -17.81% | 2.993 |
| warm-container-lanes-deferred | 5468ns | 5278ns | 5711ns | -17.88% | 2.996 |
| warm-container-minimum | 8437ns | 5263ns | 20290ns | +26.70% | 1.942 |
| warm-container-native | 17114ns | 5268ns | 63823ns | +156.99% | 0.957 |
| warm-container-plusone | 5459ns | 5251ns | 5628ns | -18.02% | 3.001 |

## Performance model

- Peak throughput: **3.133 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.980 | 95.1% |
| warm-container-kernel | 2.984 | 95.2% |
| warm-container-lanes-deferred | 2.992 | 95.5% |
| warm-container-minimum | 2.978 | 95.0% |
| warm-container-native | 2.980 | 95.1% |
| warm-container-plusone | 2.981 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 6960ns | 6960ns | base |
| warm-container-kernel | 5547ns | 5547ns | -20.30% |
| warm-container-lanes-deferred | 5544ns | 5544ns | -20.36% |
| warm-container-minimum | 8545ns | 8545ns | +22.77% |
| warm-container-native | 17207ns | 17207ns | +147.22% |
| warm-container-plusone | 5527ns | 5527ns | -20.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 5497ns | base | --- | [5472, 5536] | --- | --- | --- | --- |
| warm-container-kernel | 5490ns | -21.7ns (-0.4%) | [-95, -4]ns | [5457, 5501] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-container-lanes-deferred | 5476ns | no significant difference | [-186, +11]ns | [5394, 5498] | no | 0.2564 | 0.1539 | 0 |
| warm-container-minimum | 5502ns | no significant difference | [-50, +22]ns | [5459, 5534] | no | 1.0000 | 0.8746 | 0 |
| warm-container-native | 5498ns | no significant difference | [-32, +24]ns | [5493, 5517] | no | 1.0000 | 1.0000 | 0 |
| warm-container-plusone | 5496ns | -50.0ns (-0.9%) | [-79, -20]ns | [5448, 5505] | YES | 0.0111 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 5398ns | -3.3% | +4.6% | -3.3% | -0.2% | -0.2% |
| 2 | 5272ns | -0.3% | +0.6% | -0.6% | +0.3% | +0.3% |
| 3 | 5378ns | -2.7% | +0.2% | -2.9% | -1.9% | -0.9% |
| 4 | 5566ns | -6.3% | -5.5% | -6.2% | -2.1% | -6.2% |
| 5 | 5336ns | -2.3% | +0.6% | -2.1% | -0.8% | -2.0% |
| 6 | 5300ns | -1.3% | +4.2% | +2.8% | -0.9% | -1.5% |
| 7 | 5287ns | -1.1% | +1.7% | +1.6% | +0.4% | -1.0% |
| 8 | 5270ns | -0.7% | -0.7% | +1.1% | -0.9% | -0.8% |
| 9 | 5300ns | +0.5% | -0.7% | +0.4% | -1.5% | -1.0% |
| 10 | 5470ns | +0.4% | -4.4% | -2.3% | -3.5% | +0.8% |
| 11 | 5493ns | -1.1% | +3.6% | -0.6% | -1.1% | -0.1% |
| 12 | 5495ns | -0.3% | +4.5% | +0.1% | +0.0% | -0.4% |
| 13 | 5492ns | -0.1% | +0.3% | -0.2% | +0.1% | -1.4% |
| 14 | 5503ns | -0.2% | +3.5% | +1.2% | -0.2% | -1.2% |
| 15 | 5394ns | +1.8% | +1.8% | +1.7% | +1.9% | -0.3% |
| 16 | 5458ns | +1.5% | +0.6% | -1.1% | +0.6% | -1.8% |
| 17 | 5495ns | +0.2% | -0.1% | -2.1% | +0.0% | -1.3% |
| 18 | 5440ns | +0.9% | +0.9% | +0.3% | +1.0% | +0.4% |
| 19 | 5403ns | +1.6% | +0.5% | +1.0% | +1.0% | +1.7% |
| 20 | 5367ns | +3.1% | +0.2% | -0.7% | +1.4% | +2.4% |
| 21 | 5500ns | +0.3% | -1.9% | +0.6% | +0.6% | +0.1% |
| 22 | 5551ns | -0.2% | -1.0% | +7.6% | -0.4% | -0.9% |
| 23 | 5542ns | -0.3% | -0.8% | +0.1% | -0.2% | -0.8% |
| 24 | 5530ns | +0.3% | -1.5% | -0.3% | +0.2% | -0.4% |
| 25 | 5542ns | -1.2% | -3.2% | -0.7% | -0.7% | -0.2% |
| 26 | 5474ns | +0.6% | -2.1% | +0.7% | +0.8% | +1.2% |
| 27 | 5511ns | -0.4% | -3.7% | -0.0% | -0.1% | -0.6% |
| 28 | 5493ns | +0.2% | -3.6% | +0.1% | +0.4% | +0.2% |
| 29 | 5501ns | -0.0% | -2.9% | +0.4% | -0.1% | +0.2% |
| 30 | 5499ns | +0.1% | -1.3% | +0.7% | +0.5% | +0.8% |
| 31 | 21286ns | -74.2% | -74.2% | -62.5% | -27.7% | -74.0% |
| 32 | 7029ns | -21.8% | -23.0% | -9.2% | -21.6% | -21.7% |
| 33 | 10672ns | -49.6% | -48.3% | -7.7% | -32.9% | -48.4% |
| 34 | 8671ns | -37.8% | -32.9% | +1094.8% | +679.1% | -34.5% |
| 35 | 6870ns | -20.8% | -20.5% | -9.4% | +2215.7% | -19.8% |
| 36 | 7962ns | -32.9% | -30.5% | -26.5% | -30.9% | -30.9% |
| 37 | 8632ns | -25.5% | -36.0% | +11.5% | -35.7% | -36.2% |
| 38 | 13771ns | -55.5% | -56.1% | -39.6% | +6.8% | -55.5% |
| 39 | 7044ns | -21.6% | -21.7% | +17.5% | +1012.8% | -22.0% |
| 40 | 11182ns | -49.6% | -50.8% | -26.0% | +1355.4% | -50.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.177 | ok |
| warm-container-kernel | 0.530 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.319 | moderate+ |
| warm-container-minimum | 0.020 | ok |
| warm-container-native | 0.346 | moderate+ |
| warm-container-plusone | 0.475 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 25/40, lost 12/40
- **warm-container-lanes-deferred**: won 24/40, lost 15/40
- **warm-container-minimum**: won 20/40, lost 18/40
- **warm-container-native**: won 18/40, lost 18/40
- **warm-container-plusone**: won 29/40, lost 9/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 4.9ns | 6659.5ns | 0.1% |  |
| warm-container-kernel | 3.1ns | 5473.5ns | 0.1% |  |
| warm-container-lanes-deferred | 3.0ns | 5468.5ns | 0.1% |  |
| warm-container-minimum | 5.4ns | 8437.5ns | 0.1% |  |
| warm-container-native | 5.5ns | 17114.4ns | 0.0% |  |
| warm-container-plusone | 3.2ns | 5459.2ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 5313.8-11152.6 ns)
   5313.8 |########################################
   5605.7 |
   5897.6 |
   6189.6 |
   6481.5 |
   6773.5 |####
   7065.4 |
   7357.4 |
   7649.3 |
   7941.3 |#
   8233.2 |
   8525.1 |###
   8817.1 |
   9109.0 |
   9401.0 |
   9692.9 |
   9984.9 |
  10276.8 |
  10568.8 |#
  10860.7 |
  (5 below, 3 above range)

warm-container-kernel (n=40, range 5228.7-5734.5 ns)
   5228.7 |#########
   5254.0 |###
   5279.3 |
   5304.6 |###
   5329.9 |###
   5355.2 |###
   5380.5 |###
   5405.7 |
   5431.0 |######
   5456.3 |######
   5481.6 |########################################
   5506.9 |###############
   5532.2 |#########
   5557.5 |
   5582.7 |
   5608.0 |###
   5633.3 |
   5658.6 |
   5683.9 |
   5709.2 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 5278.3-5711.1 ns)
   5278.3 |########
   5300.0 |################
   5321.6 |########
   5343.3 |########
   5364.9 |################################
   5386.5 |################
   5408.2 |########################
   5429.8 |########
   5451.4 |########
   5473.1 |########################################
   5494.7 |########################################
   5516.4 |################################
   5538.0 |
   5559.6 |
   5581.3 |
   5602.9 |
   5624.5 |########
   5646.2 |
   5667.8 |########
   5689.5 |########
  (4 below, 3 above range)

warm-container-minimum (n=40, range 5263.1-20289.8 ns)
   5263.1 |########################################
   6014.4 |###
   6765.7 |
   7517.1 |#
   8268.4 |####
   9019.7 |#
   9771.1 |#
  10522.4 |
  11273.7 |
  12025.1 |
  12776.4 |
  13527.8 |
  14279.1 |
  15030.4 |
  15781.8 |
  16533.1 |
  17284.4 |
  18035.8 |
  18787.1 |
  19538.4 |
  (5 below, 1 above range)

warm-container-native (n=40, range 5268.0-63822.6 ns)
   5268.0 |########################################
   8195.7 |
  11123.4 |
  14051.2 |##
  16978.9 |
  19906.6 |
  22834.4 |
  25762.1 |
  28689.8 |
  31617.5 |
  34545.3 |
  37473.0 |
  40400.7 |
  43328.5 |
  46256.2 |
  49183.9 |
  52111.7 |
  55039.4 |
  57967.1 |
  60894.9 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 5250.5-5628.0 ns)
   5250.5 |
   5269.4 |###
   5288.3 |
   5307.1 |
   5326.0 |###
   5344.9 |###
   5363.8 |###
   5382.6 |###
   5401.5 |###
   5420.4 |######
   5439.3 |
   5458.1 |######
   5477.0 |#############
   5495.9 |########################################
   5514.8 |##########
   5533.6 |######
   5552.5 |###
   5571.4 |
   5590.3 |
   5609.1 |
  (6 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=44.8% (high variance, measurements may be unstable)
- **warm-container-kernel**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **warm-container-minimum**: CV=181.2% (high variance, measurements may be unstable)
- **warm-container-minimum**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-container-native**: CV=211.2% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 12.1x (possible bimodal distribution)
