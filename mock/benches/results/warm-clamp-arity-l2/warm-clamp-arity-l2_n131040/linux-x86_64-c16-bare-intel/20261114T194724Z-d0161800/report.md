# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head beats baseline by 43% (significant)

warm-clamp-head is -53.37 us (43%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-min-lanes is an outlier: 2.4x slower than the field

warm-clamp-min-lanes (169.71 us) is 2.4x the fastest (69.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 shows alternating (throttle bounce) (autocorr -0.81)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-accfit} vs {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-min-lanes} (74% apart)

The field splits into a fast tier {warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-accfit} and a slow tier {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-min-lanes} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-clamp-head** at 69694.4 ns median (-43.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.44x (fastest 69694.4 ns, slowest 169712.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 123261ns | 123317ns | 122991ns | 123242ns | 123591ns | base |
| warm-clamp-accfit | 70633ns | 70892ns | 64549ns | 69786ns | 79257ns | -42.70% |
| warm-clamp-accfit-dyn | 69376ns | 70765ns | 64686ns | 70318ns | 71240ns | -43.72% |
| warm-clamp-head | 71112ns | 69838ns | 69696ns | 70597ns | 74072ns | -42.31% |
| warm-clamp-min-lanes | 169543ns | 169884ns | 168500ns | 169686ns | 170155ns | +37.55% |
| warm-clamp-minimum | 165947ns | 165354ns | 165147ns | 165618ns | 167734ns | +34.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 123110ns | 122845ns | 123442ns | base | 8.517 |
| warm-clamp-accfit | 70447ns | 64403ns | 79093ns | -42.78% | 14.885 |
| warm-clamp-accfit-dyn | 69207ns | 64515ns | 71022ns | -43.78% | 15.151 |
| warm-clamp-head | 70959ns | 69554ns | 73912ns | -42.36% | 14.777 |
| warm-clamp-min-lanes | 169366ns | 168331ns | 169965ns | +37.57% | 6.191 |
| warm-clamp-minimum | 165767ns | 164986ns | 167530ns | +34.65% | 6.326 |

## Performance model

- Peak throughput: **16.282 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.514 | 52.3% |
| warm-clamp-accfit | 14.847 | 91.2% |
| warm-clamp-accfit-dyn | 14.847 | 91.2% |
| warm-clamp-head | 15.045 | 92.4% |
| warm-clamp-min-lanes | 6.179 | 37.9% |
| warm-clamp-minimum | 6.349 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 123261ns | 123261ns | base |
| warm-clamp-accfit | 70633ns | 70633ns | -42.70% |
| warm-clamp-accfit-dyn | 69376ns | 69376ns | -43.72% |
| warm-clamp-head | 71112ns | 71112ns | -42.31% |
| warm-clamp-min-lanes | 169543ns | 169543ns | +37.55% |
| warm-clamp-minimum | 165947ns | 165947ns | +34.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 123164ns | base | --- | [122942, 123214] | --- | --- | --- | --- |
| warm-clamp-accfit | 70625ns | -52655.6ns (-42.8%) | [-52977, -51863]ns | [70275, 70989] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 70625ns | -52621.2ns (-42.7%) | [-52709, -52558]ns | [70580, 70677] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 69694ns | -53370.2ns (-43.3%) | [-53486, -53258]ns | [69640, 69836] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 169713ns | +46572.1ns (+37.8%) | [+46458, +46763]ns | [169660, 169800] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 165150ns | +42273.1ns (+34.3%) | [+41995, +42370]ns | [165092, 165286] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 123203ns | -47.7% | -42.7% | -43.4% | +36.8% | +34.0% |
| 1 | 2 | 122847ns | -42.2% | -42.2% | -40.3% | +38.2% | +34.5% |
| 1 | 3 | 123290ns | -42.7% | -42.7% | -43.5% | +37.7% | +35.6% |
| 1 | 4 | 122946ns | -36.0% | -47.5% | -43.4% | +38.0% | +35.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.808 | HIGH- (thermal bounce) |
| warm-clamp-accfit | -0.028 | ok |
| warm-clamp-accfit-dyn | -0.052 | ok |
| warm-clamp-head | -0.394 | moderate- |
| warm-clamp-min-lanes | -0.098 | ok |
| warm-clamp-minimum | 0.005 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 32.8ns | 123110.5ns | 0.0% |  |
| warm-clamp-accfit | 35.2ns | 70446.6ns | 0.0% |  |
| warm-clamp-accfit-dyn | 30.3ns | 69207.0ns | 0.0% |  |
| warm-clamp-head | 31.3ns | 70959.0ns | 0.0% |  |
| warm-clamp-min-lanes | 19.6ns | 169365.6ns | 0.0% |  |
| warm-clamp-minimum | 22.0ns | 165766.7ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 122844.6-123441.8 ns)
  122844.6 |################################
  122874.5 |################
  122904.4 |########################
  122934.2 |########################################
  122964.1 |
  122993.9 |########
  123023.8 |
  123053.7 |
  123083.5 |
  123113.4 |
  123143.2 |########################
  123173.1 |########################
  123203.0 |################
  123232.8 |########
  123262.7 |################################
  123292.5 |########################################
  123322.4 |########
  123352.3 |
  123382.1 |
  123412.0 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 64402.7-79093.2 ns)
  64402.7 |####################
  65137.2 |##
  65871.7 |
  66606.3 |
  67340.8 |
  68075.3 |
  68809.8 |
  69544.4 |##
  70278.9 |########################################
  71013.4 |########
  71748.0 |
  72482.5 |
  73217.0 |##
  73951.5 |
  74686.1 |
  75420.6 |
  76155.1 |
  76889.7 |
  77624.2 |
  78358.7 |#################
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 64514.7-71022.2 ns)
  64514.7 |##############
  64840.1 |
  65165.4 |
  65490.8 |
  65816.2 |
  66141.6 |
  66466.9 |
  66792.3 |
  67117.7 |
  67443.1 |
  67768.5 |
  68093.8 |
  68419.2 |
  68744.6 |
  69070.0 |
  69395.4 |
  69720.7 |
  70046.1 |
  70371.5 |########################################
  70696.9 |#######################
  (4 below, 3 above range)

