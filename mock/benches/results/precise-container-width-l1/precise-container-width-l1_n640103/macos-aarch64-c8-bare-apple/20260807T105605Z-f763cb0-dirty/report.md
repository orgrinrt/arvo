# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 148718% faster than the next best (warm-container-native)

warm-container-kernel (4 ns) leads warm-container-native (5.51 us) by 148718%, a clear separation rather than a photo finish. CV 39.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 101% (significant)

warm-container-kernel is -11.46 us (101%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 3081.6x slower than the field

warm-container-plusone (11.40 us) is 3081.6x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 39.3%)

warm-container-kernel wins on median (4 ns) yet has the highest variance (CV 39.3%), while warm-container-plusone is the steadiest (CV 5.2%, 11.40 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows warm-up / thermal drift (autocorr +0.87)

warm-container-native's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} (148718% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-native, warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 148718% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3081.6x the fastest

Fastest warm-container-kernel (4 ns) to slowest warm-container-plusone (11.40 us): 3081.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 3.0x its best-20%

warm-container-kernel's best 20% of batches run at 2 ns but its worst 20% at 6 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### warm-container-plusone's edge over baseline is significant but tiny (1 ns, 0.01%)

warm-container-plusone differs from baseline warm-container-headroom by 1 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 3.7 ns median (-100.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3081.58x (fastest 3.7 ns, slowest 11401.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 11686ns | 11477ns | 11113ns | 11500ns | 12820ns | base |
| warm-container-kernel | 68ns | 68ns | 61ns | 68ns | 76ns | -99.42% |
| warm-container-minimum | 5677ns | 5601ns | 5290ns | 5682ns | 6050ns | -51.42% |
| warm-container-native | 5800ns | 5572ns | 5393ns | 5721ns | 6445ns | -50.37% |
| warm-container-plusone | 11739ns | 11509ns | 11152ns | 11632ns | 12645ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 11581ns | 11017ns | 12684ns | base | 2.830 |
| warm-container-kernel | 4ns | 2ns | 6ns | -99.97% | 8146.178 |
| warm-container-minimum | 5613ns | 5235ns | 5976ns | -51.53% | 5.838 |
| warm-container-native | 5725ns | 5334ns | 6323ns | -50.56% | 5.723 |
| warm-container-plusone | 11637ns | 11053ns | 12540ns | +0.49% | 2.816 |

## Performance model

- Peak throughput: **15697.246 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 2.879 | 0.0% |
| warm-container-kernel | 8856.216 | 56.4% |
| warm-container-minimum | 5.926 | 0.0% |
| warm-container-native | 5.951 | 0.0% |
| warm-container-plusone | 2.874 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 11686ns | 11686ns | base |
| warm-container-kernel | 68ns | 68ns | -99.42% |
| warm-container-minimum | 5677ns | 5677ns | -51.42% |
| warm-container-native | 5800ns | 5800ns | -50.37% |
| warm-container-plusone | 11739ns | 11739ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 11382ns | base | --- | [11184, 11545] | --- | --- | --- | --- |
| warm-container-kernel | 4ns | -11378.4ns (-100.0%) | [-11542, -11180]ns | [4, 5] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5530ns | -5799.2ns (-50.9%) | [-5962, -5734]ns | [5323, 5923] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 5506ns | -5751.4ns (-50.5%) | [-5913, -5645]ns | [5496, 5922] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 11402ns | no significant difference | [-22, +96]ns | [11180, 11584] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 11238ns | -100.0% | -53.5% | -51.0% | -1.0% |
| 2 | 10958ns | -100.0% | -52.4% | -50.2% | +0.6% |
| 3 | 11245ns | -100.0% | -53.6% | -52.8% | +0.0% |
| 4 | 10917ns | -100.0% | -50.9% | -51.4% | +1.5% |
| 5 | 11073ns | -100.0% | -52.7% | -51.8% | -0.2% |
| 6 | 10960ns | -100.0% | -52.3% | -51.5% | +0.6% |
| 7 | 11180ns | -100.0% | -53.3% | -52.3% | +1.8% |
| 8 | 11189ns | -100.0% | -53.3% | -51.5% | +3.1% |
| 9 | 11246ns | -100.0% | -50.5% | -52.6% | +3.3% |
| 10 | 11365ns | -100.0% | -52.0% | -52.0% | +0.1% |
| 11 | 13575ns | -100.0% | -54.1% | -53.1% | -7.4% |
| 12 | 11590ns | -99.9% | -48.9% | -45.3% | +7.6% |
| 13 | 11537ns | -100.0% | -48.6% | -44.8% | +10.1% |
| 14 | 12488ns | -100.0% | -52.6% | -49.2% | +0.8% |
| 15 | 12484ns | -100.0% | -52.5% | -49.2% | -0.0% |
| 16 | 13455ns | -100.0% | -56.0% | -52.8% | -7.8% |
| 17 | 12342ns | -100.0% | -52.0% | -48.2% | -0.9% |
| 18 | 12098ns | -100.0% | -51.0% | -50.5% | +1.3% |
| 19 | 11553ns | -100.0% | -48.7% | -48.7% | -0.3% |
| 20 | 12256ns | -100.0% | -51.6% | -50.3% | -6.1% |
| 21 | 12450ns | -100.0% | -52.2% | -51.8% | +0.2% |
| 22 | 12425ns | -100.0% | -52.3% | -52.3% | -7.9% |
| 23 | 11995ns | -100.0% | -50.6% | -50.6% | -3.7% |
| 24 | 11502ns | -99.9% | -48.5% | -48.5% | -3.5% |
| 25 | 11525ns | -100.0% | -48.0% | -48.6% | -3.0% |
| 26 | 11580ns | -99.9% | -48.9% | -48.9% | -1.4% |
| 27 | 11399ns | -100.0% | -48.0% | -49.2% | +7.9% |
| 28 | 11437ns | -100.0% | -48.2% | -51.4% | +8.6% |
| 29 | 11506ns | -100.0% | -49.6% | -52.2% | +9.2% |
| 30 | 11558ns | -100.0% | -52.4% | -52.5% | +8.0% |
| 31 | 11097ns | -100.0% | -52.1% | -52.0% | +0.8% |
| 32 | 11116ns | -100.0% | -52.0% | -51.3% | -0.2% |
| 33 | 11169ns | -100.0% | -52.5% | -50.7% | +1.4% |
| 34 | 11140ns | -100.0% | -51.8% | -50.7% | +0.4% |
| 35 | 11102ns | -100.0% | -52.1% | -50.5% | -0.0% |
| 36 | 11124ns | -100.0% | -52.2% | -50.6% | +0.3% |
| 37 | 11033ns | -100.0% | -51.8% | -50.1% | -0.7% |
| 38 | 10998ns | -100.0% | -51.7% | -48.7% | +1.0% |
| 39 | 11192ns | -100.0% | -52.5% | -51.7% | -0.6% |
| 40 | 11127ns | -100.0% | -52.0% | -50.6% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.570 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.032 | ok |
| warm-container-minimum | 0.851 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.869 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.724 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 15/40, lost 20/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.0ns | 11580.6ns | 0.0% |  |
| warm-container-kernel | 2.4ns | 4.0ns | 60.5% | HIGH |
| warm-container-minimum | 2.8ns | 5613.2ns | 0.1% |  |
| warm-container-native | 2.7ns | 5725.4ns | 0.0% |  |
| warm-container-plusone | 2.7ns | 11637.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 11017.2-12684.4 ns)
  11017.2 |#################
  11100.6 |########################################
  11184.0 |############################
  11267.3 |
  11350.7 |###########
  11434.0 |#################
  11517.4 |##################################
  11600.8 |
  11684.1 |
  11767.5 |
  11850.8 |
  11934.2 |#####
  12017.5 |#####
  12100.9 |
  12184.3 |#####
  12267.6 |#####
  12351.0 |#####
  12434.3 |#################
  12517.7 |
  12601.1 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 2.1-6.2 ns)
      2.1 |##########
      2.3 |
      2.5 |##########
      2.7 |#######
      2.9 |
      3.1 |##########
      3.3 |
      3.5 |########################################
      3.7 |
      3.9 |
      4.1 |###
      4.3 |
      4.5 |##############
      4.7 |
      4.9 |###
      5.2 |
      5.4 |##########
      5.6 |
      5.8 |##################
      6.0 |
  (2 below, 2 above range)

warm-container-minimum (n=40, range 5234.6-5976.3 ns)
   5234.6 |
   5271.7 |
   5308.8 |#####################
   5345.9 |########
   5383.0 |
   5420.0 |##
   5457.1 |
   5494.2 |##
   5531.3 |##
   5568.4 |
   5605.5 |
   5642.5 |
   5679.6 |
   5716.7 |
   5753.8 |
   5790.9 |##
   5828.0 |
   5865.0 |
   5902.1 |########################################
   5939.2 |##
  (7 below, 2 above range)

warm-container-native (n=40, range 5333.9-6323.1 ns)
   5333.9 |
   5383.3 |#############
   5432.8 |########
   5482.3 |########################################
   5531.7 |####
   5581.2 |
   5630.6 |####
   5680.1 |
   5729.6 |
   5779.0 |####
   5828.5 |
   5877.9 |######################
   5927.4 |####
   5976.9 |########
   6026.3 |
   6075.8 |####
   6125.2 |
   6174.7 |
   6224.1 |
   6273.6 |
  (7 below, 7 above range)

warm-container-plusone (n=40, range 11052.8-12540.3 ns)
  11052.8 |########################################
  11127.2 |########################################
  11201.6 |######
  11275.9 |######
  11350.3 |####################
  11424.7 |######
  11499.1 |##########################
  11573.4 |######
  11647.8 |
  11722.2 |
  11796.6 |
  11870.9 |
  11945.3 |
  12019.7 |
  12094.1 |
  12168.4 |######
  12242.8 |#############
  12317.2 |
  12391.6 |#############
  12465.9 |##########################
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=36.2% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=67.6% of algo (FFI overhead may distort results)
- **warm-container-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.72 (measurement drift or warm-up artifact)
