# Layout::Bitpacked decode against per-element consumer work

5 variants, 40 samples per variant.
Baseline: **bitpack-mac-naive**

## Highlights

Baseline for all deltas below: **bitpack-mac-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-mac-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-mac-naive has the worst median (126.43 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-mac-native at 51.80 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-mac-native dominates: 28% faster than the next best (bitpack-mac-simd)

bitpack-mac-native (51.80 us) leads bitpack-mac-simd (66.47 us) by 28%, a clear separation rather than a photo finish. CV 10.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-mac-native beats baseline by 62% (significant)

bitpack-mac-native is -78.23 us (62%) faster than baseline bitpack-mac-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-mac-naive is an outlier: 2.4x slower than the field

bitpack-mac-naive (126.43 us) is 2.4x the fastest (51.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-mac-native is fastest but the noisiest (CV 10.4%)

bitpack-mac-native wins on median (51.80 us) yet has the highest variance (CV 10.4%), while bitpack-mac-windowed is the steadiest (CV 2.5%, 105.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-mac-naive shows warm-up / thermal drift (autocorr +0.50)

bitpack-mac-naive's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-mac-native, bitpack-mac-simd} vs {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} (58% apart)

The field splits into a fast tier {bitpack-mac-native, bitpack-mac-simd} and a slow tier {bitpack-mac-narrow, bitpack-mac-windowed, bitpack-mac-naive} with a 58% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-mac-native** at 51798.3 ns median (-59.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.44x (fastest 51798.3 ns, slowest 126434.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-mac-naive | 127729ns | 126864ns | 122721ns | 127429ns | 133636ns | base |
| bitpack-mac-narrow | 107520ns | 105579ns | 103040ns | 105807ns | 117141ns | -15.82% |
| bitpack-mac-native | 53201ns | 51978ns | 49687ns | 52178ns | 59783ns | -58.35% |
| bitpack-mac-simd | 67344ns | 66739ns | 65694ns | 66941ns | 70204ns | -47.28% |
| bitpack-mac-windowed | 105448ns | 106064ns | 101556ns | 105588ns | 108921ns | -17.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-mac-naive | 127400ns | 122407ns | 133315ns | base | 2.058 |
| bitpack-mac-narrow | 107232ns | 102775ns | 116818ns | -15.83% | 2.445 |
| bitpack-mac-native | 53010ns | 49537ns | 59581ns | -58.39% | 4.945 |
| bitpack-mac-simd | 67123ns | 65477ns | 69948ns | -47.31% | 3.905 |
| bitpack-mac-windowed | 105147ns | 101248ns | 108609ns | -17.47% | 2.493 |

## Performance model

- Peak throughput: **5.292 Gops/s** (bitpack-mac-native; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-mac-naive | 2.073 | 39.2% |
| bitpack-mac-narrow | 2.489 | 47.0% |
| bitpack-mac-native | 5.061 | 95.6% |
| bitpack-mac-simd | 3.944 | 74.5% |
| bitpack-mac-windowed | 2.478 | 46.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-mac-naive | 127729ns | 127729ns | base |
| bitpack-mac-narrow | 107520ns | 107520ns | -15.82% |
| bitpack-mac-native | 53201ns | 53201ns | -58.35% |
| bitpack-mac-simd | 67344ns | 67344ns | -47.28% |
| bitpack-mac-windowed | 105448ns | 105448ns | -17.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-mac-naive | 126434ns | base | --- | [124854, 128995] | --- | --- | --- | --- |
| bitpack-mac-narrow | 105323ns | -20330.2ns (-16.1%) | [-22909, -19219]ns | [105057, 105820] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-native | 51798ns | -73598.5ns (-58.2%) | [-76910, -72472]ns | [51510, 52161] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-simd | 66475ns | -59640.6ns (-47.2%) | [-61265, -57751]ns | [66381, 66958] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-mac-windowed | 105782ns | -22121.2ns (-17.5%) | [-24075, -18319]ns | [104757, 106199] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-mac-naive | bitpack-mac-narrow | bitpack-mac-native | bitpack-mac-simd | bitpack-mac-windowed |
|---|---|---|---|---|---|
| 1 | 126150ns | -14.5% | -58.2% | -46.5% | -14.5% |
| 2 | 127657ns | -15.5% | -56.9% | -46.9% | -13.7% |
| 3 | 128690ns | -8.8% | -58.6% | -47.4% | -17.8% |
| 4 | 124602ns | -10.9% | -56.5% | -45.3% | -15.1% |
| 5 | 128216ns | -13.9% | -59.5% | -46.8% | -14.1% |
| 6 | 132369ns | -23.4% | -60.6% | -46.5% | -19.9% |
| 7 | 134606ns | -21.9% | -61.8% | -47.8% | -20.8% |
| 8 | 128868ns | -18.5% | -59.9% | -46.6% | -17.9% |
| 9 | 124522ns | -15.5% | -53.9% | -40.2% | -13.1% |
| 10 | 124419ns | +29.5% | -57.6% | -46.5% | -14.2% |
| 11 | 120009ns | -16.0% | -60.4% | -44.8% | -12.8% |
| 12 | 125395ns | -16.2% | -58.7% | -47.1% | -18.3% |
| 13 | 120170ns | -12.5% | -60.1% | -44.7% | -15.2% |
| 14 | 119709ns | -12.3% | -57.1% | -44.5% | -15.1% |
| 15 | 123890ns | -15.2% | -58.6% | -46.4% | -14.2% |
| 16 | 124010ns | -15.3% | -61.8% | -46.5% | -12.4% |
| 17 | 123943ns | -15.1% | -58.4% | -46.5% | -13.9% |
| 18 | 123825ns | -18.6% | -58.4% | -46.4% | -17.9% |
| 19 | 123850ns | -14.4% | -58.4% | -45.3% | -14.6% |
| 20 | 126718ns | -15.4% | -59.4% | -47.5% | -18.8% |
| 21 | 125329ns | -16.2% | -56.2% | -47.0% | -15.3% |
| 22 | 123858ns | -15.2% | -57.1% | -46.0% | -15.4% |
| 23 | 129115ns | -18.6% | -56.3% | -48.6% | -20.8% |
| 24 | 126060ns | -16.7% | -50.3% | -47.0% | -17.5% |
| 25 | 124328ns | -15.5% | -57.0% | -46.0% | -13.5% |
| 26 | 128874ns | -17.5% | -36.0% | -48.5% | -23.7% |
| 27 | 125798ns | -16.1% | -58.7% | -47.2% | -16.2% |
| 28 | 130784ns | -19.4% | -59.4% | -46.1% | -22.1% |
| 29 | 132094ns | -16.0% | -60.8% | -49.7% | -23.1% |
| 30 | 134072ns | -21.2% | -61.2% | -50.5% | -21.3% |
| 31 | 123949ns | -18.4% | -58.5% | -46.5% | -11.8% |
| 32 | 130555ns | -21.1% | -60.5% | -50.8% | -18.6% |
| 33 | 129433ns | -16.7% | -59.9% | -50.1% | -18.3% |
| 34 | 125107ns | -14.6% | -58.8% | -45.5% | -14.4% |
| 35 | 133395ns | -20.5% | -60.9% | -49.7% | -20.3% |
| 36 | 130837ns | -19.5% | -60.6% | -49.2% | -18.9% |
| 37 | 130800ns | -19.3% | -60.6% | -49.2% | -19.2% |
| 38 | 133416ns | -21.2% | -63.4% | -51.9% | -23.9% |
| 39 | 131094ns | -19.6% | -61.5% | -50.0% | -22.6% |
| 40 | 135476ns | -21.8% | -61.9% | -50.9% | -23.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-mac-naive | 0.504 | HIGH+ (drift/warm-up) |
| bitpack-mac-narrow | -0.082 | ok |
| bitpack-mac-native | 0.083 | ok |
| bitpack-mac-simd | 0.355 | moderate+ |
| bitpack-mac-windowed | 0.257 | moderate+ |

**Consistency summary:**

- **bitpack-mac-narrow**: won 39/40, lost 1/40
- **bitpack-mac-native**: won 40/40, lost 0/40
- **bitpack-mac-simd**: won 40/40, lost 0/40
- **bitpack-mac-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-mac-naive | 4.7ns | 127399.8ns | 0.0% |  |
| bitpack-mac-narrow | 6.5ns | 107232.4ns | 0.0% |  |
| bitpack-mac-native | 4.5ns | 53010.4ns | 0.0% |  |
| bitpack-mac-simd | 7.6ns | 67122.6ns | 0.0% |  |
| bitpack-mac-windowed | 5.9ns | 105147.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-mac-naive (n=40, range 122406.7-133315.3 ns)
  122406.7 |
  122952.1 |
  123497.5 |########################################
  124043.0 |#################
  124588.4 |###########
  125133.8 |###########
  125679.3 |#################
  126224.7 |#####
  126770.1 |
  127315.6 |#####
  127861.0 |#####
  128406.4 |#################
  128951.9 |###########
  129497.3 |
  130042.7 |#####
  130588.2 |######################
  131133.6 |
  131679.0 |#####
  132224.4 |#####
  132769.9 |
  (3 below, 5 above range)

bitpack-mac-narrow (n=40, range 102775.3-116818.2 ns)
  102775.3 |##
  103477.5 |
  104179.6 |
  104881.8 |########################################
  105583.9 |##########
  106286.1 |####
  106988.2 |##
  107690.4 |######
  108392.5 |
  109094.6 |
  109796.8 |##
  110498.9 |####
  111201.1 |
  111903.2 |
  112605.4 |
  113307.5 |
  114009.7 |
  114711.8 |
  115413.9 |
  116116.1 |
  (4 below, 2 above range)

bitpack-mac-native (n=40, range 49536.7-59581.3 ns)
  49536.7 |
  50038.9 |###
  50541.1 |
  51043.4 |########################################
  51545.6 |################################
  52047.8 |#######
  52550.1 |#######
  53052.3 |##############
  53554.5 |
  54056.8 |###
  54559.0 |#######
  55061.2 |
  55563.4 |
  56065.7 |###
  56567.9 |
  57070.1 |###
  57572.4 |
  58074.6 |
  58576.8 |
  59079.1 |
  (4 below, 2 above range)

bitpack-mac-simd (n=40, range 65477.1-69947.8 ns)
  65477.1 |##
  65700.6 |
  65924.1 |
  66147.7 |#################
  66371.2 |########################################
  66594.7 |
  66818.3 |#####
  67041.8 |#####
  67265.3 |##
  67488.9 |
  67712.4 |########
  67935.9 |
  68159.5 |########
  68383.0 |
  68606.5 |##
  68830.1 |
  69053.6 |
  69277.2 |
  69500.7 |
  69724.2 |
  (3 below, 4 above range)

bitpack-mac-windowed (n=40, range 101248.0-108608.8 ns)
  101248.0 |######################
  101616.0 |#################
  101984.1 |#####
  102352.1 |#####
  102720.2 |#####
  103088.2 |#####
  103456.2 |
  103824.3 |#####
  104192.3 |
  104560.4 |###########
  104928.4 |
  105296.4 |###########
  105664.5 |########################################
  106032.5 |############################
  106400.6 |#################
  106768.6 |#####
  107136.6 |
  107504.7 |###########
  107872.7 |
  108240.8 |###########
  (1 below, 3 above range)

```

## Diagnostics

- **bitpack-mac-naive**: autocorrelation=0.50 (measurement drift or warm-up artifact)
