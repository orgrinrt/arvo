# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed-naive is an outlier: 6.7x slower than the field

bitpack-footprint-packed-naive (594.73 us) is 6.7x the fastest (88.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-footprint-dense-alt, bitpack-footprint-dense) are a dead heat (<1%)

bitpack-footprint-dense-alt (88.96 us) and bitpack-footprint-dense (89.00 us) differ by 0.05%, inside the noise, even though the wider field spreads 568.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-footprint-dense-alt shows warm-up / thermal drift (autocorr +0.82)

bitpack-footprint-dense-alt's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} vs {bitpack-footprint-packed-naive} (346% apart)

The field splits into a fast tier {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} and a slow tier {bitpack-footprint-packed-naive} with a 346% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.7x the fastest

Fastest bitpack-footprint-dense-alt (88.96 us) to slowest bitpack-footprint-packed-naive (594.73 us): 6.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 88956.9 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 6.69x (fastest 88956.9 ns, slowest 594734.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 89779ns | 89195ns | 88941ns | 89421ns | 91694ns | base |
| bitpack-footprint-dense-alt | 89842ns | 89178ns | 88930ns | 89389ns | 92114ns | +0.07% |
| bitpack-footprint-packed | 133926ns | 133586ns | 133213ns | 133679ns | 135380ns | +49.17% |
| bitpack-footprint-packed-naive | 597414ns | 595310ns | 592706ns | 595666ns | 607364ns | +565.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 89608ns | 88793ns | 91545ns | base | 11.702 |
| bitpack-footprint-dense-alt | 89668ns | 88801ns | 91973ns | +0.07% | 11.694 |
| bitpack-footprint-packed | 133680ns | 133033ns | 135104ns | +49.18% | 7.844 |
| bitpack-footprint-packed-naive | 596823ns | 592017ns | 607032ns | +566.04% | 1.757 |

## Performance model

- Peak throughput: **11.809 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 11.781 | 99.8% |
| bitpack-footprint-dense-alt | 11.787 | 99.8% |
| bitpack-footprint-packed | 7.865 | 66.6% |
| bitpack-footprint-packed-naive | 1.763 | 14.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 89779ns | 89779ns | base |
| bitpack-footprint-dense-alt | 89842ns | 89842ns | +0.07% |
| bitpack-footprint-packed | 133926ns | 133926ns | +49.17% |
| bitpack-footprint-packed-naive | 597414ns | 597414ns | +565.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 89003ns | base | --- | [88874, 89261] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 88957ns | no significant difference | [-41, +102]ns | [88890, 89161] | no | 0.4296 | 0.4296 | 0 |
| bitpack-footprint-packed | 133330ns | +44225.2ns (+49.7%) | [+43940, +44384]ns | [133189, 133638] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 594734ns | +505135.2ns (+567.5%) | [+503797, +506519]ns | [593888, 596267] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|
| 1 | 93387ns | -1.2% | +44.7% | +535.9% |
| 2 | 91757ns | +1.0% | +45.2% | +545.3% |
| 3 | 91178ns | +0.8% | +46.4% | +554.5% |
| 4 | 91020ns | -0.1% | +46.1% | +557.5% |
| 5 | 91310ns | +1.0% | +45.8% | +551.5% |
| 6 | 91172ns | +0.1% | +46.3% | +551.4% |
| 7 | 91012ns | +0.9% | +47.1% | +558.2% |
| 8 | 91308ns | -0.4% | +46.8% | +551.3% |
| 9 | 91191ns | +1.5% | +46.6% | +552.2% |
| 10 | 91055ns | -0.0% | +46.3% | +552.7% |
| 11 | 89795ns | -0.6% | +51.7% | +562.6% |
| 12 | 88881ns | +0.0% | +52.9% | +587.6% |
| 13 | 89802ns | -1.1% | +48.8% | +565.3% |
| 14 | 89617ns | -0.7% | +48.7% | +575.3% |
| 15 | 88855ns | -0.0% | +50.1% | +582.8% |
| 16 | 89321ns | -0.4% | +49.1% | +577.2% |
| 17 | 89200ns | -0.4% | +49.3% | +577.4% |
| 18 | 88988ns | -0.1% | +49.6% | +592.5% |
| 19 | 89018ns | +0.1% | +50.2% | +577.9% |
| 20 | 88799ns | +0.0% | +51.2% | +580.8% |
| 21 | 88806ns | +0.5% | +50.1% | +567.6% |
| 22 | 89041ns | -0.2% | +50.3% | +565.9% |
| 23 | 88863ns | -0.0% | +50.4% | +567.3% |
| 24 | 88755ns | +0.0% | +52.4% | +567.4% |
| 25 | 88809ns | +0.3% | +51.8% | +566.5% |
| 26 | 88819ns | +0.1% | +51.8% | +567.0% |
| 27 | 89181ns | -0.4% | +49.6% | +566.9% |
| 28 | 88851ns | +0.1% | +51.3% | +565.9% |
| 29 | 88885ns | +0.2% | +50.3% | +567.8% |
| 30 | 88823ns | +0.1% | +49.8% | +571.5% |
| 31 | 89036ns | -0.2% | +49.7% | +567.3% |
| 32 | 88878ns | +0.5% | +49.7% | +566.7% |
| 33 | 88870ns | -0.1% | +49.7% | +565.5% |
| 34 | 88762ns | +0.0% | +50.2% | +567.9% |
| 35 | 89080ns | -0.3% | +49.3% | +569.1% |
| 36 | 88887ns | +1.1% | +49.8% | +569.2% |
| 37 | 88879ns | +0.2% | +49.8% | +568.5% |
| 38 | 88809ns | +0.2% | +49.9% | +569.8% |
| 39 | 88789ns | +0.2% | +49.8% | +572.5% |
| 40 | 88833ns | +0.1% | +49.8% | +566.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.778 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.817 | HIGH+ (drift/warm-up) |
| bitpack-footprint-packed | 0.410 | moderate+ |
| bitpack-footprint-packed-naive | 0.552 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 12/40, lost 14/40
- **bitpack-footprint-packed**: won 0/40, lost 40/40
- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 3.2ns | 89608.1ns | 0.0% |  |
| bitpack-footprint-dense-alt | 2.7ns | 89667.7ns | 0.0% |  |
| bitpack-footprint-packed | 2.7ns | 133680.5ns | 0.0% |  |
| bitpack-footprint-packed-naive | 9.0ns | 596823.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 88793.4-91544.7 ns)
  88793.4 |########################################
  88931.0 |##########
  89068.6 |#######
  89206.1 |##
  89343.7 |
  89481.3 |##
  89618.8 |
  89756.4 |#####
  89894.0 |
  90031.5 |
  90169.1 |
  90306.7 |
  90444.2 |
  90581.8 |
  90719.3 |
  90856.9 |
  90994.5 |#######
  91132.0 |#######
  91269.6 |#####
  91407.2 |
  (3 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 88801.2-91972.8 ns)
  88801.2 |########################################
  88959.8 |###############
  89118.4 |#####
  89277.0 |##
  89435.6 |
  89594.1 |
  89752.7 |##
  89911.3 |
  90069.9 |
  90228.4 |
  90387.0 |
  90545.6 |
  90704.2 |
  90862.7 |#####
  91021.3 |##
  91179.9 |##
  91338.5 |
  91497.0 |
  91655.6 |
  91814.2 |#####
  (4 below, 4 above range)

bitpack-footprint-packed (n=40, range 133032.8-135104.4 ns)
  133032.8 |########################################
  133136.3 |########################################
  133239.9 |##########################
  133343.5 |####################
  133447.1 |######
  133550.7 |####################
  133654.2 |#############
  133757.8 |#############
  133861.4 |
  133965.0 |
  134068.6 |######
  134172.2 |######
  134275.7 |
  134379.3 |######
  134482.9 |
  134586.5 |
  134690.1 |
  134793.6 |#############
  134897.2 |
  135000.8 |
  (4 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 592017.1-607031.6 ns)
  592017.1 |####################
  592767.8 |####################
  593518.6 |#########################
  594269.3 |########################################
  595020.0 |
  595770.7 |##########
  596521.5 |##########
  597272.2 |#####
  598022.9 |#####
  598773.6 |#####
  599524.3 |
  600275.1 |
  601025.8 |
  601776.5 |
  602527.2 |
  603278.0 |#####
  604028.7 |##########
  604779.4 |##########
  605530.1 |
  606280.8 |#####
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **bitpack-footprint-dense-alt**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **bitpack-footprint-packed-naive**: autocorrelation=0.55 (measurement drift or warm-up artifact)
