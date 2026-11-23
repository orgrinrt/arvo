# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe4 beats baseline by 23% (significant)

bitpack-contend-pipe4 is -135.71 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-pipe4, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-pipe4 (461.07 us) and bitpack-contend-packed-simd (461.88 us) differ by 0.18%, inside the noise, even though the wider field spreads 81.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.62)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 461065.2 ns median (-22.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.82x (fastest 461065.2 ns, slowest 837756.9 ns)

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
| bitpack-contend-d16 | 580754ns | 594007ns | 535832ns | 585531ns | 611343ns | base |
| bitpack-contend-d16-padal | 574720ns | 593089ns | 504917ns | 585584ns | 611930ns | -1.04% |
| bitpack-contend-d32 | 814414ns | 828818ns | 758413ns | 821057ns | 850487ns | +40.23% |
| bitpack-contend-d32-padal | 829621ns | 839166ns | 782857ns | 832931ns | 866452ns | +42.85% |
| bitpack-contend-packed-simd | 461387ns | 462481ns | 430108ns | 461461ns | 492444ns | -20.55% |
| bitpack-contend-pipe4 | 457182ns | 461507ns | 429106ns | 456769ns | 486495ns | -21.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 580028ns | 535015ns | 610525ns | base | 14.462 |
| bitpack-contend-d16-padal | 574014ns | 504250ns | 611173ns | -1.04% | 14.614 |
| bitpack-contend-d32 | 812994ns | 757005ns | 849110ns | +40.16% | 10.318 |
| bitpack-contend-d32-padal | 828296ns | 781491ns | 865156ns | +42.80% | 10.128 |
| bitpack-contend-packed-simd | 460817ns | 429519ns | 491718ns | -20.55% | 18.204 |
| bitpack-contend-pipe4 | 456665ns | 428589ns | 485883ns | -21.27% | 18.369 |

## Performance model

- Peak throughput: **19.573 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 14.136 | 72.2% |
| bitpack-contend-d16-padal | 14.161 | 72.4% |
| bitpack-contend-d32 | 10.139 | 51.8% |
| bitpack-contend-d32-padal | 10.013 | 51.2% |
| bitpack-contend-packed-simd | 18.162 | 92.8% |
| bitpack-contend-pipe4 | 18.194 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 580754ns | 580754ns | base |
| bitpack-contend-d16-padal | 574720ns | 574720ns | -1.04% |
| bitpack-contend-d32 | 814414ns | 814414ns | +40.23% |
| bitpack-contend-d32-padal | 829621ns | 829621ns | +42.85% |
| bitpack-contend-packed-simd | 461387ns | 461387ns | -20.55% |
| bitpack-contend-pipe4 | 457182ns | 457182ns | -21.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 593406ns | base | --- | [576171, 594647] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 592372ns | +8425.0ns (+1.4%) | [+4718, +18363]ns | [586712, 596380] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-contend-d32 | 827384ns | +232077.2ns (+39.1%) | [+208217, +249088]ns | [818520, 830989] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 837757ns | +244334.1ns (+41.2%) | [+226948, +254569]ns | [825178, 845702] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 461876ns | -133710.6ns (-22.5%) | [-137694, -116111]ns | [461257, 463037] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 461065ns | -135709.4ns (-22.9%) | [-147483, -120410]ns | [450593, 470015] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 579362ns | +2.4% | +43.3% | +39.0% | -25.8% | -22.2% |
| 1 | 2 | 597073ns | -15.4% | +37.3% | +33.2% | -22.7% | -22.8% |
| 1 | 3 | 535327ns | +9.7% | +57.7% | +59.8% | -13.5% | -12.1% |
| 1 | 4 | 594118ns | +1.3% | +27.6% | +41.1% | -19.6% | -27.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.621 | HIGH- (thermal bounce) |
| bitpack-contend-d16-padal | -0.329 | moderate- |
| bitpack-contend-d32 | -0.324 | moderate- |
| bitpack-contend-d32-padal | 0.037 | ok |
| bitpack-contend-packed-simd | 0.011 | ok |
| bitpack-contend-pipe4 | -0.313 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 106.2ns | 580028.5ns | 0.0% |  |
| bitpack-contend-d16-padal | 46.0ns | 574014.0ns | 0.0% |  |
| bitpack-contend-d32 | 214.9ns | 812993.8ns | 0.0% |  |
| bitpack-contend-d32-padal | 80.1ns | 828295.9ns | 0.0% |  |
| bitpack-contend-packed-simd | 46.2ns | 460816.6ns | 0.0% |  |
| bitpack-contend-pipe4 | 40.9ns | 456664.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 535014.6-610525.3 ns)
  535014.6 |####################
  538790.1 |
  542565.7 |####
  546341.2 |
  550116.7 |
  553892.3 |
  557667.8 |
  561443.4 |
  565218.9 |
  568994.4 |########
  572770.0 |############
  576545.5 |
  580321.0 |####
  584096.6 |
  587872.1 |############
  591647.7 |########################################
  595423.2 |############################
  599198.7 |####
  602974.3 |
  606749.8 |####
  (4 below, 2 above range)

