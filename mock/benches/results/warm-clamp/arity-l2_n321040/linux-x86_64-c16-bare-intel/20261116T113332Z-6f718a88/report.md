# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 4.2x slower than the field

warm-clamp-minimum (404.73 us) is 4.2x the fastest (97.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.62)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.2x the fastest

Fastest warm-clamp-accfit (97.40 us) to slowest warm-clamp-minimum (404.73 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-head is inconsistent: worst-20% is 2.1x its best-20%

warm-clamp-head's best 20% of batches run at 130.30 us but its worst 20% at 270.21 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 97395.5 ns median (-8.4% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.16x (fastest 97395.5 ns, slowest 404727.5 ns)

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
| warm-clamp-acc64 | 107959ns | 106535ns | 106286ns | 106715ns | 113364ns | base |
| warm-clamp-accfit | 99690ns | 97569ns | 96885ns | 98376ns | 106437ns | -7.66% |
| warm-clamp-accfit-dyn | 104446ns | 106432ns | 96784ns | 105749ns | 108199ns | -3.25% |
| warm-clamp-head | 190479ns | 186326ns | 130478ns | 183776ns | 270592ns | +76.44% |
| warm-clamp-min-lanes | 361288ns | 359364ns | 353397ns | 359897ns | 373352ns | +234.65% |
| warm-clamp-minimum | 407409ns | 404911ns | 401969ns | 405345ns | 419041ns | +277.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 107790ns | 106120ns | 113198ns | base | 9.728 |
| warm-clamp-accfit | 99519ns | 96726ns | 106280ns | -7.67% | 10.536 |
| warm-clamp-accfit-dyn | 104275ns | 96609ns | 108026ns | -3.26% | 10.056 |
| warm-clamp-head | 190152ns | 130300ns | 270213ns | +76.41% | 5.514 |
| warm-clamp-min-lanes | 361084ns | 353214ns | 373159ns | +234.99% | 2.904 |
| warm-clamp-minimum | 407219ns | 401787ns | 418852ns | +277.79% | 2.575 |

## Performance model

- Peak throughput: **10.854 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.857 | 90.8% |
| warm-clamp-accfit | 10.766 | 99.2% |
| warm-clamp-accfit-dyn | 9.869 | 90.9% |
| warm-clamp-head | 5.640 | 52.0% |
| warm-clamp-min-lanes | 2.919 | 26.9% |
| warm-clamp-minimum | 2.591 | 23.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 107959ns | 107959ns | base |
| warm-clamp-accfit | 99690ns | 99690ns | -7.66% |
| warm-clamp-accfit-dyn | 104446ns | 104446ns | -3.25% |
| warm-clamp-head | 190479ns | 190479ns | +76.44% |
| warm-clamp-min-lanes | 361288ns | 361288ns | +234.65% |
| warm-clamp-minimum | 407409ns | 407409ns | +277.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 106378ns | base | --- | [106343, 106441] | --- | --- | --- | --- |
| warm-clamp-accfit | 97396ns | -9221.5ns (-8.7%) | [-9673, -8646]ns | [97275, 97647] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 106245ns | -349.4ns (-0.3%) | [-5768, -151]ns | [106127, 106333] | YES | 0.0064 | 0.0064 | 0 |
| warm-clamp-head | 185927ns | +79486.0ns (+74.7%) | [+75763, +89248]ns | [182254, 195367] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 359175ns | +253200.0ns (+238.0%) | [+249874, +256743]ns | [356262, 363122] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 404728ns | +297822.7ns (+280.0%) | [+296688, +299561]ns | [403853, 406629] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 113159ns | -13.9% | -5.1% | +15.4% | +212.8% | +257.6% |
| 1 | 2 | 106409ns | -9.0% | -9.2% | +71.9% | +241.2% | +295.9% |
| 1 | 3 | 106301ns | -0.1% | +0.0% | +171.5% | +235.3% | +282.4% |
| 1 | 4 | 106343ns | -8.4% | -0.2% | +83.7% | +242.0% | +278.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.072 | ok |
| warm-clamp-accfit | -0.430 | moderate- |
| warm-clamp-accfit-dyn | -0.483 | moderate- |
| warm-clamp-head | -0.054 | ok |
| warm-clamp-min-lanes | -0.621 | HIGH- (thermal bounce) |
| warm-clamp-minimum | -0.296 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 0/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 29.9ns | 107790.1ns | 0.0% |  |
| warm-clamp-accfit | 30.4ns | 99519.3ns | 0.0% |  |
| warm-clamp-accfit-dyn | 27.9ns | 104274.5ns | 0.0% |  |
| warm-clamp-head | 88.8ns | 190151.6ns | 0.0% |  |
| warm-clamp-min-lanes | 20.8ns | 361084.5ns | 0.0% |  |
| warm-clamp-minimum | 34.4ns | 407219.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 106120.1-113198.1 ns)
  106120.1 |########################################
  106474.0 |###
  106827.9 |
  107181.8 |#
  107535.7 |
  107889.6 |
  108243.5 |
  108597.4 |
  108951.3 |#
  109305.2 |
  109659.1 |
  110013.0 |
  110366.9 |
  110720.8 |
  111074.7 |
  111428.6 |
  111782.5 |
  112136.4 |
  112490.3 |
  112844.2 |####
  (3 below, 5 above range)

