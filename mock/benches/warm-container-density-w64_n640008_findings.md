# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 50% (significant)

warm-container-kernel is -5.22 us (50%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.1x slower than the field

warm-container-plusone (10.35 us) is 2.1x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-native) are a dead heat (<1%)

warm-container-kernel (4.98 us) and warm-container-native (5.00 us) differ by 0.55%, inside the noise, even though the wider field spreads 108.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.75)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (105% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 105% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-kernel** at 4975.9 ns median (-51.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.08x (fastest 4975.9 ns, slowest 10351.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 10481ns | 10433ns | 10208ns | 10440ns | 10877ns | base |
| warm-container-kernel | 5076ns | 5036ns | 5011ns | 5053ns | 5211ns | -51.57% |
| warm-container-minimum | 5183ns | 5109ns | 5023ns | 5125ns | 5516ns | -50.55% |
| warm-container-native | 5175ns | 5069ns | 5007ns | 5089ns | 5601ns | -50.62% |
| warm-container-plusone | 10470ns | 10451ns | 10221ns | 10432ns | 10835ns | -0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 10382ns | 10118ns | 10767ns | base | 7.101 |
| warm-container-kernel | 5017ns | 4954ns | 5150ns | -51.67% | 14.694 |
| warm-container-minimum | 5100ns | 4961ns | 5351ns | -50.87% | 14.455 |
| warm-container-native | 5112ns | 4946ns | 5532ns | -50.76% | 14.422 |
| warm-container-plusone | 10366ns | 10129ns | 10691ns | -0.16% | 7.113 |

## Performance model

- Peak throughput: **14.906 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 73728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 7.133 | 47.9% |
| warm-container-kernel | 14.817 | 99.4% |
| warm-container-minimum | 14.596 | 97.9% |
| warm-container-native | 14.736 | 98.9% |
| warm-container-plusone | 7.122 | 47.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 10481ns | 10481ns | base |
| warm-container-kernel | 5076ns | 5076ns | -51.57% |
| warm-container-minimum | 5183ns | 5183ns | -50.55% |
| warm-container-native | 5175ns | 5175ns | -50.62% |
| warm-container-plusone | 10470ns | 10470ns | -0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 10336ns | base | --- | [10199, 10490] | --- | --- | --- | --- |
| warm-container-kernel | 4976ns | -5286.7ns (-51.1%) | [-5469, -5185]ns | [4962, 5036] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5051ns | -5244.4ns (-50.7%) | [-5349, -5155]ns | [4983, 5138] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 5003ns | -5224.6ns (-50.5%) | [-5322, -5176]ns | [4974, 5088] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 10351ns | no significant difference | [-117, +65]ns | [10234, 10434] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 10374ns | -51.3% | -51.7% | -52.3% | -1.1% |
| 2 | 10387ns | -50.8% | -51.4% | -52.1% | -2.0% |
| 3 | 10283ns | -50.9% | -51.1% | -51.3% | +1.1% |
| 4 | 10531ns | -51.8% | -52.0% | -52.3% | -4.0% |
| 5 | 11361ns | -54.3% | -55.2% | -55.7% | -10.8% |
| 6 | 10658ns | -51.4% | -51.7% | -50.5% | -4.4% |
| 7 | 10510ns | -52.0% | -50.3% | -50.9% | -1.1% |
| 8 | 10679ns | -53.3% | -49.9% | -52.8% | -2.3% |
| 9 | 10430ns | -52.5% | -50.2% | -48.9% | -1.7% |
| 10 | 10543ns | -52.8% | -51.8% | -52.0% | -0.8% |
| 11 | 10120ns | -51.0% | -50.9% | -44.5% | +0.6% |
| 12 | 10129ns | -51.1% | -50.9% | -44.4% | +0.0% |
| 13 | 10508ns | -52.9% | -52.7% | -46.6% | -3.4% |
| 14 | 10828ns | -53.4% | -54.1% | -48.1% | -6.4% |
| 15 | 10472ns | -52.7% | -51.6% | -46.6% | -2.3% |
| 16 | 10664ns | -53.4% | -47.2% | -52.1% | -2.8% |
| 17 | 10658ns | -53.3% | -50.6% | -53.2% | -1.1% |
| 18 | 10578ns | -53.1% | -53.0% | -53.1% | +1.0% |
| 19 | 10244ns | -51.7% | -51.4% | -51.3% | +4.1% |
| 20 | 10315ns | -51.9% | -51.7% | -50.5% | +3.6% |
| 21 | 10644ns | -52.7% | -53.4% | -47.5% | +0.2% |
| 22 | 10644ns | -52.6% | -53.4% | -52.4% | +0.7% |
| 23 | 10575ns | -52.1% | -53.0% | -51.7% | -0.7% |
| 24 | 10311ns | -50.3% | -47.2% | -50.3% | +0.2% |
| 25 | 10357ns | -49.7% | -51.8% | -50.7% | -1.7% |
| 26 | 10138ns | -48.7% | -48.8% | -50.7% | +3.2% |
| 27 | 10288ns | -50.3% | -49.9% | -51.5% | -1.2% |
| 28 | 10376ns | -52.2% | -47.7% | -52.3% | -1.1% |
| 29 | 10150ns | -51.1% | -48.2% | -51.4% | -0.1% |
| 30 | 10108ns | -50.4% | -48.6% | -51.2% | +4.4% |
| 31 | 10145ns | -51.1% | -48.7% | -51.0% | +2.3% |
| 32 | 10127ns | -51.0% | -49.4% | -51.1% | +7.0% |
| 33 | 10200ns | -51.4% | -50.5% | -51.7% | +3.5% |
| 34 | 10137ns | -51.1% | -50.8% | -51.1% | +5.7% |
| 35 | 10121ns | -51.0% | -49.1% | -51.0% | +3.0% |
| 36 | 10167ns | -51.0% | -51.2% | -51.2% | +0.7% |
| 37 | 10198ns | -51.3% | -51.3% | -51.3% | +0.6% |
| 38 | 10113ns | -51.0% | -50.8% | -50.9% | +3.1% |
| 39 | 10114ns | -51.0% | -51.1% | -50.7% | -0.0% |
| 40 | 10114ns | -50.4% | -49.7% | -50.9% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.542 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.746 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.332 | moderate+ |
| warm-container-native | 0.663 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.552 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 18/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.2ns | 10382.4ns | 0.0% |  |
| warm-container-kernel | 2.4ns | 5017.5ns | 0.0% |  |
| warm-container-minimum | 2.6ns | 5100.5ns | 0.1% |  |
| warm-container-native | 2.5ns | 5112.3ns | 0.0% |  |
| warm-container-plusone | 5.6ns | 10365.7ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 10118.1-10767.1 ns)
  10118.1 |########################################
  10150.6 |#####
  10183.0 |##########
  10215.5 |#####
  10247.9 |
  10280.4 |###############
  10312.8 |#####
  10345.3 |###############
  10377.7 |#####
  10410.2 |#####
  10442.6 |#####
  10475.1 |
  10507.5 |###############
  10540.0 |#####
  10572.4 |##########
  10604.9 |
  10637.3 |#########################
  10669.8 |#####
  10702.2 |
  10734.7 |
  (4 below, 2 above range)

warm-container-kernel (n=40, range 4954.0-5150.5 ns)
   4954.0 |########################################
   4963.8 |#########
   4973.7 |###
   4983.5 |######
   4993.3 |
   5003.1 |
   5013.0 |######
   5022.8 |###
   5032.6 |###
   5042.4 |############
   5052.3 |###
   5062.1 |
   5071.9 |###
   5081.7 |
   5091.6 |
   5101.4 |######
   5111.2 |
   5121.0 |###
   5130.9 |
   5140.7 |
  (4 below, 4 above range)

warm-container-minimum (n=40, range 4961.1-5351.2 ns)
   4961.1 |########################################
   4980.6 |#######
   5000.1 |###
   5019.6 |###
   5039.1 |##########
   5058.7 |###
   5078.2 |##########
   5097.7 |
   5117.2 |###
   5136.7 |#######
   5156.2 |###
   5175.7 |#######
   5195.2 |#######
   5214.7 |###
   5234.2 |
   5253.7 |#######
   5273.2 |
   5292.7 |
   5312.2 |
   5331.7 |
  (3 below, 4 above range)

warm-container-native (n=40, range 4946.4-5532.1 ns)
   4946.4 |########################################
   4975.6 |####################
   5004.9 |###
   5034.2 |######
   5063.5 |######
   5092.8 |################
   5122.1 |
   5151.4 |###
   5180.6 |
   5209.9 |
   5239.2 |
   5268.5 |###
   5297.8 |###
   5327.1 |
   5356.4 |
   5385.6 |
   5414.9 |
   5444.2 |
   5473.5 |
   5502.8 |
  (3 below, 6 above range)

warm-container-plusone (n=40, range 10128.8-10691.5 ns)
  10128.8 |########################################
  10156.9 |################################
  10185.1 |########
  10213.2 |################
  10241.3 |################################
  10269.5 |
  10297.6 |
  10325.7 |########
  10353.9 |################
  10382.0 |################
  10410.1 |########################
  10438.3 |################
  10466.4 |
  10494.5 |########
  10522.7 |################
  10550.8 |########
  10578.9 |
  10607.1 |
  10635.2 |
  10663.3 |################################
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.55 (measurement drift or warm-up artifact)
