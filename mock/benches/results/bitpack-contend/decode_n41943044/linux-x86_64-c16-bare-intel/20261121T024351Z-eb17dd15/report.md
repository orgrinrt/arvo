# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (272.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-pipe4 at 199.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-pipe4 beats baseline by 29% (significant)

bitpack-contend-pipe4 is -77.99 us (29%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-pipe4, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-pipe4 (199.63 us) and bitpack-contend-packed-simd (199.85 us) differ by 0.11%, inside the noise, even though the wider field spreads 36.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.80)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-packed-simd, bitpack-contend-pipe2} vs {bitpack-contend-d16-control, bitpack-contend-d16} (27% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-packed-simd, bitpack-contend-pipe2} and a slow tier {bitpack-contend-d16-control, bitpack-contend-d16} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 199625.6 ns median (-26.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.37x (fastest 199625.6 ns, slowest 272603.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 274757ns | 273070ns | 258842ns | 273226ns | 295265ns | base |
| bitpack-contend-d16-control | 264305ns | 254162ns | 246755ns | 258087ns | 300508ns | -3.80% |
| bitpack-contend-packed-simd | 202220ns | 200085ns | 183814ns | 201534ns | 222682ns | -26.40% |
| bitpack-contend-pipe2 | 202265ns | 200786ns | 195611ns | 202333ns | 208716ns | -26.38% |
| bitpack-contend-pipe4 | 199732ns | 199822ns | 193028ns | 200087ns | 205370ns | -27.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 274372ns | 258513ns | 294743ns | base | 15.287 |
| bitpack-contend-d16-control | 263954ns | 246552ns | 299963ns | -3.80% | 15.890 |
| bitpack-contend-packed-simd | 201956ns | 183597ns | 222432ns | -26.39% | 20.768 |
| bitpack-contend-pipe2 | 202028ns | 195397ns | 208433ns | -26.37% | 20.761 |
| bitpack-contend-pipe4 | 199528ns | 192801ns | 205163ns | -27.28% | 21.021 |

## Performance model

- Peak throughput: **22.845 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 15.386 | 67.3% |
| bitpack-contend-d16-control | 16.531 | 72.4% |
| bitpack-contend-packed-simd | 20.987 | 91.9% |
| bitpack-contend-pipe2 | 20.913 | 91.5% |
| bitpack-contend-pipe4 | 21.011 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 274757ns | 274757ns | base |
| bitpack-contend-d16-control | 264305ns | 264305ns | -3.80% |
| bitpack-contend-packed-simd | 202220ns | 202220ns | -26.40% |
| bitpack-contend-pipe2 | 202265ns | 202265ns | -26.38% |
| bitpack-contend-pipe4 | 199732ns | 199732ns | -27.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 272603ns | base | --- | [265344, 278979] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 253716ns | -10710.9ns (-3.9%) | [-13722, -8729]ns | [251194, 258187] | YES | 0.0064 | 0.0064 | 0 |
| bitpack-contend-packed-simd | 199853ns | -68186.2ns (-25.0%) | [-78465, -63965]ns | [199651, 202726] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 200562ns | -70946.9ns (-26.0%) | [-74347, -67687]ns | [200122, 204359] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 199626ns | -77992.3ns (-28.6%) | [-80145, -65838]ns | [199528, 199919] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 278979ns | -10.2% | -28.4% | -26.3% | -28.4% |
| 1 | 2 | 261031ns | -4.8% | -25.3% | -23.3% | -23.6% |
| 1 | 3 | 286532ns | +2.6% | -21.8% | -28.7% | -29.1% |
| 1 | 4 | 265199ns | -3.9% | -23.5% | -26.1% | -24.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.801 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.361 | moderate- |
| bitpack-contend-packed-simd | -0.365 | moderate- |
| bitpack-contend-pipe2 | -0.426 | moderate- |
| bitpack-contend-pipe4 | -0.436 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 3/4, lost 1/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 43.4ns | 274371.7ns | 0.0% |  |
| bitpack-contend-d16-control | 36.4ns | 263953.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 60.8ns | 201956.0ns | 0.0% |  |
| bitpack-contend-pipe2 | 23.5ns | 202027.8ns | 0.0% |  |
| bitpack-contend-pipe4 | 23.8ns | 199527.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 258512.8-294743.1 ns)
  258512.8 |##########
  260324.3 |##########
  262135.8 |#####
  263947.3 |########################################
  265758.9 |##########
  267570.4 |
  269381.9 |
  271193.4 |#####
  273004.9 |#####
  274816.4 |#####
  276628.0 |###############
  278439.5 |##########
  280251.0 |
  282062.5 |###############
  283874.0 |#####
  285685.5 |##########
  287497.1 |#####
  289308.6 |#####
  291120.1 |#####
  292931.6 |#####
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 246551.8-299962.5 ns)
  246551.8 |######
  249222.4 |########################################
  251892.9 |#############
  254563.4 |################
  257234.0 |######
  259904.5 |
  262575.0 |###
  265245.6 |
  267916.1 |
  270586.6 |
  273257.2 |
  275927.7 |
  278598.2 |###
  281268.8 |
  283939.3 |
  286609.8 |######
  289280.4 |##########
  291950.9 |
  294621.4 |###
  297292.0 |
  (3 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 183596.8-222431.7 ns)
  183596.8 |
  185538.6 |
  187480.3 |
  189422.1 |###
  191363.8 |
  193305.5 |######
  195247.3 |###
  197189.0 |######
  199130.8 |########################################
  201072.5 |##################
  203014.3 |######
  204956.0 |###
  206897.8 |
  208839.5 |###
  210781.2 |
  212723.0 |###
  214664.7 |
  216606.5 |###
  218548.2 |
  220490.0 |
  (3 below, 6 above range)

bitpack-contend-pipe2 (n=40, range 195397.2-208432.8 ns)
  195397.2 |#################
  196049.0 |########
  196700.8 |
  197352.6 |
  198004.3 |
  198656.1 |########
  199307.9 |####
  199959.7 |########################################
  200611.5 |####
  201263.2 |
  201915.0 |
  202566.8 |
  203218.6 |########
  203870.4 |#################
  204522.1 |########
  205173.9 |######################
  205825.7 |########
  206477.5 |########
  207129.3 |
  207781.0 |####
  (2 below, 1 above range)

bitpack-contend-pipe4 (n=40, range 192801.2-205163.5 ns)
  192801.2 |
  193419.3 |
  194037.4 |###
  194655.5 |###
  195273.6 |
  195891.8 |
  196509.9 |
  197128.0 |
  197746.1 |
  198364.2 |###
  198982.3 |########################################
  199600.4 |####################################
  200218.6 |
  200836.7 |
  201454.8 |###
  202072.9 |
  202691.0 |################
  203309.1 |##########
  203927.2 |
  204545.4 |
  (4 below, 1 above range)

```