warm-clamp-head (n=40, range 69554.1-73911.8 ns)
  69554.1 |########################################
  69772.0 |#####
  69989.9 |
  70207.7 |
  70425.6 |
  70643.5 |
  70861.4 |
  71079.3 |
  71297.2 |
  71515.0 |
  71732.9 |
  71950.8 |
  72168.7 |
  72386.6 |
  72604.5 |
  72822.3 |
  73040.2 |#
  73258.1 |#################
  73476.0 |#
  73693.9 |
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 168331.3-169964.5 ns)
  168331.3 |##########
  168413.0 |######
  168494.6 |######
  168576.3 |######
  168658.0 |######
  168739.6 |
  168821.3 |
  168902.9 |
  168984.6 |
  169066.3 |
  169147.9 |
  169229.6 |
  169311.2 |
  169392.9 |
  169474.6 |
  169556.2 |
  169637.9 |##############################
  169719.6 |#############
  169801.2 |########################################
  169882.9 |
  (2 below, 2 above range)

warm-clamp-minimum (n=40, range 164985.8-167530.3 ns)
  164985.8 |########################################
  165113.0 |##########################
  165240.2 |######
  165367.4 |
  165494.7 |###
  165621.9 |
  165749.1 |
  165876.3 |###
  166003.6 |
  166130.8 |
  166258.0 |
  166385.3 |
  166512.5 |
  166639.7 |
  166766.9 |
  166894.2 |
  167021.4 |######
  167148.6 |####################
  167275.8 |
  167403.1 |
  (5 below, 3 above range)

```
