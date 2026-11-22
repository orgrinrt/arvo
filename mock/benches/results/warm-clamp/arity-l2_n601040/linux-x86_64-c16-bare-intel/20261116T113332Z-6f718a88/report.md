# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes is an outlier: 4.0x slower than the field

warm-clamp-min-lanes (578.27 us) is 4.0x the fastest (143.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-accfit} vs {warm-clamp-head, warm-clamp-min-lanes} (159% apart)

The field splits into a fast tier {warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-minimum, warm-clamp-accfit} and a slow tier {warm-clamp-head, warm-clamp-min-lanes} with a 159% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest warm-clamp-accfit-dyn (143.57 us) to slowest warm-clamp-min-lanes (578.27 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 1.8x its best-20%

warm-clamp-minimum's best 20% of batches run at 118.36 us but its worst 20% at 211.38 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 143574.1 ns median (-1.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.03x (fastest 143574.1 ns, slowest 578267.2 ns)

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
| warm-clamp-acc64 | 169451ns | 145727ns | 129921ns | 164810ns | 222906ns | base |
| warm-clamp-accfit | 171886ns | 165274ns | 129839ns | 173005ns | 210576ns | +1.44% |
| warm-clamp-accfit-dyn | 163322ns | 143789ns | 130087ns | 153286ns | 226663ns | -3.62% |
| warm-clamp-head | 450410ns | 427773ns | 412437ns | 427230ns | 557921ns | +165.80% |
| warm-clamp-min-lanes | 594073ns | 578460ns | 549040ns | 588092ns | 657047ns | +250.59% |
| warm-clamp-minimum | 160824ns | 151671ns | 118522ns | 157952ns | 211743ns | -5.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 169180ns | 129725ns | 222588ns | base | 6.198 |
| warm-clamp-accfit | 171629ns | 129675ns | 210224ns | +1.45% | 6.110 |
| warm-clamp-accfit-dyn | 163027ns | 129886ns | 226154ns | -3.64% | 6.432 |
| warm-clamp-head | 449653ns | 411524ns | 557269ns | +165.78% | 2.332 |
| warm-clamp-min-lanes | 593828ns | 548801ns | 656741ns | +251.00% | 1.766 |
| warm-clamp-minimum | 160562ns | 118358ns | 211378ns | -5.09% | 6.531 |

## Performance model

- Peak throughput: **8.859 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 7.214 | 81.4% |
| warm-clamp-accfit | 6.352 | 71.7% |
| warm-clamp-accfit-dyn | 7.303 | 82.4% |
| warm-clamp-head | 2.455 | 27.7% |
| warm-clamp-min-lanes | 1.813 | 20.5% |
| warm-clamp-minimum | 6.926 | 78.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 169451ns | 169451ns | base |
| warm-clamp-accfit | 171886ns | 171886ns | +1.44% |
| warm-clamp-accfit-dyn | 163322ns | 163322ns | -3.62% |
| warm-clamp-head | 450410ns | 450410ns | +165.80% |
| warm-clamp-min-lanes | 594073ns | 594073ns | +250.59% |
| warm-clamp-minimum | 160824ns | 160824ns | -5.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 145346ns | base | --- | [130731, 213577] | --- | --- | --- | --- |
| warm-clamp-accfit | 165066ns | -1087.5ns (-0.7%) | [-3412, -496]ns | [164696, 197899] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 143574ns | no significant difference | [-1563, +25512]ns | [130066, 176947] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-head | 427158ns | +281325.0ns (+193.6%) | [+252909, +295672]ns | [417856, 427633] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 578267ns | +417246.2ns (+287.1%) | [+400168, +423520]ns | [573176, 607197] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 151398ns | -12431.5ns (-8.6%) | [-15561, -11840]ns | [129627, 187572] | YES | 0.0276 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 129777ns | +26.9% | +36.2% | +217.5% | +388.0% | +64.8% |
| 1 | 2 | 130750ns | -0.8% | -0.6% | +227.0% | +320.1% | -9.5% |
| 1 | 3 | 215804ns | -2.5% | -39.8% | +98.6% | +166.4% | -7.9% |
| 1 | 4 | 222599ns | -10.7% | -1.1% | +87.1% | +160.1% | -41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.269 | moderate+ |
| warm-clamp-accfit | -0.073 | ok |
| warm-clamp-accfit-dyn | -0.208 | moderate- |
| warm-clamp-head | -0.229 | moderate- |
| warm-clamp-min-lanes | -0.360 | moderate- |
| warm-clamp-minimum | -0.725 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 40.5ns | 169180.2ns | 0.0% |  |
| warm-clamp-accfit | 60.8ns | 171628.6ns | 0.0% |  |
| warm-clamp-accfit-dyn | 41.2ns | 163026.9ns | 0.0% |  |
| warm-clamp-head | 134.3ns | 449652.8ns | 0.0% |  |
| warm-clamp-min-lanes | 59.0ns | 593827.8ns | 0.0% |  |
| warm-clamp-minimum | 37.2ns | 160561.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 129725.1-222588.2 ns)
  129725.1 |########################################
  134368.3 |
  139011.5 |
  143654.6 |
  148297.8 |
  152940.9 |
  157584.1 |##
  162227.2 |##
  166870.4 |
  171513.5 |#####
  176156.7 |
  180799.8 |
  185443.0 |
  190086.1 |
  194729.3 |
  199372.4 |##
  204015.6 |
  208658.7 |##
  213301.9 |#################
  217945.0 |#####
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 129674.8-210223.9 ns)
  129674.8 |#######################
  133702.2 |
  137729.7 |
  141757.1 |
  145784.6 |
  149812.0 |
  153839.5 |
  157867.0 |
  161894.4 |########################################
  165921.9 |
  169949.3 |##########
  173976.8 |
  178004.2 |
  182031.7 |
  186059.2 |
  190086.6 |
  194114.1 |######
  198141.5 |################
  202169.0 |###
  206196.4 |######
  (3 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 129885.7-226154.5 ns)
  129885.7 |########################################
  134699.2 |
  139512.6 |
  144326.0 |
  149139.5 |
  153952.9 |##
  158766.4 |
  163579.8 |##
  168393.2 |
  173206.7 |#################
  178020.1 |##
  182833.5 |##########
  187647.0 |##
  192460.4 |
  197273.9 |
  202087.3 |
  206900.7 |
  211714.2 |
  216527.6 |
  221341.0 |
  (4 below, 5 above range)

