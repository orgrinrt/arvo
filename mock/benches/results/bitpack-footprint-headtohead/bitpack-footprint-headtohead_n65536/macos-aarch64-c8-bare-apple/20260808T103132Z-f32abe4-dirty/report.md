# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed-naive is an outlier: 6.7x slower than the field

bitpack-footprint-packed-naive (37.13 us) is 6.7x the fastest (5.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-footprint-dense-alt, bitpack-footprint-dense) are a dead heat (<1%)

bitpack-footprint-dense-alt (5.54 us) and bitpack-footprint-dense (5.55 us) differ by 0.15%, inside the noise, even though the wider field spreads 570.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-footprint-packed shows warm-up / thermal drift (autocorr +0.64)

bitpack-footprint-packed's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} vs {bitpack-footprint-packed-naive} (347% apart)

The field splits into a fast tier {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} and a slow tier {bitpack-footprint-packed-naive} with a 347% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest bitpack-footprint-dense-alt (5.54 us) to slowest bitpack-footprint-packed-naive (37.13 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-dense-alt's edge over baseline is significant but tiny (0 ns, 0.00%)

bitpack-footprint-dense-alt differs from baseline bitpack-footprint-dense by 0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 5537.5 ns median (-0.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 6.70x (fastest 5537.5 ns, slowest 37126.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 5693ns | 5635ns | 5522ns | 5674ns | 5918ns | base |
| bitpack-footprint-dense-alt | 5631ns | 5629ns | 5554ns | 5632ns | 5707ns | -1.08% |
| bitpack-footprint-packed | 8381ns | 8379ns | 8224ns | 8357ns | 8610ns | +47.22% |
| bitpack-footprint-packed-naive | 37276ns | 37232ns | 37063ns | 37235ns | 37611ns | +554.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 5603ns | 5437ns | 5825ns | base | 11.696 |
| bitpack-footprint-dense-alt | 5539ns | 5465ns | 5614ns | -1.15% | 11.832 |
| bitpack-footprint-packed | 8313ns | 8159ns | 8540ns | +48.36% | 7.883 |
| bitpack-footprint-packed-naive | 37181ns | 36978ns | 37513ns | +563.54% | 1.763 |

## Performance model

- Peak throughput: **12.054 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 11.817 | 98.0% |
| bitpack-footprint-dense-alt | 11.835 | 98.2% |
| bitpack-footprint-packed | 7.885 | 65.4% |
| bitpack-footprint-packed-naive | 1.765 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 5693ns | 5693ns | base |
| bitpack-footprint-dense-alt | 5631ns | 5631ns | -1.08% |
| bitpack-footprint-packed | 8381ns | 8381ns | +47.22% |
| bitpack-footprint-packed-naive | 37276ns | 37276ns | +554.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 5546ns | base | --- | [5537, 5621] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 5538ns | no significant difference | [-104, +1]ns | [5537, 5539] | no | 0.1996 | 0.1996 | 1 |
| bitpack-footprint-packed | 8311ns | +2774.1ns (+50.0%) | [+2695, +2788]ns | [8273, 8316] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 37127ns | +31559.2ns (+569.0%) | [+31512, +31637]ns | [37103, 37169] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|
| 1 | 5460ns | +2.9% | +52.2% | +570.4% |
| 2 | 5434ns | +3.5% | +53.0% | +582.7% |
| 3 | 5432ns | +3.0% | +52.2% | +584.4% |
| 4 | 5436ns | +1.9% | +50.0% | +584.3% |
| 5 | 5434ns | -0.0% | +50.1% | +582.3% |
| 6 | 5634ns | -2.2% | +45.6% | +560.9% |
| 7 | 5436ns | -0.1% | +52.8% | +582.6% |
| 8 | 5618ns | -3.3% | +48.9% | +560.9% |
| 9 | 5431ns | +0.0% | +53.0% | +583.1% |
| 10 | 5433ns | +0.0% | +53.0% | +584.5% |
| 11 | 5827ns | -5.0% | +50.1% | +537.1% |
| 12 | 5823ns | -4.9% | +50.2% | +537.1% |
| 13 | 5825ns | -4.9% | +50.1% | +536.8% |
| 14 | 5826ns | -5.0% | +40.3% | +536.6% |
| 15 | 5826ns | -5.0% | +41.5% | +536.1% |
| 16 | 5825ns | -4.9% | +42.7% | +537.6% |
| 17 | 5825ns | -5.0% | +42.7% | +542.8% |
| 18 | 5616ns | -1.4% | +48.8% | +559.5% |
| 19 | 5538ns | +0.0% | +50.0% | +573.1% |
| 20 | 5535ns | +0.0% | +50.2% | +570.2% |
| 21 | 5641ns | -2.3% | +46.7% | +552.6% |
| 22 | 5678ns | -2.3% | +43.7% | +554.4% |
| 23 | 5624ns | -1.5% | +45.2% | +560.3% |
| 24 | 5822ns | -4.8% | +40.1% | +538.6% |
| 25 | 5684ns | -2.6% | +43.6% | +553.1% |
| 26 | 5808ns | -4.7% | +40.5% | +539.3% |
| 27 | 5591ns | -1.0% | +45.9% | +564.0% |
| 28 | 5537ns | -0.0% | +48.2% | +570.3% |
| 29 | 5592ns | -0.4% | +46.7% | +563.5% |
| 30 | 5540ns | +1.6% | +50.2% | +569.6% |
| 31 | 5538ns | +4.0% | +50.1% | +576.4% |
| 32 | 5538ns | -0.0% | +50.1% | +574.0% |
| 33 | 5541ns | -0.0% | +50.1% | +591.6% |
| 34 | 5538ns | +0.2% | +50.3% | +580.1% |
| 35 | 5535ns | +0.1% | +50.4% | +571.4% |
| 36 | 5536ns | +0.0% | +50.3% | +574.7% |
| 37 | 5536ns | +0.0% | +50.5% | +571.4% |
| 38 | 5552ns | +0.4% | +52.1% | +571.3% |
| 39 | 5537ns | +0.0% | +53.5% | +569.3% |
| 40 | 5551ns | -0.3% | +51.7% | +568.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.643 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.549 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed | 0.644 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed-naive | 0.331 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 19/40, lost 9/40
- **bitpack-footprint-packed**: won 0/40, lost 40/40
- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 2.2ns | 5603.3ns | 0.0% |  |
| bitpack-footprint-dense-alt | 2.1ns | 5538.8ns | 0.0% |  |
| bitpack-footprint-packed | 2.4ns | 8313.4ns | 0.0% |  |
| bitpack-footprint-packed-naive | 2.0ns | 37180.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 5437.0-5824.8 ns)
   5437.0 |
   5456.4 |##
   5475.8 |
   5495.2 |
   5514.5 |
   5533.9 |########################################
   5553.3 |
   5572.7 |#####
   5592.1 |
   5611.5 |########
   5630.9 |#####
   5650.3 |
   5669.7 |#####
   5689.1 |
   5708.5 |
   5727.9 |
   5747.3 |
   5766.7 |
   5786.1 |
   5805.4 |##############
  (7 below, 4 above range)

bitpack-footprint-dense-alt (n=40, range 5465.3-5614.2 ns)
   5465.3 |
   5472.8 |
   5480.2 |
   5487.6 |
   5495.1 |
   5502.5 |
   5510.0 |###
   5517.4 |
   5524.9 |
   5532.3 |########################################
   5539.7 |#####
   5547.2 |#
   5554.6 |
   5562.1 |#
   5569.5 |#
   5577.0 |
   5584.4 |
   5591.8 |#
   5599.3 |
   5606.7 |
  (5 below, 4 above range)

bitpack-footprint-packed (n=40, range 8159.3-8539.7 ns)
   8159.3 |#############
   8178.3 |
   8197.4 |##########
   8216.4 |
   8235.4 |###
   8254.4 |###
   8273.4 |###
   8292.4 |################
   8311.5 |########################################
   8330.5 |
   8349.5 |######
   8368.5 |
   8387.5 |
   8406.6 |###
   8425.6 |###
   8444.6 |
   8463.6 |
   8482.6 |###
   8501.6 |
   8520.7 |
  (5 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 36977.6-37512.6 ns)
  36977.6 |
  37004.4 |
  37031.1 |########
  37057.9 |########
  37084.6 |########################################
  37111.4 |############################
  37138.1 |########
  37164.9 |####################
  37191.6 |####
  37218.3 |####
  37245.1 |####
  37271.8 |####
  37298.6 |
  37325.3 |####
  37352.1 |####
  37378.8 |
  37405.6 |
  37432.3 |########
  37459.1 |
  37485.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **bitpack-footprint-dense-alt**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **bitpack-footprint-packed**: autocorrelation=0.64 (measurement drift or warm-up artifact)
