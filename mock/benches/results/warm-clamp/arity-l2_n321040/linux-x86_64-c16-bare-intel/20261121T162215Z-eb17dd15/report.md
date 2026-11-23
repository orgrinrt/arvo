# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 3.9x slower than the field

warm-clamp-minimum (393.25 us) is 3.9x the fastest (102.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum} (184% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 184% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.9x the fastest

Fastest warm-clamp-accfit (102.05 us) to slowest warm-clamp-minimum (393.25 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-head is inconsistent: worst-20% is 1.7x its best-20%

warm-clamp-head's best 20% of batches run at 116.81 us but its worst 20% at 198.48 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 102052.9 ns median (-3.8% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 3.85x (fastest 102052.9 ns, slowest 393251.2 ns)

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
| Counter frequency | 2112705153 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 107145ns | 106289ns | 97158ns | 104041ns | 126444ns | base |
| warm-clamp-accfit | 102346ns | 102209ns | 97454ns | 102181ns | 107732ns | -4.48% |
| warm-clamp-accfit-dyn | 104963ns | 107479ns | 97022ns | 106591ns | 108021ns | -2.04% |
| warm-clamp-head | 140223ns | 128751ns | 116967ns | 128446ns | 198813ns | +30.87% |
| warm-clamp-min-lanes | 373436ns | 365189ns | 353662ns | 369480ns | 405077ns | +248.53% |
| warm-clamp-minimum | 397700ns | 393413ns | 389095ns | 395769ns | 412100ns | +271.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 106972ns | 97000ns | 126242ns | base | 9.802 |
| warm-clamp-accfit | 102153ns | 97292ns | 107447ns | -4.50% | 10.265 |
| warm-clamp-accfit-dyn | 104805ns | 96864ns | 107869ns | -2.03% | 10.005 |
| warm-clamp-head | 140019ns | 116806ns | 198477ns | +30.89% | 7.489 |
| warm-clamp-min-lanes | 373255ns | 353487ns | 404877ns | +248.93% | 2.809 |
| warm-clamp-minimum | 397524ns | 388916ns | 411913ns | +271.62% | 2.638 |

## Performance model

- Peak throughput: **10.825 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.883 | 91.3% |
| warm-clamp-accfit | 10.275 | 94.9% |
| warm-clamp-accfit-dyn | 9.769 | 90.2% |
| warm-clamp-head | 8.154 | 75.3% |
| warm-clamp-min-lanes | 2.873 | 26.5% |
| warm-clamp-minimum | 2.666 | 24.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 107145ns | 107145ns | base |
| warm-clamp-accfit | 102346ns | 102346ns | -4.48% |
| warm-clamp-accfit-dyn | 104963ns | 104963ns | -2.04% |
| warm-clamp-head | 140223ns | 140223ns | +30.87% |
| warm-clamp-min-lanes | 373436ns | 373436ns | +248.53% |
| warm-clamp-minimum | 397700ns | 397700ns | +271.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 106097ns | base | --- | [97421, 107145] | --- | --- | --- | --- |
| warm-clamp-accfit | 102053ns | no significant difference | [-9500, +1308]ns | [97485, 106416] | no | 0.2682 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 107337ns | +327.1ns (+0.3%) | [+63, +10020]ns | [106374, 107422] | YES | 0.0481 | 0.0385 | 0 |
| warm-clamp-head | 128593ns | +22472.8ns (+21.2%) | [+21547, +24726]ns | [118477, 129637] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 365019ns | +261088.0ns (+246.1%) | [+257525, +275966]ns | [355448, 383238] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 393251ns | +292644.2ns (+275.8%) | [+291384, +296115]ns | [390673, 396669] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 102282ns | -4.8% | +5.1% | +15.8% | +245.7% | +281.7% |
| 1 | 2 | 97274ns | +9.4% | +10.5% | +19.8% | +265.3% | +300.3% |
| 1 | 3 | 106097ns | +1.2% | +0.3% | +21.3% | +257.6% | +271.4% |
| 1 | 4 | 107354ns | -9.2% | -9.8% | +63.1% | +257.1% | +281.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.008 | ok |
| warm-clamp-accfit | -0.251 | moderate- |
| warm-clamp-accfit-dyn | -0.003 | ok |
| warm-clamp-head | 0.071 | ok |
| warm-clamp-min-lanes | 0.291 | moderate+ |
| warm-clamp-minimum | 0.082 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 2/4, lost 2/4
- **warm-clamp-accfit-dyn**: won 1/4, lost 3/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 0/4, lost 4/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 32.8ns | 106971.9ns | 0.0% |  |
| warm-clamp-accfit | 29.9ns | 102153.5ns | 0.0% |  |
| warm-clamp-accfit-dyn | 29.2ns | 104804.8ns | 0.0% |  |
| warm-clamp-head | 57.5ns | 140019.2ns | 0.0% |  |
| warm-clamp-min-lanes | 21.8ns | 373255.0ns | 0.0% |  |
| warm-clamp-minimum | 33.4ns | 397524.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 96999.7-126242.0 ns)
  96999.7 |###########################
  98461.8 |
  99923.9 |
  101386.0 |
  102848.1 |
  104310.3 |
  105772.4 |########################################
  107234.5 |###############
  108696.6 |##
  110158.7 |
  111620.9 |
  113083.0 |
  114545.1 |##
  116007.2 |
  117469.3 |
  118931.4 |
  120393.6 |
  121855.7 |
  123317.8 |
  124779.9 |
  (4 below, 1 above range)

warm-clamp-accfit (n=40, range 97291.8-107447.2 ns)
  97291.8 |########################################
  97799.6 |##
  98307.4 |
  98815.1 |
  99322.9 |
  99830.7 |
  100338.4 |
  100846.2 |
  101354.0 |
  101861.7 |
  102369.5 |
  102877.3 |
  103385.1 |
  103892.8 |
  104400.6 |
  104908.4 |
  105416.1 |
  105923.9 |########################
  106431.7 |##
  106939.4 |################
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 96864.3-107869.0 ns)
  96864.3 |###########
  97414.6 |
  97964.8 |##
  98515.0 |
  99065.3 |
  99615.5 |
  100165.7 |
  100716.0 |
  101266.2 |
  101816.4 |
  102366.7 |##
  102916.9 |
  103467.1 |
  104017.4 |
  104567.6 |
  105117.8 |
  105668.1 |
  106218.3 |####################
  106768.5 |##
  107318.8 |########################################
  (3 below, 2 above range)

