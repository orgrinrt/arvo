# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (6.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 348.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1082% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (348.73 us) leads bitpack-write-windowed (4.12 ms) by 1082%, a clear separation rather than a photo finish. CV 83.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 96% (significant)

bitpack-write-dense is -5.94 ms (96%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 17.8x slower than the field

bitpack-write-aligned (6.21 ms) is 17.8x the fastest (348.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 83.0%)

bitpack-write-dense wins on median (348.73 us) yet has the highest variance (CV 83.0%), while bitpack-write-windowed is the steadiest (CV 24.5%, 4.12 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-aligned shows warm-up / thermal drift (autocorr +0.84)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation +0.84, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 17.8x the fastest

Fastest bitpack-write-dense (348.73 us) to slowest bitpack-write-aligned (6.21 ms): 17.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 3.6x its best-20%

bitpack-write-dense's best 20% of batches run at 246.05 us but its worst 20% at 879.83 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 348725.7 ns median (-94.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 17.80x (fastest 348725.7 ns, slowest 6208342.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 7509354ns | 6210526ns | 4524324ns | 6693900ns | 12940745ns | base |
| bitpack-write-dense | 435120ns | 349284ns | 246451ns | 348985ns | 882196ns | -94.21% |
| bitpack-write-windowed | 4285842ns | 4122477ns | 3386704ns | 4092416ns | 5765259ns | -42.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 7506050ns | 4522599ns | 12934110ns | base | 0.279 |
| bitpack-write-dense | 434154ns | 246051ns | 879830ns | -94.22% | 4.830 |
| bitpack-write-windowed | 4283947ns | 3385146ns | 5762483ns | -42.93% | 0.490 |

## Performance model

- Peak throughput: **8.523 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.338 | 4.0% |
| bitpack-write-dense | 6.014 | 70.6% |
| bitpack-write-windowed | 0.509 | 6.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 7509354ns | 7509354ns | base |
| bitpack-write-dense | 435120ns | 435120ns | -94.21% |
| bitpack-write-windowed | 4285842ns | 4285842ns | -42.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 6208342ns | base | --- | [6013108, 6478317] | --- | --- | --- | --- |
| bitpack-write-dense | 348726ns | -5863219.3ns (-94.4%) | [-6143358, -5695305]ns | [323149, 370618] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 4120791ns | -2099600.5ns (-33.8%) | [-2610227, -1875090]ns | [3902000, 4206955] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 9105388ns | -85.4% | -35.3% |
| 2 | 10368238ns | -83.2% | -59.3% |
| 3 | 10129636ns | -94.9% | -28.4% |
| 4 | 10770223ns | -93.4% | -58.6% |
| 5 | 9953268ns | -96.4% | -53.9% |
| 6 | 11583727ns | -95.9% | -64.5% |
| 7 | 14843959ns | -94.9% | -68.4% |
| 8 | 18231353ns | -96.2% | -72.2% |
| 9 | 15833667ns | -96.1% | -73.5% |
| 10 | 11445395ns | -94.0% | -54.1% |
| 11 | 6230236ns | -94.8% | -31.7% |
| 12 | 5889663ns | -94.5% | -34.0% |
| 13 | 6068583ns | -95.3% | -27.0% |
| 14 | 6016907ns | -94.6% | -20.8% |
| 15 | 6398693ns | -94.8% | -28.1% |
| 16 | 6008992ns | -93.8% | -34.8% |
| 17 | 6009309ns | -95.3% | -35.8% |
| 18 | 6370660ns | -94.2% | -41.0% |
| 19 | 6062838ns | -93.9% | -37.1% |
| 20 | 5996580ns | -94.5% | -32.7% |
| 21 | 4507012ns | -93.2% | -22.0% |
| 22 | 4939971ns | -95.2% | -34.0% |
| 23 | 6299320ns | -96.1% | -45.6% |
| 24 | 4932898ns | -94.5% | -26.5% |
| 25 | 4515402ns | -94.5% | -24.4% |
| 26 | 4663256ns | -94.8% | -22.2% |
| 27 | 4335245ns | -93.9% | -22.0% |
| 28 | 4501095ns | -94.7% | -25.6% |
| 29 | 4320299ns | -94.3% | -24.3% |
| 30 | 4405582ns | -94.4% | -21.4% |
| 31 | 6328249ns | -94.1% | -34.6% |
| 32 | 6082685ns | -93.9% | -32.1% |
| 33 | 9184849ns | -95.7% | -56.7% |
| 34 | 6282090ns | -93.9% | -33.6% |
| 35 | 5569065ns | -93.1% | -25.0% |
| 36 | 6186449ns | -94.2% | -32.7% |
| 37 | 6040224ns | -94.6% | +41.5% |
| 38 | 6876755ns | -94.6% | -32.4% |
| 39 | 10396315ns | -96.7% | -60.7% |
| 40 | 6557941ns | -94.9% | -39.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.836 | HIGH+ (drift/warm-up) |
| bitpack-write-dense | 0.563 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | 0.225 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 39/40, lost 1/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 253.9ns | 7506050.4ns | 0.0% |  |
| bitpack-write-dense | 74.4ns | 434154.4ns | 0.0% |  |
| bitpack-write-windowed | 133.8ns | 4283946.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 4522598.5-12934109.6 ns)
  4522598.5 |############
  4943174.1 |
  5363749.7 |####
  5784325.2 |########################################
  6204900.8 |############################
  6625476.3 |####
  7046051.9 |
  7466627.4 |
  7887203.0 |
  8307778.5 |
  8728354.1 |####
  9148929.6 |####
  9569505.2 |####
  9990080.7 |############
  10410656.3 |####
  10831231.8 |
  11251807.4 |########
  11672382.9 |
  12092958.5 |
  12513534.0 |
  (6 below, 3 above range)

bitpack-write-dense (n=40, range 246051.4-879830.4 ns)
  246051.4 |####################
  277740.4 |###############
  309429.3 |########################################
  341118.3 |###################################
  372807.2 |####################
  404496.2 |
  436185.1 |
  467874.0 |#####
  499563.0 |#####
  531251.9 |
  562940.9 |
  594629.8 |#####
  626318.8 |
  658007.7 |#####
  689696.7 |##########
  721385.6 |#####
  753074.6 |
  784763.5 |
  816452.5 |
  848141.4 |
  (5 below, 2 above range)

bitpack-write-windowed (n=40, range 3385145.9-5762482.6 ns)
  3385145.9 |#################
  3504012.8 |#####
  3622879.6 |###########
  3741746.4 |#################
  3860613.3 |######################
  3979480.1 |###########
  4098346.9 |########################################
  4217213.8 |###########
  4336080.6 |#####
  4454947.4 |#####
  4573814.3 |######################
  4692681.1 |#####
  4811548.0 |
  4930414.8 |
  5049281.6 |#####
  5168148.5 |#####
  5287015.3 |
  5405882.1 |
  5524749.0 |
  5643615.8 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-write-aligned**: CV=43.6% (high variance, measurements may be unstable)
- **bitpack-write-aligned**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **bitpack-write-dense**: CV=66.7% (high variance, measurements may be unstable)
- **bitpack-write-dense**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **bitpack-write-dense**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **bitpack-write-windowed**: CV=23.5% (high variance, measurements may be unstable)