bitpack-contend-d16-padal (n=40, range 504249.7-611173.1 ns)
  504249.7 |#################
  509595.8 |
  514942.0 |
  520288.2 |
  525634.4 |
  530980.5 |####
  536326.7 |
  541672.9 |
  547019.1 |
  552365.2 |
  557711.4 |
  563057.6 |########
  568403.7 |
  573749.9 |
  579096.1 |
  584442.3 |###############################
  589788.4 |###############################
  595134.6 |########
  600480.8 |########################################
  605827.0 |
  (5 below, 3 above range)

bitpack-contend-d32 (n=40, range 757005.3-849109.6 ns)
  757005.3 |########
  761610.6 |########################
  766215.8 |########
  770821.0 |
  775426.2 |
  780031.4 |
  784636.6 |################
  789241.8 |
  793847.0 |
  798452.2 |
  803057.5 |
  807662.7 |
  812267.9 |########
  816873.1 |########################################
  821478.3 |################
  826083.5 |########################################
  830688.7 |################
  835293.9 |################################
  839899.1 |########################
  844504.4 |################################
  (5 below, 2 above range)

bitpack-contend-d32-padal (n=40, range 781491.0-865155.9 ns)
  781491.0 |##########################
  785674.2 |######
  789857.5 |####################
  794040.7 |####################
  798224.0 |
  802407.2 |
  806590.5 |
  810773.7 |
  814957.0 |#############
  819140.2 |
  823323.5 |
  827506.7 |
  831690.0 |####################
  835873.2 |########################################
  840056.4 |#############
  844239.7 |########################################
  848422.9 |#############
  852606.2 |##########################
  856789.4 |
  860972.7 |######
  (1 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 429518.6-491717.7 ns)
  429518.6 |##################
  432628.6 |
  435738.5 |
  438848.5 |##
  441958.4 |##
  445068.4 |
  448178.3 |
  451288.3 |
  454398.2 |
  457508.2 |
  460618.1 |########################################
  463728.1 |##
  466838.1 |
  469948.0 |
  473058.0 |##
  476167.9 |#########
  479277.9 |##
  482387.8 |##
  485497.8 |
  488607.7 |####
  (1 below, 2 above range)

bitpack-contend-pipe4 (n=40, range 428588.8-485883.3 ns)
  428588.8 |########################################
  431453.5 |########
  434318.2 |
  437182.9 |
  440047.7 |
  442912.4 |
  445777.1 |####
  448641.9 |
  451506.6 |#############
  454371.3 |
  457236.0 |
  460100.8 |###################################
  462965.5 |
  465830.2 |
  468694.9 |###############################
  471559.7 |####
  474424.4 |########
  477289.1 |####
  480153.8 |
  483018.6 |
  (2 below, 4 above range)

```