warm-clamp-head (n=40, range 116806.0-198476.9 ns)
  116806.0 |########################################
  120889.6 |
  124973.1 |#############################
  129056.7 |##################
  133140.2 |
  137223.8 |
  141307.3 |
  145390.8 |
  149474.4 |
  153557.9 |
  157641.5 |###
  161725.0 |
  165808.6 |
  169892.1 |#######
  173975.6 |##########
  178059.2 |
  182142.7 |
  186226.3 |#######
  190309.8 |###
  194393.3 |
  (6 below, 1 above range)

warm-clamp-min-lanes (n=40, range 353487.1-404877.0 ns)
  353487.1 |########################################
  356056.6 |###
  358626.1 |
  361195.6 |##############
  363765.1 |
  366334.6 |###
  368904.1 |###
  371473.5 |
  374043.0 |
  376612.5 |
  379182.0 |
  381751.5 |#############################
  384321.0 |##################
  386890.5 |
  389460.0 |###
  392029.5 |##########
  394599.0 |
  397168.5 |
  399738.0 |
  402307.5 |
  (4 below, 1 above range)

warm-clamp-minimum (n=40, range 388915.6-411913.1 ns)
  388915.6 |########################################
  390065.5 |###############
  391215.4 |#####
  392365.3 |###############
  393515.1 |#########################
  394665.0 |#####
  395814.9 |
  396964.8 |
  398114.6 |#####
  399264.5 |
  400414.4 |
  401564.3 |
  402714.1 |
  403864.0 |
  405013.9 |
  406163.8 |#####
  407313.6 |##########
  408463.5 |###############
  409613.4 |##########
  410763.3 |#####
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=21.8% (high variance, measurements may be unstable)
- **warm-clamp-head**: CV=26.8% (high variance, measurements may be unstable)
