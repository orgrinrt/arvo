# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 64% (significant)

warm-clamp-minimum is -458 ns (64%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 18.8x slower than the field

warm-clamp-accfit-dyn (4.78 us) is 18.8x the fastest (254 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.81)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (571% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 571% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.8x the fastest

Fastest warm-clamp-minimum (254 ns) to slowest warm-clamp-accfit-dyn (4.78 us): 18.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-minimum** at 253.9 ns median (-64.3% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 18.83x (fastest 253.9 ns, slowest 4780.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 774ns | 774ns | 771ns | 774ns | 779ns | base |
| warm-clamp-accfit | 442ns | 439ns | 434ns | 440ns | 455ns | -42.97% |
| warm-clamp-accfit-dyn | 4855ns | 4839ns | 4715ns | 4827ns | 5081ns | +526.94% |
| warm-clamp-head | 625ns | 630ns | 607ns | 625ns | 645ns | -19.26% |
| warm-clamp-min-lanes | 322ns | 324ns | 314ns | 323ns | 328ns | -58.38% |
| warm-clamp-minimum | 313ns | 313ns | 309ns | 313ns | 318ns | -59.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 712ns | 711ns | 714ns | base | 11.499 |
| warm-clamp-accfit | 382ns | 377ns | 393ns | -46.36% | 21.438 |
| warm-clamp-accfit-dyn | 4786ns | 4658ns | 4976ns | +571.77% | 1.712 |
| warm-clamp-head | 561ns | 546ns | 579ns | -21.23% | 14.599 |
| warm-clamp-min-lanes | 261ns | 255ns | 265ns | -63.37% | 31.390 |
| warm-clamp-minimum | 254ns | 252ns | 257ns | -64.33% | 32.239 |

## Performance model

- Peak throughput: **32.550 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 11.504 | 35.3% |
| warm-clamp-accfit | 21.603 | 66.4% |
| warm-clamp-accfit-dyn | 1.714 | 5.3% |
| warm-clamp-head | 14.515 | 44.6% |
| warm-clamp-min-lanes | 31.303 | 96.2% |
| warm-clamp-minimum | 32.258 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 774ns | 774ns | base |
| warm-clamp-accfit | 442ns | 442ns | -42.97% |
| warm-clamp-accfit-dyn | 4855ns | 4855ns | +526.94% |
| warm-clamp-head | 625ns | 625ns | -19.26% |
| warm-clamp-min-lanes | 322ns | 322ns | -58.38% |
| warm-clamp-minimum | 313ns | 313ns | -59.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 712ns | base | --- | [712, 713] | --- | --- | --- | --- |
| warm-clamp-accfit | 379ns | -332.7ns (-46.7%) | [-335, -331]ns | [378, 380] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4781ns | +4069.4ns (+571.5%) | [+4002, +4108]ns | [4712, 4821] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 564ns | -149.4ns (-21.0%) | [-163, -146]ns | [549, 566] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 262ns | -450.4ns (-63.2%) | [-453, -450]ns | [261, 263] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 254ns | -458.6ns (-64.4%) | [-460, -457]ns | [253, 255] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 714ns | -47.0% | +552.2% | -20.8% | -63.0% | -64.5% |
| 2 | 712ns | -46.7% | +565.4% | -20.3% | -63.5% | -64.5% |
| 3 | 715ns | -47.2% | +551.7% | -21.1% | -63.4% | -64.4% |
| 4 | 712ns | -47.1% | +554.5% | -19.7% | -63.1% | -64.3% |
| 5 | 715ns | -47.3% | +551.6% | -21.0% | -63.3% | -64.6% |
| 6 | 711ns | -46.6% | +558.7% | -20.4% | -63.3% | -64.2% |
| 7 | 714ns | -46.9% | +552.3% | -20.6% | -63.5% | -64.6% |
| 8 | 716ns | -47.1% | +550.6% | -20.2% | -63.6% | -64.4% |
| 9 | 712ns | -47.1% | +553.7% | -21.0% | -63.6% | -64.4% |
| 10 | 712ns | -47.1% | +571.3% | -20.3% | -63.1% | -64.6% |
| 11 | 711ns | -46.3% | +565.0% | -23.4% | -62.3% | -63.9% |
| 12 | 710ns | -46.4% | +616.9% | -23.1% | -62.6% | -64.1% |
| 13 | 712ns | -46.7% | +563.2% | -23.1% | -62.8% | -64.4% |
| 14 | 711ns | -46.7% | +557.0% | -23.1% | -63.2% | -64.2% |
| 15 | 711ns | -46.7% | +558.4% | -15.6% | -63.2% | -64.1% |
| 16 | 709ns | -45.6% | +564.0% | -20.0% | -62.7% | -63.6% |
| 17 | 712ns | -46.7% | +583.6% | -20.5% | -63.1% | -64.3% |
| 18 | 711ns | -46.5% | +572.6% | -20.1% | -63.0% | -63.8% |
| 19 | 711ns | -46.5% | +563.3% | -23.0% | -62.7% | -64.2% |
| 20 | 712ns | -46.9% | +554.7% | -22.8% | -63.1% | -64.2% |
| 21 | 712ns | -47.0% | +556.3% | -20.5% | -63.5% | -64.5% |
| 22 | 714ns | -47.3% | +554.5% | -20.9% | -62.8% | -64.9% |
| 23 | 713ns | -46.6% | +576.1% | -20.6% | -63.3% | -64.6% |
| 24 | 713ns | -47.1% | +575.7% | -20.5% | -63.5% | -64.5% |
| 25 | 714ns | -47.2% | +574.5% | -20.5% | -63.2% | -64.7% |
| 26 | 712ns | -47.1% | +575.5% | -20.5% | -63.2% | -64.6% |
| 27 | 713ns | -47.0% | +586.2% | -21.4% | -63.1% | -64.7% |
| 28 | 714ns | -46.9% | +601.0% | -21.0% | -63.1% | -64.8% |
| 29 | 713ns | -46.9% | +576.6% | -20.4% | -63.2% | -64.3% |
| 30 | 712ns | -47.1% | +569.2% | -20.9% | -63.2% | -64.5% |
| 31 | 711ns | -44.7% | +577.6% | -23.1% | -63.2% | -64.2% |
| 32 | 713ns | -44.9% | +576.5% | -13.0% | -63.9% | -63.9% |
| 33 | 714ns | -45.2% | +575.8% | -23.2% | -63.7% | -64.0% |
| 34 | 714ns | -45.0% | +590.2% | -23.5% | -64.3% | -63.9% |
| 35 | 712ns | -44.9% | +578.3% | -23.2% | -64.1% | -64.0% |
| 36 | 712ns | -44.9% | +604.1% | -23.1% | -64.2% | -63.7% |
| 37 | 711ns | -44.5% | +578.6% | -23.1% | -64.0% | -64.4% |
| 38 | 712ns | -44.9% | +577.2% | -23.5% | -64.2% | -64.4% |
| 39 | 712ns | -44.5% | +590.5% | -23.4% | -64.4% | -64.5% |
| 40 | 712ns | -45.3% | +616.1% | -23.3% | -64.4% | -64.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.238 | moderate+ |
| warm-clamp-accfit | 0.813 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.422 | moderate+ |
| warm-clamp-head | 0.065 | ok |
| warm-clamp-min-lanes | 0.715 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.458 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.0ns | 712.4ns | 0.4% |  |
| warm-clamp-accfit | 2.4ns | 382.1ns | 0.6% |  |
| warm-clamp-accfit-dyn | 3.0ns | 4785.6ns | 0.1% |  |
| warm-clamp-head | 2.7ns | 561.1ns | 0.5% |  |
| warm-clamp-min-lanes | 2.9ns | 261.0ns | 1.1% |  |
| warm-clamp-minimum | 2.8ns | 254.1ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 710.6-714.4 ns)
    710.6 |
    710.8 |###########
    711.0 |
    711.2 |##################################
    711.4 |
    711.6 |##################################
    711.7 |
    711.9 |########################################
    712.1 |
    712.3 |###########
    712.5 |
    712.7 |
    712.9 |#################
    713.1 |
    713.3 |###########
    713.5 |
    713.7 |######################
    713.9 |
    714.0 |#################
    714.2 |
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 376.7-393.1 ns)
    376.7 |##################################
    377.5 |########################################
    378.3 |######################
    379.1 |############################
    380.0 |######################
    380.8 |
    381.6 |#####
    382.4 |
    383.3 |
    384.1 |
    384.9 |#####
    385.7 |
    386.5 |
    387.4 |
    388.2 |
    389.0 |#####
    389.8 |
    390.7 |#####
    391.5 |###########
    392.3 |#################
  (2 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 4657.8-4976.3 ns)
   4657.8 |##########################
   4673.8 |#############
   4689.7 |
   4705.6 |########
   4721.5 |#############
   4737.5 |
   4753.4 |####
   4769.3 |########
   4785.2 |
   4801.2 |########
   4817.1 |########################################
   4833.0 |
   4848.9 |
   4864.9 |####
   4880.8 |####
   4896.7 |
   4912.6 |####
   4928.6 |####
   4944.5 |
   4960.4 |
  (4 below, 4 above range)

warm-clamp-head (n=40, range 545.9-579.3 ns)
    545.9 |################################
    547.6 |########
    549.3 |####
    550.9 |
    552.6 |
    554.3 |
    555.9 |
    557.6 |
    559.3 |####
    560.9 |
    562.6 |################
    564.3 |####################
    565.9 |########################################
    567.6 |####
    569.3 |
    570.9 |########
    572.6 |
    574.3 |
    575.9 |
    577.6 |
  (4 below, 2 above range)

warm-clamp-min-lanes (n=40, range 255.0-265.1 ns)
    255.0 |################
    255.5 |########
    256.0 |
    256.5 |
    257.0 |########
    257.5 |
    258.0 |
    258.5 |
    259.0 |########
    259.5 |################################
    260.1 |########
    260.6 |################
    261.1 |########
    261.6 |########################################
    262.1 |################################
    262.6 |################################
    263.1 |################
    263.6 |################
    264.1 |################
    264.6 |
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 251.7-257.4 ns)
    251.7 |####################
    252.0 |#############
    252.2 |####################
    252.5 |
    252.8 |#################################
    253.1 |######
    253.4 |
    253.7 |##########################
    253.9 |##########################
    254.2 |
    254.5 |########################################
    254.8 |#############
    255.1 |
    255.4 |
    255.6 |
    255.9 |
    256.2 |
    256.5 |#############
    256.8 |
    257.1 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.71 (measurement drift or warm-up artifact)
