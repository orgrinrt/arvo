# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (523.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 279.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 18% faster than the next best (wide-rung-ragged-overread)

wide-rung-ragged (279.92 us) leads wide-rung-ragged-overread (330.40 us) by 18%, a clear separation rather than a photo finish. CV 29.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 43% (significant)

wide-rung-ragged is -224.15 us (43%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-ragged is fastest but the noisiest (CV 29.5%)

wide-rung-ragged wins on median (279.92 us) yet has the highest variance (CV 29.5%), while wide-rung-align16 is the steadiest (CV 8.8%, 523.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.69)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged, wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias} vs {wide-rung-align16} (45% apart)

The field splits into a fast tier {wide-rung-ragged, wide-rung-ragged-overread, wide-rung-wordround, wide-rung-wordround-alias} and a slow tier {wide-rung-align16} with a 45% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-ragged is inconsistent: worst-20% is 1.9x its best-20%

wide-rung-ragged's best 20% of batches run at 223.31 us but its worst 20% at 430.51 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 279917.5 ns median (-46.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.87x (fastest 279917.5 ns, slowest 523089.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 514176ns | 523570ns | 458418ns | 513191ns | 572886ns | base |
| wide-rung-ragged | 293743ns | 280132ns | 223533ns | 271369ns | 431073ns | -42.87% |
| wide-rung-ragged-overread | 330276ns | 330730ns | 281009ns | 324314ns | 397430ns | -35.77% |
| wide-rung-wordround | 355816ns | 338549ns | 308179ns | 342795ns | 442513ns | -30.80% |
| wide-rung-wordround-alias | 398109ns | 361017ns | 346346ns | 385449ns | 487851ns | -22.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 513675ns | 457949ns | 572322ns | base | 0.893 |
| wide-rung-ragged | 293440ns | 223311ns | 430513ns | -42.87% | 1.563 |
| wide-rung-ragged-overread | 329934ns | 280725ns | 397015ns | -35.77% | 1.390 |
| wide-rung-wordround | 355402ns | 307800ns | 442081ns | -30.81% | 1.291 |
| wide-rung-wordround-alias | 397729ns | 346001ns | 487390ns | -22.57% | 1.153 |

## Performance model

- Peak throughput: **2.054 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.877 | 42.7% |
| wide-rung-ragged | 1.639 | 79.8% |
| wide-rung-ragged-overread | 1.388 | 67.6% |
| wide-rung-wordround | 1.357 | 66.1% |
| wide-rung-wordround-alias | 1.272 | 61.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 514176ns | 514176ns | base |
| wide-rung-ragged | 293743ns | 293743ns | -42.87% |
| wide-rung-ragged-overread | 330276ns | 330276ns | -35.77% |
| wide-rung-wordround | 355816ns | 355816ns | -30.80% |
| wide-rung-wordround-alias | 398109ns | 398109ns | -22.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 523089ns | base | --- | [487005, 532643] | --- | --- | --- | --- |
| wide-rung-ragged | 279918ns | -224150.9ns (-42.9%) | [-254082, -184202]ns | [240345, 282373] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 330400ns | -182267.9ns (-34.8%) | [-192897, -176788]ns | [318925, 331938] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 338051ns | -171547.4ns (-32.8%) | [-183688, -158215]ns | [322547, 366412] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 360659ns | -102407.7ns (-19.6%) | [-135237, -101135]ns | [358733, 422090] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 523850ns | -49.7% | -36.4% | -36.5% | -32.5% |
| 1 | 2 | 487005ns | -52.7% | -31.8% | -35.2% | -28.8% |
| 1 | 3 | 550559ns | -26.3% | -40.2% | -33.3% | -18.2% |
| 1 | 4 | 462783ns | -39.0% | -39.3% | -33.5% | -22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.687 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.374 | moderate- |
| wide-rung-ragged-overread | -0.033 | ok |
| wide-rung-wordround | -0.678 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.392 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 4/4, lost 0/4
- **wide-rung-ragged-overread**: won 4/4, lost 0/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 168.2ns | 513674.8ns | 0.0% |  |
| wide-rung-ragged | 80.9ns | 293439.6ns | 0.0% |  |
| wide-rung-ragged-overread | 99.3ns | 329933.9ns | 0.0% |  |
| wide-rung-wordround | 161.3ns | 355402.0ns | 0.0% |  |
| wide-rung-wordround-alias | 84.8ns | 397728.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 457948.5-572321.6 ns)
  457948.5 |################################
  463667.2 |
  469385.8 |####
  475104.5 |####
  480823.2 |########
  486541.8 |####
  492260.5 |########
  497979.1 |####
  503697.8 |
  509416.4 |
  515135.1 |
  520853.7 |############################
  526572.4 |####
  532291.0 |
  538009.7 |####
  543728.3 |
  549447.0 |########################################
  555165.6 |####
  560884.2 |
  566602.9 |
  (2 below, 2 above range)

wide-rung-ragged (n=40, range 223311.4-430512.6 ns)
  223311.4 |##########################
  233671.5 |
  244031.5 |##
  254391.6 |
  264751.6 |
  275111.7 |########################################
  285471.8 |
  295831.8 |
  306191.9 |
  316552.0 |
  326912.0 |########
  337272.1 |
  347632.1 |
  357992.2 |##
  368352.3 |
  378712.3 |
  389072.4 |
  399432.5 |##########
  409792.5 |
  420152.6 |
  (4 below, 2 above range)

wide-rung-ragged-overread (n=40, range 280725.0-397015.2 ns)
  280725.0 |#############
  286539.6 |
  292354.1 |##
  298168.6 |
  303983.1 |########
  309797.6 |
  315612.1 |
  321426.6 |
  327241.1 |########################################
  333055.6 |#####
  338870.1 |
  344684.6 |##
  350499.1 |
  356313.7 |##
  362128.2 |
  367942.7 |
  373757.2 |##
  379571.7 |#####
  385386.2 |##
  391200.7 |
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 307800.3-442081.5 ns)
  307800.3 |########
  314514.3 |###############################
  321228.4 |
  327942.5 |##########################
  334656.5 |
  341370.6 |####
  348084.6 |########
  354798.7 |####
  361512.8 |########################################
  368226.8 |#############
  374940.9 |
  381654.9 |
  388369.0 |
  395083.1 |
  401797.1 |
  408511.2 |
  415225.2 |
  421939.3 |
  428653.4 |
  435367.4 |
  (5 below, 4 above range)

wide-rung-wordround-alias (n=40, range 346000.5-487390.2 ns)
  346000.5 |########################################
  353070.0 |###################################
  360139.5 |###############
  367209.0 |
  374278.5 |
  381347.9 |##########
  388417.4 |
  395486.9 |#####
  402556.4 |#####
  409625.9 |
  416695.4 |#####
  423764.9 |
  430834.3 |
  437903.8 |
  444973.3 |########################################
  452042.8 |
  459112.3 |##########
  466181.8 |
  473251.3 |
  480320.7 |
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged**: CV=28.1% (high variance, measurements may be unstable)
