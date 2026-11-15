# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 54% faster than the next best (warm-clamp-head)

warm-clamp-accfit (34.90 us) leads warm-clamp-head (53.85 us) by 54%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 49% (significant)

warm-clamp-accfit is -34.35 us (49%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 4.3x slower than the field

warm-clamp-minimum (150.85 us) is 4.3x the fastest (34.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.70)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit} vs {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} (54% apart)

The field splits into a fast tier {warm-clamp-accfit} and a slow tier {warm-clamp-head, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest warm-clamp-accfit (34.90 us) to slowest warm-clamp-minimum (150.85 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 34901.3 ns median (-50.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.32x (fastest 34901.3 ns, slowest 150849.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 71234ns | 70329ns | 68389ns | 70771ns | 75467ns | base |
| warm-clamp-accfit | 35378ns | 35026ns | 34111ns | 35193ns | 37198ns | -50.34% |
| warm-clamp-accfit-dyn | 93829ns | 93262ns | 91852ns | 93341ns | 97271ns | +31.72% |
| warm-clamp-head | 54520ns | 54062ns | 50660ns | 54233ns | 59241ns | -23.46% |
| warm-clamp-min-lanes | 141928ns | 139961ns | 133976ns | 140606ns | 153846ns | +99.24% |
| warm-clamp-minimum | 150381ns | 151138ns | 137877ns | 149481ns | 165586ns | +111.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 71042ns | 68196ns | 75234ns | base | 14.760 |
| warm-clamp-accfit | 35236ns | 33996ns | 37015ns | -50.40% | 29.759 |
| warm-clamp-accfit-dyn | 93654ns | 91634ns | 97108ns | +31.83% | 11.196 |
| warm-clamp-head | 54328ns | 50490ns | 59025ns | -23.53% | 19.301 |
| warm-clamp-min-lanes | 141660ns | 133700ns | 153587ns | +99.40% | 7.402 |
| warm-clamp-minimum | 150108ns | 137611ns | 165408ns | +111.30% | 6.985 |

## Performance model

- Peak throughput: **30.844 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 14.938 | 48.4% |
| warm-clamp-accfit | 30.044 | 97.4% |
| warm-clamp-accfit-dyn | 11.263 | 36.5% |
| warm-clamp-head | 19.471 | 63.1% |
| warm-clamp-min-lanes | 7.514 | 24.4% |
| warm-clamp-minimum | 6.951 | 22.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 71234ns | 71234ns | base |
| warm-clamp-accfit | 35378ns | 35378ns | -50.34% |
| warm-clamp-accfit-dyn | 93829ns | 93829ns | +31.72% |
| warm-clamp-head | 54520ns | 54520ns | -23.46% |
| warm-clamp-min-lanes | 141928ns | 141928ns | +99.24% |
| warm-clamp-minimum | 150381ns | 150381ns | +111.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 70198ns | base | --- | [69288, 72004] | --- | --- | --- | --- |
| warm-clamp-accfit | 34901ns | -34893.3ns (-49.7%) | [-36753, -34495]ns | [34454, 35811] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 93098ns | +23288.3ns (+33.2%) | [+21533, +23894]ns | [92772, 93437] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 53855ns | -17530.4ns (-25.0%) | [-18427, -15790]ns | [52421, 55966] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 139550ns | +67932.4ns (+96.8%) | [+65291, +70391]ns | [136872, 144175] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 150849ns | +80607.4ns (+114.8%) | [+76247, +82636]ns | [144272, 154453] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 74419ns | -52.5% | +25.4% | -24.4% | +86.8% | +86.4% |
| 2 | 74583ns | -52.0% | +24.2% | -29.7% | +83.5% | +86.3% |
| 3 | 69965ns | -45.8% | +34.2% | -27.4% | +90.6% | +97.5% |
| 4 | 71679ns | -49.2% | +29.4% | -29.7% | +88.4% | +91.3% |
| 5 | 73341ns | -52.2% | +27.4% | -29.6% | +91.7% | +86.8% |
| 6 | 77252ns | -50.1% | +19.3% | -27.0% | +82.7% | +83.5% |
| 7 | 77406ns | -52.6% | +18.3% | -26.1% | +88.6% | +99.6% |
| 8 | 68867ns | -50.6% | +34.0% | -13.4% | +110.4% | +106.1% |
| 9 | 68477ns | -50.4% | +38.2% | -23.3% | +93.8% | +120.4% |
| 10 | 69727ns | -47.4% | +43.3% | -23.8% | +90.2% | +123.1% |
| 11 | 74901ns | -54.7% | +31.8% | -26.7% | +93.7% | +112.9% |
| 12 | 75559ns | -52.7% | +22.4% | -22.9% | +111.8% | +115.7% |
| 13 | 73410ns | -53.1% | +24.8% | -18.7% | +89.9% | +96.0% |
| 14 | 67978ns | -49.9% | +38.5% | -8.2% | +112.3% | +128.3% |
| 15 | 69448ns | -51.2% | +31.8% | -17.9% | +100.9% | +133.9% |
| 16 | 72186ns | -53.0% | +26.8% | -24.6% | +84.4% | +130.1% |
| 17 | 70795ns | -49.4% | +29.1% | -19.1% | +97.1% | +134.8% |
| 18 | 73190ns | -52.8% | +27.7% | -21.3% | +87.9% | +125.9% |
| 19 | 69128ns | -50.2% | +34.6% | -16.9% | +102.2% | +147.2% |
| 20 | 68241ns | -49.6% | +37.5% | -12.3% | +98.6% | +149.0% |
| 21 | 68065ns | -47.3% | +37.3% | -26.2% | +101.1% | +111.2% |
| 22 | 69227ns | -48.0% | +34.4% | -27.2% | +95.4% | +111.3% |
| 23 | 69349ns | -50.0% | +34.5% | -26.1% | +96.7% | +117.4% |
| 24 | 68950ns | -50.0% | +34.7% | -19.8% | +94.8% | +119.0% |
| 25 | 68126ns | -49.4% | +36.3% | -24.7% | +95.8% | +126.5% |
| 26 | 70579ns | -49.2% | +31.7% | -20.5% | +91.4% | +119.3% |
| 27 | 72608ns | -52.6% | +28.4% | -30.5% | +87.8% | +106.0% |
| 28 | 72141ns | -51.8% | +30.3% | -26.8% | +90.1% | +89.3% |
| 29 | 71918ns | -50.3% | +32.8% | -24.8% | +89.5% | +92.7% |
| 30 | 71207ns | -49.6% | +32.7% | -19.6% | +107.7% | +92.6% |
| 31 | 70171ns | -50.1% | +30.2% | -26.6% | +116.1% | +109.6% |
| 32 | 69425ns | -48.2% | +33.6% | -24.5% | +125.0% | +101.1% |
| 33 | 69007ns | -50.4% | +33.1% | -21.8% | +126.3% | +99.2% |
| 34 | 67833ns | -49.7% | +39.0% | -23.1% | +129.2% | +113.3% |
| 35 | 68382ns | -50.1% | +44.0% | -26.4% | +115.2% | +118.3% |
| 36 | 68468ns | -46.2% | +35.0% | -22.1% | +113.0% | +120.5% |
| 37 | 69004ns | -46.5% | +35.0% | -27.5% | +108.0% | +119.7% |
| 38 | 74344ns | -54.0% | +24.8% | -27.7% | +93.8% | +108.0% |
| 39 | 72090ns | -52.7% | +31.7% | -22.5% | +108.4% | +114.7% |
| 40 | 70224ns | -48.5% | +42.7% | -26.9% | +115.1% | +119.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.491 | moderate+ |
| warm-clamp-accfit | 0.160 | ok |
| warm-clamp-accfit-dyn | 0.312 | moderate+ |
| warm-clamp-head | 0.408 | moderate+ |
| warm-clamp-min-lanes | 0.631 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.696 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 4.0ns | 71041.7ns | 0.0% |  |
| warm-clamp-accfit | 3.4ns | 35235.8ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.0ns | 93654.2ns | 0.0% |  |
| warm-clamp-head | 4.8ns | 54328.0ns | 0.0% |  |
| warm-clamp-min-lanes | 4.8ns | 141660.3ns | 0.0% |  |
| warm-clamp-minimum | 5.0ns | 150108.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 68196.1-75234.3 ns)
  68196.1 |################################
  68548.0 |########
  68900.0 |########################################
  69251.9 |########################
  69603.8 |########
  69955.7 |########################
  70307.6 |########
  70659.5 |########
  71011.4 |########
  71363.3 |########
  71715.2 |########
  72067.1 |########################
  72419.1 |########
  72771.0 |
  73122.9 |########################
  73474.8 |
  73826.7 |
  74178.6 |################
  74530.5 |########
  74882.4 |########
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 33996.4-37014.9 ns)
  33996.4 |########################################
  34147.3 |################
  34298.3 |################
  34449.2 |########################################
  34600.1 |########
  34751.0 |########
  34902.0 |########
  35052.9 |########
  35203.8 |
  35354.8 |########
  35505.7 |
  35656.6 |########################
  35807.5 |########################################
  35958.5 |########
  36109.4 |########
  36260.3 |########
  36411.2 |
  36562.2 |################
  36713.1 |########
  36864.0 |########
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 91634.0-97108.1 ns)
  91634.0 |######
  91907.7 |######
  92181.4 |#############
  92455.1 |#############
  92728.8 |########################################
  93002.5 |##########################
  93276.2 |#################################
  93549.9 |######
  93823.6 |#############
  94097.3 |#############
  94371.0 |#############
  94644.7 |
  94918.4 |######
  95192.1 |
  95465.9 |######
  95739.6 |
  96013.3 |
  96287.0 |
  96560.7 |
  96834.4 |
  (6 below, 4 above range)

warm-clamp-head (n=40, range 50489.7-59024.8 ns)
  50489.7 |####################
  50916.4 |##############################
  51343.2 |####################
  51769.9 |##########
  52196.7 |##############################
  52623.5 |##########
  53050.2 |####################
  53477.0 |##########
  53903.7 |####################
  54330.5 |##########
  54757.2 |##########
  55184.0 |##########
  55610.7 |##########
  56037.5 |##############################
  56464.2 |
  56891.0 |########################################
  57317.8 |####################
  57744.5 |
  58171.3 |##########
  58598.0 |
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 133699.7-153587.3 ns)
  133699.7 |##########
  134694.1 |########################################
  135688.5 |##############################
  136682.9 |########################################
  137677.2 |
  138671.6 |########################################
  139666.0 |####################
  140660.4 |##########
  141654.8 |
  142649.1 |##########
  143643.5 |####################
  144637.9 |####################
  145632.3 |####################
  146626.6 |##########
  147621.0 |##########
  148615.4 |
  149609.8 |##########
  150604.2 |##########
  151598.5 |##########
  152592.9 |
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 137610.9-165408.4 ns)
  137610.9 |####################
  139000.7 |#####
  140390.6 |#####
  141780.5 |#####
  143170.4 |##########
  144560.2 |#####
  145950.1 |##########
  147340.0 |
  148729.9 |##########
  150119.8 |####################
  151509.6 |#####
  152899.5 |
  154289.4 |########################################
  155679.3 |
  157069.2 |
  158459.0 |#####
  159848.9 |
  161238.8 |#####
  162628.7 |#####
  164018.6 |#####
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-min-lanes**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.70 (measurement drift or warm-up artifact)
