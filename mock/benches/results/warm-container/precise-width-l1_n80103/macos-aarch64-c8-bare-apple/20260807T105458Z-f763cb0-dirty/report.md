# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 318353% faster than the next best (warm-container-headroom)

warm-container-kernel (3 ns) leads warm-container-headroom (10.51 us) by 318353%, a clear separation rather than a photo finish. CV 37.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 100% (significant)

warm-container-kernel is -10.50 us (100%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native is an outlier: 3231.6x slower than the field

warm-container-native (10.66 us) is 3231.6x the fastest (3 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 37.1%)

warm-container-kernel wins on median (3 ns) yet has the highest variance (CV 37.1%), while warm-container-plusone is the steadiest (CV 2.0%, 10.64 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-minimum shows warm-up / thermal drift (autocorr +0.71)

warm-container-minimum's per-pass series has lag-1 autocorrelation +0.71, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} (318353% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} with a 318353% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3231.6x the fastest

Fastest warm-container-kernel (3 ns) to slowest warm-container-native (10.66 us): 3231.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.6x its best-20%

warm-container-kernel's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 3.3 ns median (-100.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3231.58x (fastest 3.3 ns, slowest 10664.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 10703ns | 10588ns | 10481ns | 10618ns | 11183ns | base |
| warm-container-kernel | 65ns | 64ns | 61ns | 64ns | 72ns | -99.39% |
| warm-container-minimum | 10774ns | 10661ns | 10489ns | 10719ns | 11225ns | +0.66% |
| warm-container-native | 10817ns | 10731ns | 10511ns | 10780ns | 11233ns | +1.06% |
| warm-container-plusone | 10803ns | 10710ns | 10572ns | 10758ns | 11167ns | +0.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 10632ns | 10415ns | 11102ns | base | 3.082 |
| warm-container-kernel | 4ns | 2ns | 6ns | -99.97% | 9070.727 |
| warm-container-minimum | 10698ns | 10421ns | 11137ns | +0.62% | 3.063 |
| warm-container-native | 10741ns | 10442ns | 11146ns | +1.02% | 3.051 |
| warm-container-plusone | 10730ns | 10501ns | 11088ns | +0.92% | 3.054 |

## Performance model

- Peak throughput: **15420.235 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.118 | 0.0% |
| warm-container-kernel | 9929.697 | 64.4% |
| warm-container-minimum | 3.095 | 0.0% |
| warm-container-native | 3.073 | 0.0% |
| warm-container-plusone | 3.081 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 10703ns | 10703ns | base |
| warm-container-kernel | 65ns | 65ns | -99.39% |
| warm-container-minimum | 10774ns | 10774ns | +0.66% |
| warm-container-native | 10817ns | 10817ns | +1.06% |
| warm-container-plusone | 10803ns | 10803ns | +0.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 10509ns | base | --- | [10465, 10624] | --- | --- | --- | --- |
| warm-container-kernel | 3ns | -10505.2ns (-100.0%) | [-10620, -10462]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 10586ns | no significant difference | [-3, +168]ns | [10492, 10801] | no | 0.1996 | 0.1996 | 1 |
| warm-container-native | 10664ns | +95.4ns (+0.9%) | [+13, +155]ns | [10592, 10807] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| warm-container-plusone | 10636ns | +109.5ns (+1.0%) | [+33, +191]ns | [10616, 10754] | YES | 0.0129 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 10633ns | -100.0% | +3.0% | +1.0% | +4.1% |
| 2 | 10490ns | -99.9% | +4.5% | +5.6% | +4.8% |
| 3 | 10423ns | -99.9% | +5.1% | +3.9% | +9.3% |
| 4 | 10424ns | -100.0% | +0.1% | +3.6% | +3.9% |
| 5 | 10504ns | -100.0% | -0.2% | +2.9% | +1.0% |
| 6 | 10558ns | -99.9% | -1.2% | +3.3% | +0.8% |
| 7 | 11211ns | -100.0% | -6.9% | -4.1% | -5.3% |
| 8 | 11197ns | -100.0% | -6.8% | -5.3% | -5.5% |
| 9 | 10506ns | -100.0% | -0.9% | +1.0% | +0.9% |
| 10 | 10416ns | -100.0% | +0.3% | +1.5% | +3.9% |
| 11 | 11012ns | -100.0% | +0.4% | +0.0% | -2.4% |
| 12 | 11048ns | -100.0% | +4.7% | -0.3% | +0.4% |
| 13 | 11024ns | -100.0% | -0.6% | +0.4% | -3.6% |
| 14 | 10968ns | -100.0% | +0.0% | +0.2% | -3.5% |
| 15 | 10791ns | -100.0% | +2.5% | +1.4% | -1.6% |
| 16 | 10600ns | -100.0% | +5.4% | +10.7% | +3.5% |
| 17 | 10798ns | -100.0% | +1.7% | +2.1% | -0.0% |
| 18 | 11528ns | -100.0% | -4.9% | -2.4% | -3.7% |
| 19 | 10826ns | -100.0% | +2.5% | +1.2% | +1.2% |
| 20 | 10686ns | -100.0% | +4.8% | -0.4% | +2.7% |
| 21 | 10420ns | -100.0% | +3.7% | -0.1% | +0.2% |
| 22 | 10415ns | -100.0% | +1.9% | +0.2% | +2.5% |
| 23 | 10412ns | -100.0% | +0.8% | -0.0% | +2.6% |
| 24 | 10512ns | -100.0% | -0.1% | -0.9% | +1.9% |
| 25 | 10461ns | -100.0% | +3.2% | -0.2% | +3.9% |
| 26 | 10694ns | -100.0% | -1.2% | -1.6% | -0.6% |
| 27 | 10438ns | -100.0% | +2.7% | -0.2% | +1.2% |
| 28 | 10500ns | -100.0% | +0.3% | -0.2% | +5.8% |
| 29 | 10414ns | -100.0% | -0.0% | +1.5% | +3.4% |
| 30 | 10479ns | -100.0% | -0.6% | +1.1% | +0.9% |
| 31 | 10661ns | -100.0% | -0.6% | -0.6% | -0.9% |
| 32 | 10415ns | -100.0% | +1.5% | +2.4% | +2.1% |
| 33 | 10417ns | -100.0% | +0.0% | +3.8% | +1.6% |
| 34 | 10420ns | -100.0% | -0.0% | +1.6% | +1.1% |
| 35 | 10421ns | -99.9% | +0.1% | +1.5% | +0.7% |
| 36 | 10469ns | -100.0% | -0.2% | +3.5% | +1.1% |
| 37 | 10614ns | -100.0% | -1.8% | +0.8% | +0.1% |
| 38 | 10530ns | -100.0% | +0.2% | +0.4% | +1.7% |
| 39 | 10415ns | -99.9% | +1.8% | +2.3% | +0.0% |
| 40 | 10535ns | -100.0% | +0.7% | +0.5% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.514 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.130 | ok |
| warm-container-minimum | 0.713 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.634 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.369 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 13/40, lost 23/40
- **warm-container-native**: won 12/40, lost 26/40
- **warm-container-plusone**: won 10/40, lost 27/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.9ns | 10632.1ns | 0.0% |  |
| warm-container-kernel | 2.6ns | 3.6ns | 72.8% | HIGH |
| warm-container-minimum | 4.7ns | 10698.0ns | 0.0% |  |
| warm-container-native | 3.1ns | 10740.7ns | 0.0% |  |
| warm-container-plusone | 2.8ns | 10730.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 10415.4-11101.6 ns)
  10415.4 |########################################
  10449.7 |###############
  10484.0 |#########################
  10518.4 |##########
  10552.7 |#####
  10587.0 |##########
  10621.3 |#####
  10655.6 |##########
  10689.9 |#####
  10724.2 |
  10758.5 |#####
  10792.8 |##########
  10827.1 |
  10861.4 |
  10895.8 |
  10930.1 |
  10964.4 |#####
  10998.7 |##########
  11033.0 |#####
  11067.3 |
  (5 below, 3 above range)

warm-container-kernel (n=40, range 2.1-5.6 ns)
      2.1 |
      2.3 |
      2.5 |###############
      2.6 |
      2.8 |########################################
      3.0 |
      3.2 |##############################
      3.3 |
      3.5 |
      3.7 |########################################
      3.9 |
      4.0 |##########
      4.2 |
      4.4 |
      4.6 |#####
      4.7 |
      4.9 |##########
      5.1 |
      5.3 |###############
      5.4 |
  (4 below, 3 above range)

warm-container-minimum (n=40, range 10420.6-11136.5 ns)
  10420.6 |########################################
  10456.4 |#####
  10492.2 |##########
  10528.0 |###############
  10563.8 |#####
  10599.6 |####################
  10635.4 |
  10671.2 |
  10707.0 |#####
  10742.8 |
  10778.6 |##########
  10814.4 |
  10850.2 |
  10886.0 |
  10921.7 |##########
  10957.5 |#########################
  10993.3 |
  11029.1 |##########
  11064.9 |#####
  11100.7 |
  (5 below, 3 above range)

warm-container-native (n=40, range 10441.7-11145.8 ns)
  10441.7 |######
  10476.9 |######
  10512.1 |######
  10547.3 |##########################
  10582.5 |########################################
  10617.7 |######
  10652.9 |#############
  10688.1 |######
  10723.3 |#############
  10758.5 |
  10793.7 |##########################
  10828.9 |######
  10864.1 |
  10899.3 |######
  10934.6 |#############
  10969.8 |######
  11005.0 |####################
  11040.2 |######
  11075.4 |######
  11110.6 |
  (5 below, 2 above range)

warm-container-plusone (n=40, range 10501.5-11088.1 ns)
  10501.5 |
  10530.8 |#####
  10560.1 |########################################
  10589.4 |###########
  10618.8 |########################################
  10648.1 |#####
  10677.4 |#####
  10706.8 |###########
  10736.1 |###########
  10765.4 |#####
  10794.8 |#####
  10824.1 |#####
  10853.4 |#####
  10882.8 |
  10912.1 |
  10941.4 |###########
  10970.8 |###########
  11000.1 |
  11029.4 |
  11058.7 |#####
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=33.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=75.8% of algo (FFI overhead may distort results)
- **warm-container-minimum**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.63 (measurement drift or warm-up artifact)
