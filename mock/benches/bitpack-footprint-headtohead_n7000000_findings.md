# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed-naive is an outlier: 6.4x slower than the field

bitpack-footprint-packed-naive (4.00 ms) is 6.4x the fastest (623.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-footprint-dense-alt, bitpack-footprint-dense) are a dead heat (<1%)

bitpack-footprint-dense-alt (623.02 us) and bitpack-footprint-dense (623.07 us) differ by 0.01%, inside the noise, even though the wider field spreads 541.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} vs {bitpack-footprint-packed-naive} (336% apart)

The field splits into a fast tier {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} and a slow tier {bitpack-footprint-packed-naive} with a 336% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.4x the fastest

Fastest bitpack-footprint-dense-alt (623.02 us) to slowest bitpack-footprint-packed-naive (4.00 ms): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 623017.1 ns median (-0.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 6.42x (fastest 623017.1 ns, slowest 3996798.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 626527ns | 624918ns | 618583ns | 625958ns | 636177ns | base |
| bitpack-footprint-dense-alt | 624841ns | 623839ns | 620328ns | 624031ns | 631783ns | -0.27% |
| bitpack-footprint-packed | 919161ns | 917652ns | 915357ns | 917949ns | 926599ns | +46.71% |
| bitpack-footprint-packed-naive | 4060481ns | 3998436ns | 3990649ns | 3999816ns | 4312307ns | +548.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 625428ns | 617683ns | 635562ns | base | 11.192 |
| bitpack-footprint-dense-alt | 623927ns | 619574ns | 630942ns | -0.24% | 11.219 |
| bitpack-footprint-packed | 918099ns | 914175ns | 925572ns | +46.80% | 7.624 |
| bitpack-footprint-packed-naive | 4058904ns | 3989047ns | 4310822ns | +548.98% | 1.725 |

## Performance model

- Peak throughput: **11.333 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 11.235 | 99.1% |
| bitpack-footprint-dense-alt | 11.236 | 99.1% |
| bitpack-footprint-packed | 7.637 | 67.4% |
| bitpack-footprint-packed-naive | 1.751 | 15.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 626527ns | 626527ns | base |
| bitpack-footprint-dense-alt | 624841ns | 624841ns | -0.27% |
| bitpack-footprint-packed | 919161ns | 919161ns | +46.71% |
| bitpack-footprint-packed-naive | 4060481ns | 4060481ns | +548.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 623073ns | base | --- | [621903, 628363] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 623017ns | no significant difference | [-4534, +1502]ns | [621844, 624551] | no | 0.2682 | 0.2682 | 0 |
| bitpack-footprint-packed | 916626ns | +293309.6ns (+47.1%) | [+287394, +295764]ns | [915755, 918357] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 3996799ns | +3373875.0ns (+541.5%) | [+3368081, +3379580]ns | [3993135, 4000858] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|
| 1 | 622950ns | +0.3% | +50.5% | +542.5% |
| 2 | 621979ns | +0.6% | +47.2% | +546.2% |
| 3 | 622181ns | +0.2% | +47.1% | +543.7% |
| 4 | 629559ns | -0.7% | +45.3% | +534.4% |
| 5 | 626170ns | -0.2% | +46.0% | +537.5% |
| 6 | 628804ns | -0.8% | +45.4% | +700.4% |
| 7 | 621955ns | +2.4% | +47.1% | +542.7% |
| 8 | 622392ns | +1.2% | +47.3% | +548.9% |
| 9 | 623339ns | +1.4% | +46.7% | +540.3% |
| 10 | 621851ns | -0.4% | +47.3% | +546.2% |
| 11 | 621594ns | -0.2% | +47.9% | +541.8% |
| 12 | 614020ns | +1.0% | +49.7% | +549.6% |
| 13 | 613970ns | +0.9% | +51.4% | +551.0% |
| 14 | 619672ns | +0.1% | +47.9% | +544.3% |
| 15 | 621083ns | -0.1% | +47.6% | +549.1% |
| 16 | 623142ns | -0.5% | +46.5% | +540.8% |
| 17 | 630765ns | -1.4% | +45.1% | +532.1% |
| 18 | 620423ns | +0.3% | +47.7% | +544.7% |
| 19 | 630315ns | -1.6% | +45.1% | +533.6% |
| 20 | 626582ns | -1.3% | +45.8% | +537.0% |
| 21 | 618705ns | +1.4% | +50.6% | +547.9% |
| 22 | 617213ns | +1.7% | +48.8% | +546.9% |
| 23 | 639024ns | -1.8% | +43.8% | +524.1% |
| 24 | 620879ns | +1.3% | +48.1% | +542.5% |
| 25 | 623004ns | -0.2% | +47.6% | +542.1% |
| 26 | 619331ns | -0.1% | +48.3% | +555.6% |
| 27 | 618306ns | +1.0% | +48.5% | +738.8% |
| 28 | 623506ns | -0.5% | +47.3% | +540.4% |
| 29 | 620933ns | -0.1% | +48.0% | +543.6% |
| 30 | 620250ns | +0.8% | +48.4% | +545.8% |
| 31 | 640120ns | -2.8% | +43.8% | +523.4% |
| 32 | 631319ns | -1.4% | +44.9% | +543.9% |
| 33 | 634147ns | -1.7% | +44.3% | +530.4% |
| 34 | 638336ns | -2.4% | +43.5% | +525.4% |
| 35 | 629002ns | -0.8% | +45.6% | +535.1% |
| 36 | 636445ns | -1.7% | +44.0% | +526.6% |
| 37 | 633777ns | +0.5% | +44.7% | +531.4% |
| 38 | 627922ns | -0.9% | +45.9% | +545.3% |
| 39 | 630823ns | -1.6% | +46.8% | +533.5% |
| 40 | 631324ns | -1.2% | +45.2% | +533.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.381 | moderate+ |
| bitpack-footprint-dense-alt | 0.331 | moderate+ |
| bitpack-footprint-packed | 0.030 | ok |
| bitpack-footprint-packed-naive | -0.039 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 23/40, lost 16/40
- **bitpack-footprint-packed**: won 0/40, lost 40/40
- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 8.4ns | 625427.8ns | 0.0% |  |
| bitpack-footprint-dense-alt | 8.0ns | 623927.1ns | 0.0% |  |
| bitpack-footprint-packed | 10.7ns | 918098.9ns | 0.0% |  |
| bitpack-footprint-packed-naive | 48.2ns | 4058903.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 617683.4-635561.6 ns)
  617683.4 |##########
  618577.3 |####################
  619471.2 |####################
  620365.1 |########################################
  621259.1 |########################################
  622153.0 |########################################
  623046.9 |##############################
  623940.8 |
  624834.7 |
  625728.6 |####################
  626622.5 |
  627516.4 |##########
  628410.3 |####################
  629304.2 |##########
  630198.1 |##############################
  631092.0 |####################
  631985.9 |
  632879.9 |
  633773.8 |####################
  634667.7 |
  (3 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 619574.4-630941.8 ns)
  619574.4 |####################
  620142.7 |########################################
  620711.1 |######
  621279.5 |######
  621847.8 |####################
  622416.2 |####################
  622984.6 |#############
  623553.0 |####################
  624121.3 |######
  624689.7 |####################
  625258.1 |####################
  625826.5 |
  626394.8 |
  626963.2 |#############
  627531.6 |######
  628100.0 |
  628668.3 |######
  629236.7 |######
  629805.1 |
  630373.5 |
  (3 below, 3 above range)

bitpack-footprint-packed (n=40, range 914174.9-925572.0 ns)
  914174.9 |########################
  914744.8 |########################################
  915314.6 |########################
  915884.5 |################################
  916454.3 |########################################
  917024.2 |
  917594.0 |
  918163.9 |################################
  918733.7 |########################
  919303.6 |########################
  919873.4 |########
  920443.3 |########
  921013.1 |
  921583.0 |
  922152.8 |
  922722.7 |
  923292.6 |
  923862.4 |
  924432.3 |
  925002.1 |
  (4 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 3989047.2-4310822.3 ns)
  3989047.2 |########################################
  4005135.9 |######
  4021224.7 |#
  4037313.5 |###
  4053402.2 |###
  4069491.0 |
  4085579.7 |
  4101668.5 |
  4117757.2 |
  4133846.0 |
  4149934.7 |
  4166023.5 |
  4182112.2 |
  4198201.0 |
  4214289.8 |
  4230378.5 |
  4246467.3 |
  4262556.0 |
  4278644.8 |
  4294733.5 |
  (4 below, 2 above range)

```
