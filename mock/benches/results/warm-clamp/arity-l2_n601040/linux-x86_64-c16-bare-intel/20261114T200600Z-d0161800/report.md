# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-acc64 dominates: 10% faster than the next best (warm-clamp-accfit)

warm-clamp-acc64 (129.18 us) leads warm-clamp-accfit (142.47 us) by 10%, a clear separation rather than a photo finish. CV 27.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is an outlier: 4.5x slower than the field

warm-clamp-min-lanes (575.16 us) is 4.5x the fastest (129.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 is fastest but the noisiest (CV 27.5%)

warm-clamp-acc64 wins on median (129.18 us) yet has the highest variance (CV 27.5%), while warm-clamp-min-lanes is the steadiest (CV 5.8%, 575.16 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit shows alternating (throttle bounce) (autocorr -0.78)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (129.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-minimum, warm-clamp-accfit-dyn} vs {warm-clamp-head, warm-clamp-min-lanes} (129% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-minimum, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-head, warm-clamp-min-lanes} with a 129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.5x the fastest

Fastest warm-clamp-acc64 (129.18 us) to slowest warm-clamp-min-lanes (575.16 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 1.8x its best-20%

warm-clamp-minimum's best 20% of batches run at 116.02 us but its worst 20% at 213.46 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 129179.5 ns median
- 4 variants significantly slower than baseline
- Spread: 4.45x (fastest 129179.5 ns, slowest 575162.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 148648ns | 129345ns | 125873ns | 134307ns | 214447ns | base |
| warm-clamp-accfit | 159321ns | 142784ns | 128797ns | 154212ns | 205172ns | +7.18% |
| warm-clamp-accfit-dyn | 198089ns | 193596ns | 164983ns | 193549ns | 244817ns | +33.26% |
| warm-clamp-head | 503631ns | 444460ns | 415531ns | 491310ns | 628693ns | +238.81% |
| warm-clamp-min-lanes | 575192ns | 575360ns | 532729ns | 573106ns | 623914ns | +286.95% |
| warm-clamp-minimum | 174160ns | 189944ns | 116206ns | 180230ns | 213904ns | +17.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 148408ns | 125701ns | 214016ns | base | 7.066 |
| warm-clamp-accfit | 159061ns | 128614ns | 204853ns | +7.18% | 6.592 |
| warm-clamp-accfit-dyn | 197708ns | 164684ns | 244262ns | +33.22% | 5.304 |
| warm-clamp-head | 502582ns | 414691ns | 627630ns | +238.65% | 2.086 |
| warm-clamp-min-lanes | 574905ns | 532539ns | 623515ns | +287.38% | 1.824 |
| warm-clamp-minimum | 173827ns | 116024ns | 213460ns | +17.13% | 6.032 |

## Performance model

- Peak throughput: **9.038 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.117 | 89.8% |
| warm-clamp-accfit | 7.360 | 81.4% |
| warm-clamp-accfit-dyn | 5.425 | 60.0% |
| warm-clamp-head | 2.365 | 26.2% |
| warm-clamp-min-lanes | 1.823 | 20.2% |
| warm-clamp-minimum | 5.531 | 61.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 148648ns | 148648ns | base |
| warm-clamp-accfit | 159321ns | 159321ns | +7.18% |
| warm-clamp-accfit-dyn | 198089ns | 198089ns | +33.26% |
| warm-clamp-head | 503631ns | 503631ns | +238.81% |
| warm-clamp-min-lanes | 575192ns | 575192ns | +286.95% |
| warm-clamp-minimum | 174160ns | 174160ns | +17.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 129180ns | base | --- | [128554, 129714] | --- | --- | --- | --- |
| warm-clamp-accfit | 142474ns | +13386.8ns (+10.4%) | [+301, +62722]ns | [129654, 191227] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-accfit-dyn | 193273ns | +64751.7ns (+50.1%) | [+54808, +66295]ns | [191707, 195074] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-head | 443411ns | +319859.3ns (+247.6%) | [+313739, +394084]ns | [441823, 552879] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 575162ns | +422876.4ns (+327.4%) | [+407093, +447484]ns | [568490, 585903] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 189582ns | no significant difference | [-12398, +54705]ns | [164873, 196975] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 213804ns | -39.8% | -10.2% | +184.7% | +167.1% | -8.9% |
| 1 | 2 | 129668ns | +58.6% | +49.6% | +219.4% | +311.5% | +65.2% |
| 1 | 3 | 128513ns | +0.3% | +51.6% | +245.0% | +363.3% | +49.9% |
| 1 | 4 | 128500ns | +49.0% | +67.7% | +243.5% | +347.6% | -9.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.073 | ok |
| warm-clamp-accfit | -0.784 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.032 | ok |
| warm-clamp-head | -0.210 | moderate- |
| warm-clamp-min-lanes | -0.427 | moderate- |
| warm-clamp-minimum | 0.028 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 45.4ns | 148407.9ns | 0.0% |  |
| warm-clamp-accfit | 84.1ns | 159060.5ns | 0.1% |  |
| warm-clamp-accfit-dyn | 59.4ns | 197708.4ns | 0.0% |  |
| warm-clamp-head | 192.1ns | 502582.2ns | 0.0% |  |
| warm-clamp-min-lanes | 56.7ns | 574904.9ns | 0.0% |  |
| warm-clamp-minimum | 78.6ns | 173827.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 125700.8-214016.4 ns)
  125700.8 |########################################
  130116.6 |###
  134532.4 |
  138948.1 |
  143363.9 |
  147779.7 |
  152195.5 |
  156611.3 |
  161027.0 |
  165442.8 |
  169858.6 |#
  174274.4 |
  178690.2 |
  183105.9 |
  187521.7 |
  191937.5 |
  196353.3 |
  200769.0 |#
  205184.8 |
  209600.6 |######
  (2 below, 4 above range)

warm-clamp-accfit (n=40, range 128613.8-204853.4 ns)
  128613.8 |########################################
  132425.8 |#####
  136237.8 |
  140049.7 |
  143861.7 |
  147673.7 |#####
  151485.7 |#####
  155297.7 |
  159109.6 |
  162921.6 |
  166733.6 |
  170545.6 |
  174357.6 |
  178169.6 |##
  181981.5 |
  185793.5 |
  189605.5 |####################
  193417.5 |##
  197229.5 |
  201041.5 |##
  (4 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 164684.1-244261.8 ns)
  164684.1 |
  168663.0 |###
  172641.9 |
  176620.8 |
  180599.7 |################
  184578.5 |
  188557.4 |#######################
  192536.3 |########################################
  196515.2 |###
  200494.1 |
  204473.0 |
  208451.9 |
  212430.7 |#############
  216409.6 |
  220388.5 |
  224367.4 |
  228346.3 |
  232325.2 |
  236304.1 |
  240282.9 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 414691.3-627630.2 ns)
  414691.3 |#####
  425338.3 |
  435985.2 |########################################
  446632.1 |##
  457279.1 |
  467926.0 |
  478573.0 |#####
  489219.9 |
  499866.9 |
  510513.8 |
  521160.8 |
  531807.7 |##
  542454.7 |
  553101.6 |
  563748.5 |##
  574395.5 |
  585042.4 |
  595689.4 |##
  606336.3 |#########################
  616983.3 |
  (6 below, 3 above range)

warm-clamp-min-lanes (n=40, range 532538.9-623515.0 ns)
  532538.9 |###################################
  537087.7 |#####
  541636.5 |#####
  546185.3 |
  550734.1 |
  555282.9 |
  559831.7 |
  564380.5 |####################
  568929.3 |##########
  573478.1 |###################################
  578026.9 |#####
  582575.7 |
  587124.6 |
  591673.4 |########################################
  596222.2 |
  600771.0 |
  605319.8 |
  609868.6 |#####
  614417.4 |
  618966.2 |#####
  (3 below, 4 above range)

warm-clamp-minimum (n=40, range 116024.1-213459.5 ns)
  116024.1 |########################################
  120895.9 |
  125767.7 |
  130639.4 |
  135511.2 |
  140383.0 |
  145254.8 |
  150126.5 |
  154998.3 |###########
  159870.1 |###########
  164741.8 |#####
  169613.6 |#####
  174485.4 |
  179357.1 |#################
  184228.9 |#####
  189100.7 |
  193972.4 |########################################
  198844.2 |############################
  203716.0 |#####
  208587.8 |#####
  (3 below, 6 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=23.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=20.5% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=21.2% (high variance, measurements may be unstable)
