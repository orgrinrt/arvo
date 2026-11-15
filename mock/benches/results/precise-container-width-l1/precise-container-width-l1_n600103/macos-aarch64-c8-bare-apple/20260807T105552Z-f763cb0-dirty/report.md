# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (15.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-kernel at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-kernel dominates: 189751% faster than the next best (warm-container-minimum)

warm-container-kernel (4 ns) leads warm-container-minimum (7.97 us) by 189751%, a clear separation rather than a photo finish. CV 30.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 101% (significant)

warm-container-kernel is -15.55 us (101%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3669.8x slower than the field

warm-container-headroom (15.41 us) is 3669.8x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 30.9%)

warm-container-kernel wins on median (4 ns) yet has the highest variance (CV 30.9%), while warm-container-headroom is the steadiest (CV 4.1%, 15.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.86)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.86, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} (189751% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-minimum, warm-container-plusone, warm-container-native, warm-container-headroom} with a 189751% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3669.8x the fastest

Fastest warm-container-kernel (4 ns) to slowest warm-container-headroom (15.41 us): 3669.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.7x its best-20%

warm-container-kernel's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 4.2 ns median (-100.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 3669.79x (fastest 4.2 ns, slowest 15413.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 15707ns | 15508ns | 15067ns | 15556ns | 16801ns | base |
| warm-container-kernel | 64ns | 63ns | 60ns | 63ns | 71ns | -99.59% |
| warm-container-minimum | 8374ns | 8036ns | 7932ns | 8245ns | 9204ns | -46.69% |
| warm-container-native | 8429ns | 8359ns | 8003ns | 8349ns | 9095ns | -46.34% |
| warm-container-plusone | 8372ns | 8085ns | 7934ns | 8162ns | 9442ns | -46.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 15602ns | 14974ns | 16675ns | base | 2.100 |
| warm-container-kernel | 4ns | 2ns | 6ns | -99.97% | 8070.936 |
| warm-container-minimum | 8310ns | 7873ns | 9135ns | -46.74% | 3.943 |
| warm-container-native | 8364ns | 7942ns | 9028ns | -46.39% | 3.918 |
| warm-container-plusone | 8309ns | 7876ns | 9376ns | -46.74% | 3.944 |

## Performance model