warm-clamp-accfit (n=40, range 96726.1-106279.8 ns)
  96726.1 |#################
  97203.8 |########################################
  97681.4 |##
  98159.1 |
  98636.8 |#####
  99114.5 |
  99592.2 |
  100069.9 |
  100547.6 |
  101025.3 |
  101502.9 |
  101980.6 |
  102458.3 |
  102936.0 |
  103413.7 |
  103891.4 |
  104369.1 |
  104846.8 |
  105324.4 |
  105802.1 |#################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 96609.0-108025.9 ns)
  96609.0 |##################
  97179.8 |
  97750.7 |
  98321.5 |
  98892.4 |
  99463.2 |
  100034.1 |
  100604.9 |
  101175.8 |
  101746.6 |
  102317.4 |
  102888.3 |
  103459.1 |
  104030.0 |
  104600.8 |
  105171.7 |
  105742.5 |########################################
  106313.4 |##################
  106884.2 |##################
  107455.1 |############
  (4 below, 1 above range)

warm-clamp-head (n=40, range 130300.1-270213.1 ns)
  130300.1 |###############################
  137295.8 |
  144291.4 |####
  151287.1 |####
  158282.7 |
  165278.4 |
  172274.0 |
  179269.7 |###################################
  186265.3 |####
  193261.0 |########################################
  200256.6 |
  207252.3 |#################
  214247.9 |
  221243.6 |
  228239.2 |
  235234.9 |
  242230.5 |
  249226.2 |
  256221.8 |
  263217.5 |
  (3 below, 6 above range)

warm-clamp-min-lanes (n=40, range 353214.0-373158.7 ns)
  353214.0 |
  354211.2 |#################################
  355208.4 |##########################
  356205.7 |########################################
  357202.9 |
  358200.1 |
  359197.4 |
  360194.6 |
  361191.8 |######
  362189.1 |########################################
  363186.3 |####################
  364183.5 |######
  365180.8 |##########################
  366178.0 |######
  367175.2 |
  368172.5 |
  369169.7 |
  370166.9 |
  371164.2 |
  372161.4 |
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 401787.0-418851.9 ns)
  401787.0 |###############
  402640.2 |##########
  403493.5 |########################################
  404346.7 |##############################
  405200.0 |
  406053.2 |#########################
  406906.5 |####################
  407759.7 |#####
  408612.9 |#####
  409466.2 |
  410319.4 |
  411172.7 |
  412025.9 |
  412879.2 |
  413732.4 |
  414585.6 |
  415438.9 |
  416292.1 |
  417145.4 |
  417998.6 |
  (4 below, 6 above range)

```

## Diagnostics

- **warm-clamp-head**: CV=26.3% (high variance, measurements may be unstable)
