# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe4 beats baseline by 30% (significant)

bitpack-contend-pipe4 is -79.14 us (30%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.78)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-packed-simd} vs {bitpack-contend-d16-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-d32-padal} (35% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-packed-simd} and a slow tier {bitpack-contend-d16-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-d32-padal} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 178421.0 ns median (-32.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.79x (fastest 178421.0 ns, slowest 319791.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761890 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 258216ns | 263550ns | 217133ns | 260202ns | 293342ns | base |
| bitpack-contend-d16-padal | 262472ns | 256626ns | 252338ns | 258803ns | 283611ns | +1.65% |
| bitpack-contend-d32 | 308007ns | 309124ns | 278883ns | 310578ns | 329417ns | +19.28% |
| bitpack-contend-d32-padal | 319238ns | 320636ns | 292209ns | 317497ns | 351491ns | +23.63% |
| bitpack-contend-packed-simd | 188576ns | 189559ns | 171437ns | 189154ns | 203981ns | -26.97% |
| bitpack-contend-pipe4 | 179741ns | 178606ns | 162695ns | 180017ns | 195961ns | -30.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 257869ns | 216732ns | 292997ns | base | 16.265 |
| bitpack-contend-d16-padal | 262167ns | 252125ns | 283217ns | +1.67% | 15.999 |
| bitpack-contend-d32 | 307322ns | 278085ns | 328712ns | +19.18% | 13.648 |
| bitpack-contend-d32-padal | 318541ns | 291594ns | 350888ns | +23.53% | 13.167 |
| bitpack-contend-packed-simd | 188361ns | 171207ns | 203740ns | -26.95% | 22.267 |
| bitpack-contend-pipe4 | 179549ns | 162504ns | 195778ns | -30.37% | 23.360 |

## Performance model

- Peak throughput: **25.810 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 15.934 | 61.7% |
| bitpack-contend-d16-padal | 16.365 | 63.4% |
| bitpack-contend-d32 | 13.594 | 52.7% |
| bitpack-contend-d32-padal | 13.116 | 50.8% |
| bitpack-contend-packed-simd | 22.148 | 85.8% |
| bitpack-contend-pipe4 | 23.508 | 91.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 258216ns | 258216ns | base |
| bitpack-contend-d16-padal | 262472ns | 262472ns | +1.65% |
| bitpack-contend-d32 | 308007ns | 308007ns | +19.28% |
| bitpack-contend-d32-padal | 319238ns | 319238ns | +23.63% |
| bitpack-contend-packed-simd | 188576ns | 188576ns | -26.97% |
| bitpack-contend-pipe4 | 179741ns | 179741ns | -30.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 263228ns | base | --- | [236017, 281760] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 256290ns | no significant difference | [-4829, +16682]ns | [252458, 259225] | no | 1.0000 | 1.0000 | 0 |
| bitpack-contend-d32 | 308541ns | +41820.0ns (+15.9%) | [+28161, +61511]ns | [303120, 319184] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 319791ns | +69424.4ns (+26.4%) | [+61460, +72723]ns | [300208, 322488] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 189374ns | -72821.4ns (-27.7%) | [-89238, -40263]ns | [178252, 200082] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 178421ns | -79144.2ns (-30.1%) | [-87873, -71668]ns | [168320, 189332] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 235962ns | +7.0% | +17.5% | +28.6% | -24.5% | -26.2% |
| 1 | 2 | 286802ns | -2.6% | +8.6% | +22.3% | -30.1% | -34.0% |
| 1 | 3 | 226823ns | +11.2% | +40.0% | +32.5% | -11.7% | -28.4% |
| 1 | 4 | 285689ns | -9.3% | +8.5% | +8.5% | -40.1% | -31.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.785 | HIGH- (thermal bounce) |
| bitpack-contend-d16-padal | -0.607 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.017 | ok |
| bitpack-contend-d32-padal | -0.536 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.240 | moderate- |
| bitpack-contend-pipe4 | -0.739 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 51.6ns | 257869.2ns | 0.0% |  |
| bitpack-contend-d16-padal | 25.3ns | 262167.0ns | 0.0% |  |
| bitpack-contend-d32 | 91.2ns | 307321.7ns | 0.0% |  |
| bitpack-contend-d32-padal | 38.6ns | 318541.4ns | 0.0% |  |
| bitpack-contend-packed-simd | 35.4ns | 188361.4ns | 0.0% |  |
| bitpack-contend-pipe4 | 22.5ns | 179549.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 216732.2-292997.1 ns)
  216732.2 |########
  220545.5 |########################
  224358.7 |################################
  228172.0 |########
  231985.2 |########
  235798.5 |########
  239611.7 |
  243424.9 |
  247238.2 |################################
  251051.4 |########
  254864.7 |
  258677.9 |
  262491.2 |
  266304.4 |
  270117.7 |
  273930.9 |################################
  277744.1 |################
  281557.4 |################################
  285370.6 |########################
  289183.9 |########################################
  (4 below, 2 above range)

bitpack-contend-d16-padal (n=40, range 252125.0-283217.1 ns)
  252125.0 |########################################
  253679.6 |########
  255234.2 |
  256788.8 |##
  258343.4 |#################
  259898.0 |
  261452.6 |
  263007.2 |##
  264561.8 |
  266116.4 |
  267671.0 |
  269225.6 |
  270780.2 |
  272334.8 |########
  273889.4 |##
  275444.0 |##
  276998.7 |
  278553.3 |
  280107.9 |##
  281662.5 |#####
  (3 below, 4 above range)

bitpack-contend-d32 (n=40, range 278084.8-328712.2 ns)
  278084.8 |
  280616.2 |##########
  283147.6 |##########
  285678.9 |
  288210.3 |
  290741.7 |
  293273.1 |
  295804.4 |########################################
  298335.8 |
  300867.2 |########################################
  303398.5 |####################
  305929.9 |##############################
  308461.3 |########################################
  310992.6 |
  313524.0 |
  316055.4 |
  318586.7 |########################################
  321118.1 |####################
  323649.5 |##############################
  326180.9 |####################
  (5 below, 5 above range)

bitpack-contend-d32-padal (n=40, range 291594.5-350887.7 ns)
  291594.5 |########################################
  294559.1 |
  297523.8 |########################################
  300488.5 |######
  303453.1 |
  306417.8 |
  309382.4 |######
  312347.1 |######
  315311.8 |
  318276.4 |#################################
  321241.1 |##########################
  324205.7 |
  327170.4 |######
  330135.1 |
  333099.7 |######
  336064.4 |######
  339029.0 |
  341993.7 |
  344958.3 |######
  347923.0 |##########################
  (3 below, 5 above range)

bitpack-contend-packed-simd (n=40, range 171206.7-203740.5 ns)
  171206.7 |#####
  172833.4 |#####
  174460.1 |
  176086.8 |
  177713.5 |#########################
  179340.1 |##
  180966.8 |
  182593.5 |
  184220.2 |
  185846.9 |
  187473.6 |
  189100.3 |
  190727.0 |
  192353.6 |
  193980.3 |
  195607.0 |
  197233.7 |
  198860.4 |########################################
  200487.1 |#######
  202113.8 |
  (5 below, 1 above range)

bitpack-contend-pipe4 (n=40, range 162504.4-195778.5 ns)
  162504.4 |####################
  164168.1 |
  165831.8 |##############################
  167495.5 |
  169159.2 |#####
  170822.9 |
  172486.6 |
  174150.3 |
  175814.0 |
  177477.7 |########################################
  179141.4 |
  180805.1 |
  182468.8 |
  184132.6 |
  185796.3 |#####
  187460.0 |#####
  189123.7 |##########
  190787.4 |#####
  192451.1 |####################
  194114.8 |#########################
  (4 below, 3 above range)

```
