# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 65% faster than the next best (warm-clamp-head)

warm-clamp-accfit (34.94 us) leads warm-clamp-head (57.55 us) by 65%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 52% (significant)

warm-clamp-accfit is -37.35 us (52%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 4.2x slower than the field

warm-clamp-minimum (147.92 us) is 4.2x the fastest (34.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows warm-up / thermal drift (autocorr +0.67)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation +0.67, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit} vs {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} (65% apart)

The field splits into a fast tier {warm-clamp-accfit} and a slow tier {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} with a 65% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.2x the fastest

Fastest warm-clamp-accfit (34.94 us) to slowest warm-clamp-minimum (147.92 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 34940.2 ns median (-51.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.23x (fastest 34940.2 ns, slowest 147921.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 72411ns | 71602ns | 68304ns | 71941ns | 77928ns | base |
| warm-clamp-accfit | 35854ns | 35046ns | 34452ns | 35463ns | 38429ns | -50.49% |
| warm-clamp-accfit-dyn | 102494ns | 101941ns | 99845ns | 102087ns | 106367ns | +41.54% |
| warm-clamp-head | 58052ns | 57828ns | 53606ns | 58293ns | 61774ns | -19.83% |
| warm-clamp-min-lanes | 142628ns | 143290ns | 132185ns | 142957ns | 152084ns | +96.97% |
| warm-clamp-minimum | 148137ns | 148072ns | 136720ns | 148489ns | 158499ns | +104.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 72228ns | 68154ns | 77709ns | base | 14.518 |
| warm-clamp-accfit | 35718ns | 34333ns | 38261ns | -50.55% | 29.357 |
| warm-clamp-accfit-dyn | 102262ns | 99642ns | 106129ns | +41.58% | 10.254 |
| warm-clamp-head | 57810ns | 53384ns | 61522ns | -19.96% | 18.138 |
| warm-clamp-min-lanes | 142370ns | 131924ns | 151840ns | +97.11% | 7.365 |
| warm-clamp-minimum | 147938ns | 136456ns | 158326ns | +104.82% | 7.088 |

## Performance model

- Peak throughput: **30.542 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 14.702 | 48.1% |
| warm-clamp-accfit | 30.011 | 98.3% |
| warm-clamp-accfit-dyn | 10.313 | 33.8% |
| warm-clamp-head | 18.219 | 59.7% |
| warm-clamp-min-lanes | 7.337 | 24.0% |
| warm-clamp-minimum | 7.089 | 23.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 72411ns | 72411ns | base |
| warm-clamp-accfit | 35854ns | 35854ns | -50.49% |
| warm-clamp-accfit-dyn | 102494ns | 102494ns | +41.54% |
| warm-clamp-head | 58052ns | 58052ns | -19.83% |
| warm-clamp-min-lanes | 142628ns | 142628ns | +96.97% |
| warm-clamp-minimum | 148137ns | 148137ns | +104.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 71323ns | base | --- | [69568, 73831] | --- | --- | --- | --- |
| warm-clamp-accfit | 34940ns | -35832.9ns (-50.2%) | [-38697, -33707]ns | [34662, 35800] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 101678ns | +30335.0ns (+42.5%) | [+29132, +31278]ns | [100873, 102467] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 57554ns | -14424.4ns (-20.2%) | [-15004, -13067]ns | [56888, 59418] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 142913ns | +70496.0ns (+98.8%) | [+65765, +75338]ns | [139280, 146789] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 147922ns | +75708.1ns (+106.1%) | [+70350, +79844]ns | [143589, 152664] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 69208ns | -49.9% | +45.1% | -23.3% | +95.3% | +102.8% |
| 2 | 69703ns | -51.3% | +46.5% | -23.9% | +109.1% | +92.5% |
| 3 | 68246ns | -49.5% | +45.7% | -19.8% | +119.6% | +97.5% |
| 4 | 69842ns | -50.1% | +44.0% | -17.6% | +107.3% | +93.8% |
| 5 | 70422ns | -49.3% | +43.7% | -16.5% | +105.7% | +93.1% |
| 6 | 69433ns | -45.1% | +43.4% | -21.7% | +102.1% | +97.0% |
| 7 | 75338ns | -51.9% | +36.4% | -24.5% | +74.2% | +88.6% |
| 8 | 73672ns | -52.6% | +34.7% | -29.5% | +77.2% | +85.5% |
| 9 | 72025ns | -46.6% | +41.2% | -20.1% | +81.6% | +92.1% |
| 10 | 79110ns | -55.4% | +26.6% | -24.6% | +64.9% | +76.4% |
| 11 | 68355ns | -43.2% | +47.0% | -18.0% | +102.9% | +111.6% |
| 12 | 70328ns | -43.9% | +44.5% | -20.9% | +110.4% | +108.7% |
| 13 | 68213ns | -46.8% | +49.3% | -16.5% | +118.4% | +109.8% |
| 14 | 72706ns | -52.2% | +39.6% | -17.8% | +103.9% | +101.6% |
| 15 | 74069ns | -53.9% | +38.3% | -17.7% | +102.4% | +106.3% |
| 16 | 69326ns | -48.5% | +45.1% | -12.3% | +120.8% | +125.2% |
| 17 | 67982ns | -45.2% | +46.6% | -10.9% | +118.2% | +135.2% |
| 18 | 69010ns | -44.5% | +53.4% | -11.3% | +95.9% | +128.2% |
| 19 | 69032ns | -47.8% | +55.7% | -17.8% | +90.2% | +128.0% |
| 20 | 67953ns | -49.4% | +55.7% | -15.4% | +92.4% | +135.8% |
| 21 | 72679ns | -53.2% | +41.0% | -19.4% | +96.8% | +104.9% |
| 22 | 68229ns | -48.2% | +46.1% | -11.6% | +101.3% | +121.4% |
| 23 | 69277ns | -48.4% | +47.6% | -15.8% | +101.9% | +120.4% |
| 24 | 73990ns | -51.5% | +34.5% | -22.3% | +85.9% | +109.9% |
| 25 | 76922ns | -51.1% | +30.2% | -18.7% | +82.1% | +91.8% |
| 26 | 77370ns | -50.9% | +29.1% | -26.5% | +84.5% | +97.2% |
| 27 | 77220ns | -51.6% | +31.0% | -19.5% | +84.0% | +102.4% |
| 28 | 70880ns | -50.7% | +49.3% | -20.6% | +109.1% | +120.3% |
| 29 | 68161ns | -49.3% | +48.7% | -21.8% | +117.7% | +123.7% |
| 30 | 68096ns | -45.3% | +51.1% | -21.9% | +117.5% | +124.2% |
| 31 | 70261ns | -50.5% | +49.3% | -18.5% | +93.8% | +103.9% |
| 32 | 75792ns | -54.3% | +40.1% | -20.7% | +78.9% | +108.7% |
| 33 | 75930ns | -54.3% | +38.4% | -17.9% | +78.5% | +106.2% |
| 34 | 78981ns | -56.2% | +35.8% | -28.8% | +77.9% | +82.2% |
| 35 | 74845ns | -53.8% | +40.5% | -28.6% | +94.5% | +104.2% |
| 36 | 79855ns | -56.7% | +29.1% | -25.9% | +82.2% | +85.7% |
| 37 | 72680ns | -52.5% | +40.1% | -16.5% | +103.4% | +120.9% |
| 38 | 71766ns | -51.8% | +39.5% | -18.9% | +108.7% | +95.7% |
| 39 | 76039ns | -54.5% | +38.8% | -19.7% | +109.0% | +92.2% |
| 40 | 76176ns | -54.6% | +33.5% | -19.8% | +104.6% | +87.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.448 | moderate+ |
| warm-clamp-accfit | 0.430 | moderate+ |
| warm-clamp-accfit-dyn | 0.443 | moderate+ |
| warm-clamp-head | 0.336 | moderate+ |
| warm-clamp-min-lanes | 0.675 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.657 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 6.0ns | 72228.0ns | 0.0% |  |
| warm-clamp-accfit | 3.6ns | 35717.7ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.9ns | 102262.5ns | 0.0% |  |
| warm-clamp-head | 4.3ns | 57809.7ns | 0.0% |  |
| warm-clamp-min-lanes | 3.9ns | 142369.6ns | 0.0% |  |
| warm-clamp-minimum | 4.6ns | 147937.9ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 68154.4-77709.2 ns)
  68154.4 |########################################
  68632.2 |################
  69109.9 |################################
  69587.6 |################
  70065.4 |########################
  70543.1 |########
  71020.9 |
  71498.6 |########
  71976.3 |########
  72454.1 |########################
  72931.8 |
  73409.6 |########
  73887.3 |################
  74365.0 |
  74842.8 |########
  75320.5 |################
  75798.3 |########################
  76276.0 |
  76753.7 |################
  77231.5 |########
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 34332.8-38260.5 ns)
  34332.8 |#############
  34529.2 |########################################
  34725.5 |#################
  34921.9 |########
  35118.3 |####
  35314.7 |####
  35511.1 |########
  35707.5 |########
  35903.9 |####
  36100.3 |####
  36296.6 |####
  36493.0 |
  36689.4 |
  36885.8 |
  37082.2 |########
  37278.6 |####
  37475.0 |####
  37671.4 |
  37867.7 |####
  38064.1 |####
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 99641.7-106129.4 ns)
  99641.7 |########################
  99966.1 |########################
  100290.5 |################################
  100614.9 |
  100939.2 |################
  101263.6 |################
  101588.0 |########################################
  101912.4 |########
  102236.8 |########################
  102561.2 |########
  102885.5 |################
  103209.9 |
  103534.3 |
  103858.7 |
  104183.1 |
  104507.5 |
  104831.8 |########################
  105156.2 |
  105480.6 |########
  105805.0 |########################
  (4 below, 3 above range)

warm-clamp-head (n=40, range 53383.8-61522.2 ns)
  53383.8 |##########
  53790.7 |
  54197.7 |##########
  54604.6 |##########
  55011.5 |
  55418.4 |##########
  55825.3 |####################
  56232.3 |##########
  56639.2 |########################################
  57046.1 |##########
  57453.0 |########################################
  57859.9 |##########
  58266.9 |####################
  58673.8 |##########
  59080.7 |##########
  59487.6 |####################
  59894.5 |##########
  60301.5 |##############################
  60708.4 |########################################
  61115.3 |##########
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 131923.9-151839.9 ns)
  131923.9 |
  132919.7 |
  133915.5 |
  134911.3 |##########################
  135907.1 |######
  136902.9 |#############
  137898.7 |######
  138894.5 |
  139890.3 |##########################
  140886.1 |
  141881.9 |#############
  142877.7 |######
  143873.5 |######
  144869.3 |##########################
  145865.1 |
  146860.9 |######
  147856.7 |########################################
  148852.5 |#############
  149848.3 |#############
  150844.1 |
  (6 below, 3 above range)

warm-clamp-minimum (n=40, range 136456.2-158326.0 ns)
  136456.2 |#############
  137549.7 |######
  138643.2 |######
  139736.7 |#############
  140830.2 |
  141923.7 |#############
  143017.1 |####################
  144110.6 |######
  145204.1 |######
  146297.6 |#############
  147391.1 |#############
  148484.6 |######
  149578.1 |
  150671.6 |######
  151765.1 |########################################
  152858.5 |
  153952.0 |
  155045.5 |####################
  156139.0 |#############
  157232.5 |####################
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-min-lanes**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.66 (measurement drift or warm-up artifact)