- Peak throughput: **14979.657 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.126 | 0.0% |
| warm-container-kernel | 7801.905 | 52.1% |
| warm-container-minimum | 4.109 | 0.0% |
| warm-container-native | 3.950 | 0.0% |
| warm-container-plusone | 4.085 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 15707ns | 15707ns | base |
| warm-container-kernel | 64ns | 64ns | -99.59% |
| warm-container-minimum | 8374ns | 8374ns | -46.69% |
| warm-container-native | 8429ns | 8429ns | -46.34% |
| warm-container-plusone | 8372ns | 8372ns | -46.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 15413ns | base | --- | [15226, 15710] | --- | --- | --- | --- |
| warm-container-kernel | 4ns | -15409.4ns (-100.0%) | [-15706, -15220]ns | [4, 5] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 7974ns | -7194.6ns (-46.7%) | [-7446, -6833]ns | [7942, 8296] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 8296ns | -7207.1ns (-46.8%) | [-7423, -7049]ns | [8085, 8334] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8021ns | -7131.2ns (-46.3%) | [-7403, -6903]ns | [7951, 8145] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 14923ns | -100.0% | -45.9% | -47.3% | -46.9% |
| 2 | 15251ns | -100.0% | -48.1% | -46.2% | -48.3% |
| 3 | 15262ns | -100.0% | -48.5% | -47.4% | -45.3% |
| 4 | 14922ns | -100.0% | -46.7% | -46.8% | -45.3% |
| 5 | 15011ns | -100.0% | -43.2% | -47.2% | -46.3% |
| 6 | 14990ns | -100.0% | -40.0% | -47.0% | -46.0% |
| 7 | 15095ns | -100.0% | -41.5% | -47.8% | -47.1% |
| 8 | 15466ns | -100.0% | -48.5% | -48.3% | -48.1% |
| 9 | 15075ns | -100.0% | -47.2% | -46.3% | -46.9% |
| 10 | 15360ns | -100.0% | -48.7% | -47.9% | -47.2% |
| 11 | 17260ns | -100.0% | -54.4% | -44.6% | -54.4% |
| 12 | 16952ns | -100.0% | -53.6% | -47.1% | -53.5% |
| 13 | 16898ns | -100.0% | -53.1% | -47.1% | -53.4% |
| 14 | 16909ns | -100.0% | -53.4% | -47.2% | -53.4% |
| 15 | 16956ns | -100.0% | -53.2% | -47.2% | -53.5% |
| 16 | 15468ns | -100.0% | -49.1% | -42.2% | -49.1% |
| 17 | 15710ns | -100.0% | -49.9% | -43.0% | -49.8% |
| 18 | 15177ns | -100.0% | -48.1% | -41.2% | -47.7% |
| 19 | 15155ns | -100.0% | -48.0% | -40.9% | -47.2% |
| 20 | 15758ns | -100.0% | -50.0% | -43.1% | -49.5% |
| 21 | 14916ns | -100.0% | -44.4% | -46.4% | -47.2% |
| 22 | 15202ns | -100.0% | -45.4% | -47.3% | -48.2% |
| 23 | 15634ns | -100.0% | -47.0% | -48.8% | -49.6% |
| 24 | 16448ns | -100.0% | -51.1% | -51.2% | -49.2% |
| 25 | 15800ns | -100.0% | -49.7% | -48.5% | -49.0% |
| 26 | 15890ns | -100.0% | -49.9% | -49.6% | -49.7% |
| 27 | 15292ns | -100.0% | -48.5% | -46.2% | -48.0% |
| 28 | 15044ns | -100.0% | -47.0% | -42.7% | -45.8% |
| 29 | 14990ns | -100.0% | -46.8% | -44.8% | -45.7% |
| 30 | 15920ns | -100.0% | -49.5% | -47.8% | -49.7% |
| 31 | 16058ns | -100.0% | -40.6% | -48.0% | -40.5% |
| 32 | 15777ns | -100.0% | -39.5% | -47.2% | -39.4% |
| 33 | 15710ns | -100.0% | -41.2% | -47.2% | -38.1% |
| 34 | 15695ns | -100.0% | -43.1% | -47.0% | -38.4% |
| 35 | 15742ns | -100.0% | -43.3% | -47.2% | -39.3% |
| 36 | 15572ns | -100.0% | -42.6% | -46.7% | -42.1% |
| 37 | 15321ns | -100.0% | -41.5% | -45.5% | -41.3% |
| 38 | 15327ns | -100.0% | -41.7% | -45.5% | -41.7% |
| 39 | 15150ns | -100.0% | -41.1% | -46.7% | -41.1% |
| 40 | 14998ns | -100.0% | -40.5% | -44.6% | -43.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.637 | HIGH+ (drift/warm-up) |
| warm-container-kernel | -0.014 | ok |
| warm-container-minimum | 0.794 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.665 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.858 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.3ns | 15602.0ns | 0.0% |  |
| warm-container-kernel | 2.3ns | 4.1ns | 56.0% | HIGH |
| warm-container-minimum | 2.9ns | 8309.9ns | 0.0% |  |
| warm-container-native | 2.9ns | 8363.9ns | 0.0% |  |
| warm-container-plusone | 2.7ns | 8309.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 14974.3-16675.0 ns)
  14974.3 |########################################
  15059.3 |################
  15144.3 |################################
  15229.4 |########################
  15314.4 |########################
  15399.5 |################
  15484.5 |
  15569.5 |################
  15654.6 |########################
  15739.6 |################################
  15824.7 |########
  15909.7 |########
  15994.7 |########
  16079.8 |
  16164.8 |
  16249.9 |
  16334.9 |
  16419.9 |########
  16505.0 |
  16590.0 |
  (3 below, 5 above range)

warm-container-kernel (n=40, range 2.2-5.8 ns)
      2.2 |
      2.4 |####
      2.5 |
      2.7 |#################
      2.9 |
      3.1 |
      3.3 |########
      3.5 |
      3.6 |#################
      3.8 |
      4.0 |
      4.2 |########################################
      4.4 |
      4.5 |#################
      4.7 |
      4.9 |#################
      5.1 |
      5.3 |########
      5.4 |
      5.6 |#################
  (5 below, 1 above range)

warm-container-minimum (n=40, range 7873.0-9134.9 ns)
   7873.0 |########################################
   7936.1 |#########################
   7999.2 |#######
   8062.2 |###
   8125.3 |
   8188.4 |
   8251.5 |##########
   8314.6 |
   8377.7 |
   8440.8 |
   8503.9 |###
   8567.0 |
   8630.1 |
   8693.2 |
   8756.3 |
   8819.4 |###
   8882.5 |#####################
   8945.6 |#######
   9008.7 |
   9071.8 |
  (3 below, 3 above range)

warm-container-native (n=40, range 7942.3-9028.1 ns)
   7942.3 |####
   7996.6 |###############################
   8050.9 |########
   8105.2 |####
   8159.5 |####
   8213.8 |####
   8268.0 |###############################
   8322.3 |#################
   8376.6 |
   8430.9 |
   8485.2 |
   8539.5 |
   8593.8 |####
   8648.1 |
   8702.4 |
   8756.7 |
   8811.0 |
   8865.2 |
   8919.5 |########################################
   8973.8 |
  (5 below, 1 above range)

warm-container-plusone (n=40, range 7876.2-9376.0 ns)
   7876.2 |########################################
   7951.2 |########################
   8026.2 |################
   8101.2 |################
   8176.1 |
   8251.1 |
   8326.1 |########
   8401.1 |
   8476.1 |####
   8551.1 |
   8626.1 |
   8701.1 |
   8776.1 |
   8851.0 |
   8926.0 |############
   9001.0 |####
   9076.0 |
   9151.0 |
   9226.0 |
   9301.0 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=31.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=50.0% of algo (FFI overhead may distort results)
- **warm-container-minimum**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.86 (measurement drift or warm-up artifact)