warm-clamp-head (n=40, range 411524.3-557268.9 ns)
  411524.3 |####################################
  418811.5 |##########
  426098.7 |########################################
  433386.0 |
  440673.2 |
  447960.4 |###
  455247.7 |
  462534.9 |##########
  469822.1 |###
  477109.4 |
  484396.6 |
  491683.8 |
  498971.1 |
  506258.3 |
  513545.5 |
  520832.8 |
  528120.0 |
  535407.2 |###
  542694.5 |
  549981.7 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 548801.5-656741.4 ns)
  548801.5 |######################
  554198.5 |####
  559595.5 |####
  564992.5 |####
  570389.5 |#################
  575786.5 |########################################
  581183.5 |
  586580.5 |####
  591977.5 |
  597374.5 |
  602771.5 |
  608168.5 |
  613565.5 |
  618962.5 |
  624359.5 |########
  629756.5 |######################
  635153.5 |#############
  640550.5 |########
  645947.5 |
  651344.5 |
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 118358.0-211377.9 ns)
  118358.0 |########################
  123009.0 |
  127660.0 |########################################
  132311.0 |
  136962.0 |
  141613.0 |
  146264.0 |
  150915.0 |
  155566.0 |
  160217.0 |
  164868.0 |
  169518.9 |############
  174169.9 |
  178820.9 |####
  183471.9 |############
  188122.9 |####
  192773.9 |
  197424.9 |####################
  202075.9 |####
  206726.9 |
  (4 below, 6 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=24.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=24.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=23.8% (high variance, measurements may be unstable)
