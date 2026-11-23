# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 dominates: 14% faster than the next best (wide-rung-ragged-overread)

wide-rung-align16 (12.18 us) leads wide-rung-ragged-overread (13.92 us) by 14%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (12.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 12184.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.22x (fastest 12184.2 ns, slowest 14872.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112722093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 12939ns | 12268ns | 12232ns | 12496ns | 14976ns | base |
| wide-rung-ragged | 14289ns | 14945ns | 12158ns | 14763ns | 14996ns | +10.43% |
| wide-rung-ragged-overread | 13814ns | 14026ns | 12388ns | 13813ns | 15244ns | +6.76% |
| wide-rung-wordround | 14305ns | 14971ns | 12238ns | 14751ns | 15030ns | +10.55% |
| wide-rung-wordround-alias | 14975ns | 14975ns | 14948ns | 14974ns | 15003ns | +15.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 12849ns | 12152ns | 14876ns | base | 1.434 |
| wide-rung-ragged | 14194ns | 12078ns | 14887ns | +10.47% | 1.299 |
| wide-rung-ragged-overread | 13720ns | 12305ns | 15132ns | +6.77% | 1.343 |
| wide-rung-wordround | 14208ns | 12156ns | 14921ns | +10.57% | 1.297 |
| wide-rung-wordround-alias | 14873ns | 14846ns | 14896ns | +15.75% | 1.239 |

## Performance model

- Peak throughput: **1.526 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 18432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.513 | 99.1% |
| wide-rung-ragged | 1.241 | 81.3% |
| wide-rung-ragged-overread | 1.325 | 86.8% |
| wide-rung-wordround | 1.239 | 81.2% |
| wide-rung-wordround-alias | 1.239 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 12939ns | 12939ns | base |
| wide-rung-ragged | 14289ns | 14289ns | +10.43% |
| wide-rung-ragged-overread | 13814ns | 13814ns | +6.76% |
| wide-rung-wordround | 14305ns | 14305ns | +10.55% |
| wide-rung-wordround-alias | 14975ns | 14975ns | +15.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 12184ns | base | --- | [12172, 12201] | --- | --- | --- | --- |
| wide-rung-ragged | 14850ns | no significant difference | [-7, +2664]ns | [14840, 14862] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-ragged-overread | 13916ns | +190.7ns (+1.6%) | [+165, +216]ns | [12330, 15067] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 14872ns | +2687.6ns (+22.1%) | [+2681, +2699]ns | [14862, 14884] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround-alias | 14872ns | +2681.8ns (+22.0%) | [+2669, +2705]ns | [14869, 14881] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 12192ns | +21.9% | +1.1% | +22.0% | +22.0% |
| 1 | 2 | 14872ns | -0.1% | +1.4% | -18.2% | +0.0% |
| 1 | 3 | 12177ns | +22.1% | +23.8% | +22.1% | +22.1% |
| 1 | 4 | 12159ns | -0.3% | +1.2% | +22.6% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.409 | moderate- |
| wide-rung-ragged | -0.086 | ok |
| wide-rung-ragged-overread | -0.250 | moderate- |
| wide-rung-wordround | -0.409 | moderate- |
| wide-rung-wordround-alias | -0.206 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 2/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 1/4, lost 3/4
- **wide-rung-wordround-alias**: won 0/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 21.6ns | 12849.2ns | 0.2% |  |
| wide-rung-ragged | 23.7ns | 14194.2ns | 0.2% |  |
| wide-rung-ragged-overread | 23.5ns | 13719.5ns | 0.2% |  |
| wide-rung-wordround | 24.3ns | 14207.8ns | 0.2% |  |
| wide-rung-wordround-alias | 24.5ns | 14872.9ns | 0.2% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 12151.9-14876.0 ns)
  12151.9 |########################################
  12288.2 |
  12424.4 |
  12560.6 |
  12696.8 |
  12833.0 |
  12969.2 |
  13105.4 |
  13241.6 |
  13377.8 |
  13514.0 |
  13650.2 |
  13786.4 |
  13922.6 |
  14058.8 |
  14195.0 |
  14331.2 |
  14467.4 |
  14603.6 |
  14739.8 |#########
  (4 below, 4 above range)

wide-rung-ragged (n=40, range 12078.1-14887.3 ns)
  12078.1 |########
  12218.6 |
  12359.0 |
  12499.5 |
  12639.9 |
  12780.4 |
  12920.9 |
  13061.3 |#
  13201.8 |
  13342.3 |
  13482.7 |
  13623.2 |
  13763.6 |
  13904.1 |
  14044.6 |
  14185.0 |
  14325.5 |
  14465.9 |
  14606.4 |
  14746.9 |########################################
  (3 below, 3 above range)

wide-rung-ragged-overread (n=40, range 12305.2-15131.8 ns)
  12305.2 |#################################
  12446.5 |
  12587.8 |
  12729.2 |##
  12870.5 |
  13011.8 |
  13153.2 |
  13294.5 |
  13435.8 |
  13577.2 |
  13718.5 |
  13859.8 |
  14001.2 |
  14142.5 |
  14283.8 |
  14425.2 |
  14566.5 |
  14707.8 |
  14849.1 |
  14990.5 |########################################
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 12156.3-14921.0 ns)
  12156.3 |########
  12294.5 |
  12432.8 |
  12571.0 |
  12709.2 |
  12847.5 |
  12985.7 |
  13123.9 |
  13262.2 |
  13400.4 |
  13538.6 |
  13676.9 |
  13815.1 |
  13953.3 |
  14091.6 |
  14229.8 |
  14368.0 |
  14506.3 |
  14644.5 |
  14782.7 |########################################
  (4 below, 3 above range)

wide-rung-wordround-alias (n=40, range 14845.8-14896.3 ns)
  14845.8 |######
  14848.3 |######
  14850.9 |
  14853.4 |
  14855.9 |######
  14858.4 |#############
  14860.9 |
  14863.5 |######
  14866.0 |##########################
  14868.5 |####################
  14871.0 |########################################
  14873.6 |
  14876.1 |
  14878.6 |####################
  14881.1 |####################
  14883.6 |####################
  14886.2 |#############
  14888.7 |######
  14891.2 |
  14893.7 |#############
  (4 below, 3 above range)

```
