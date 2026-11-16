# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 42% (significant)

warm-clamp-accfit is -52.19 us (42%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.4x slower than the field

warm-clamp-min-lanes (169.67 us) is 2.4x the fastest (70.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-accfit, warm-clamp-accfit-dyn) are a dead heat (<1%)

warm-clamp-accfit (70.68 us) and warm-clamp-accfit-dyn (70.97 us) differ by 0.41%, inside the noise, even though the wider field spreads 140.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: warm-clamp-accfit** at 70683.2 ns median (-42.5% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.40x (fastest 70683.2 ns, slowest 169673.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112826587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 121403ns | 123036ns | 118842ns | 121610ns | 123342ns | base |
| warm-clamp-accfit | 69780ns | 70853ns | 64687ns | 70842ns | 71687ns | -42.52% |
| warm-clamp-accfit-dyn | 71069ns | 71123ns | 70751ns | 71099ns | 71296ns | -41.46% |
| warm-clamp-head | 70228ns | 72053ns | 63463ns | 71360ns | 73597ns | -42.15% |
| warm-clamp-min-lanes | 169637ns | 169843ns | 168312ns | 169593ns | 171093ns | +39.73% |
| warm-clamp-minimum | 167452ns | 167407ns | 167333ns | 167419ns | 167672ns | +37.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 121246ns | 118686ns | 123182ns | base | 8.648 |
| warm-clamp-accfit | 69620ns | 64529ns | 71531ns | -42.58% | 15.061 |
| warm-clamp-accfit-dyn | 70914ns | 70601ns | 71137ns | -41.51% | 14.787 |
| warm-clamp-head | 70065ns | 63312ns | 73439ns | -42.21% | 14.966 |
| warm-clamp-min-lanes | 169448ns | 168113ns | 170893ns | +39.75% | 6.188 |
| warm-clamp-minimum | 167245ns | 167151ns | 167437ns | +37.94% | 6.270 |

## Performance model

- Peak throughput: **16.562 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.533 | 51.5% |
| warm-clamp-accfit | 14.835 | 89.6% |
| warm-clamp-accfit-dyn | 14.775 | 89.2% |
| warm-clamp-head | 14.585 | 88.1% |
| warm-clamp-min-lanes | 6.180 | 37.3% |
| warm-clamp-minimum | 6.271 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 121403ns | 121403ns | base |
| warm-clamp-accfit | 69780ns | 69780ns | -42.52% |
| warm-clamp-accfit-dyn | 71069ns | 71069ns | -41.46% |
| warm-clamp-head | 70228ns | 70228ns | -42.15% |
| warm-clamp-min-lanes | 169637ns | 169637ns | +39.73% |
| warm-clamp-minimum | 167452ns | 167452ns | +37.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 122885ns | base | --- | [118775, 122909] | --- | --- | --- | --- |
| warm-clamp-accfit | 70683ns | -52186.3ns (-42.5%) | [-52354, -48016]ns | [70635, 70725] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 70971ns | -51979.9ns (-42.3%) | [-52047, -47813]ns | [70950, 70979] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 71894ns | -51002.2ns (-41.5%) | [-52880, -45508]ns | [70226, 73241] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 169673ns | +47708.1ns (+38.8%) | [+46759, +49324]ns | [169310, 169688] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 167210ns | +44363.3ns (+36.1%) | [+44303, +48417]ns | [167185, 167233] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 118692ns | -40.4% | -40.2% | -38.3% | +41.7% | +40.8% |
| 1 | 2 | 122894ns | -47.5% | -42.5% | -48.5% | +37.8% | +36.0% |
| 1 | 3 | 118860ns | -40.5% | -40.3% | -38.2% | +42.8% | +40.7% |
| 1 | 4 | 122988ns | -42.5% | -42.3% | -42.8% | +38.0% | +36.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.733 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.425 | moderate- |
| warm-clamp-accfit-dyn | -0.492 | moderate- |
| warm-clamp-head | -0.641 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | 0.080 | ok |
| warm-clamp-minimum | -0.026 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 30.1ns | 121246.4ns | 0.0% |  |
| warm-clamp-accfit | 28.1ns | 69620.3ns | 0.0% |  |
| warm-clamp-accfit-dyn | 27.7ns | 70914.2ns | 0.0% |  |
| warm-clamp-head | 34.1ns | 70065.0ns | 0.0% |  |
| warm-clamp-min-lanes | 20.0ns | 169447.7ns | 0.0% |  |
| warm-clamp-minimum | 20.8ns | 167245.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 118685.6-123182.0 ns)
  118685.6 |#####################################
  118910.4 |##
  119135.2 |
  119360.1 |
  119584.9 |
  119809.7 |
  120034.5 |
  120259.3 |
  120484.2 |
  120709.0 |
  120933.8 |
  121158.6 |##
  121383.4 |
  121608.3 |
  121833.1 |
  122057.9 |
  122282.7 |
  122507.5 |
  122732.4 |########################################
  122957.2 |#################
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 64528.8-71531.2 ns)
  64528.8 |#
  64878.9 |#
  65229.0 |
  65579.1 |
  65929.2 |
  66279.4 |
  66629.5 |
  66979.6 |
  67329.7 |
  67679.9 |
  68030.0 |
  68380.1 |
  68730.2 |
  69080.4 |
  69430.5 |
  69780.6 |
  70130.7 |
  70480.9 |########################################
  70831.0 |##
  71181.1 |
  (6 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 70601.0-71136.9 ns)
  70601.0 |#############
  70627.8 |####
  70654.6 |
  70681.4 |####
  70708.2 |
  70735.0 |
  70761.8 |
  70788.6 |
  70815.4 |
  70842.2 |
  70869.0 |
  70895.8 |
  70922.6 |#################
  70949.4 |########################################
  70976.2 |###############################
  71003.0 |#################
  71029.8 |####
  71056.5 |####
  71083.3 |########
  71110.1 |
  (5 below, 2 above range)

warm-clamp-head (n=40, range 63312.0-73438.8 ns)
  63312.0 |################
  63818.3 |
  64324.7 |
  64831.0 |
  65337.3 |
  65843.7 |
  66350.0 |
  66856.4 |
  67362.7 |
  67869.0 |
  68375.4 |
  68881.7 |##
  69388.0 |
  69894.4 |#########
  70400.7 |###########
  70907.1 |
  71413.4 |
  71919.7 |
  72426.1 |
  72932.4 |########################################
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 168112.5-170893.5 ns)
  168112.5 |##########
  168251.6 |####
  168390.6 |##
  168529.7 |
  168668.7 |##
  168807.8 |##
  168946.8 |##
  169085.9 |
  169224.9 |
  169364.0 |
  169503.0 |
  169642.0 |########################################
  169781.1 |##
  169920.1 |##
  170059.2 |
  170198.2 |
  170337.3 |##
  170476.3 |
  170615.4 |
  170754.4 |##
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 167151.2-167437.0 ns)
  167151.2 |################
  167165.5 |########################################
  167179.8 |########################################
  167194.0 |################################
  167208.3 |########################################
  167222.6 |################
  167236.9 |################
  167251.2 |################
  167265.5 |########################
  167279.8 |################
  167294.1 |
  167308.4 |
  167322.7 |########
  167337.0 |########
  167351.3 |
  167365.5 |
  167379.8 |
  167394.1 |
  167408.4 |
  167422.7 |
  (4 below, 2 above range)

```
