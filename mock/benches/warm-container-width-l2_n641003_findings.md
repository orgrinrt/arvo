# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 60% (significant)

warm-container-native is -470.33 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (789.37 us) is 2.5x the fastest (321.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {warm-container-native, warm-container-minimum} vs {warm-container-headroom, warm-container-plusone} (139% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-plusone} with a 139% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-native** at 321627.7 ns median (-59.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.45x (fastest 321627.7 ns, slowest 789373.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 791392ns | 789372ns | 779476ns | 790356ns | 806416ns | base |
| warm-container-minimum | 332417ns | 330194ns | 322126ns | 330814ns | 347518ns | -58.00% |
| warm-container-native | 327186ns | 322121ns | 316193ns | 323847ns | 348196ns | -58.66% |
| warm-container-plusone | 798602ns | 790580ns | 781793ns | 790755ns | 838951ns | +0.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 790130ns | 778198ns | 805224ns | base | 5.308 |
| warm-container-minimum | 332006ns | 321606ns | 347144ns | -57.98% | 12.633 |
| warm-container-native | 326664ns | 315716ns | 347651ns | -58.66% | 12.840 |
| warm-container-plusone | 797377ns | 780719ns | 837672ns | +0.92% | 5.260 |

## Performance model

- Peak throughput: **13.285 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 5.322 | 40.1% |
| warm-container-minimum | 12.717 | 95.7% |
| warm-container-native | 13.041 | 98.2% |
| warm-container-plusone | 5.313 | 40.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 791392ns | 791392ns | base |
| warm-container-minimum | 332417ns | 332417ns | -58.00% |
| warm-container-native | 327186ns | 327186ns | -58.66% |
| warm-container-plusone | 798602ns | 798602ns | +0.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 788104ns | base | --- | [785883, 792536] | --- | --- | --- | --- |
| warm-container-minimum | 329819ns | -457623.5ns (-58.1%) | [-461571, -453489]ns | [326358, 334682] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 321628ns | -467184.3ns (-59.3%) | [-468732, -460226]ns | [320101, 325917] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 789374ns | no significant difference | [-4465, +6814]ns | [786171, 792289] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|
| 1 | 788103ns | -59.1% | -52.6% | +2.5% |
| 2 | 795935ns | -59.8% | -57.9% | -1.6% |
| 3 | 784085ns | -58.7% | -56.7% | +0.8% |
| 4 | 799739ns | -60.0% | -58.9% | -2.2% |
| 5 | 790810ns | -57.4% | -59.1% | -1.1% |
| 6 | 792918ns | -55.9% | -58.8% | -0.6% |
| 7 | 780323ns | -57.4% | -58.8% | +1.0% |
| 8 | 777529ns | -56.0% | -58.9% | +1.0% |
| 9 | 785885ns | -57.5% | -58.5% | -0.3% |
| 10 | 774341ns | -58.4% | -58.0% | +3.9% |
| 11 | 809328ns | -58.8% | -59.0% | -0.6% |
| 12 | 793103ns | -58.8% | -52.5% | -0.8% |
| 13 | 784440ns | -57.9% | -56.6% | -0.3% |
| 14 | 782080ns | -59.0% | -56.2% | +1.6% |
| 15 | 805981ns | -58.0% | -58.2% | -1.4% |
| 16 | 797509ns | -58.6% | -58.3% | -1.1% |
| 17 | 788104ns | -57.5% | -58.3% | +1.2% |
| 18 | 807432ns | -59.6% | -59.8% | +7.5% |
| 19 | 812351ns | -59.8% | -59.9% | -2.7% |
| 20 | 811769ns | -60.1% | -58.5% | +11.5% |
| 21 | 781050ns | -58.5% | -58.8% | +0.4% |
| 22 | 788139ns | -58.8% | -59.5% | +0.8% |
| 23 | 795589ns | -59.1% | -60.4% | +1.1% |
| 24 | 777370ns | -58.5% | -58.6% | +8.4% |
| 25 | 792154ns | -58.4% | -59.1% | -0.1% |
| 26 | 787563ns | -56.4% | -60.0% | -1.1% |
| 27 | 784535ns | -56.9% | -59.8% | +0.4% |
| 28 | 795813ns | -58.0% | -60.4% | -1.3% |
| 29 | 781628ns | -58.3% | -59.7% | -0.5% |
| 30 | 797685ns | -57.9% | -60.1% | -0.6% |
| 31 | 776078ns | -58.5% | -58.9% | +2.0% |
| 32 | 787536ns | -58.2% | -59.3% | -1.3% |
| 33 | 789372ns | -57.4% | -59.4% | +0.2% |
| 34 | 777263ns | -56.9% | -58.8% | +0.9% |
| 35 | 785880ns | -58.3% | -59.4% | +0.1% |
| 36 | 785413ns | -56.6% | -59.9% | +1.3% |
| 37 | 788450ns | -55.6% | -59.4% | +0.3% |
| 38 | 786976ns | -54.2% | -59.4% | -0.5% |
| 39 | 797195ns | -55.8% | -60.3% | +8.5% |
| 40 | 787731ns | -58.3% | -59.3% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.075 | ok |
| warm-container-minimum | 0.480 | moderate+ |
| warm-container-native | 0.440 | moderate+ |
| warm-container-plusone | -0.084 | ok |

**Consistency summary:**

- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 18/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 16.4ns | 790129.6ns | 0.0% |  |
| warm-container-minimum | 11.0ns | 332006.0ns | 0.0% |  |
| warm-container-native | 9.5ns | 326663.5ns | 0.0% |  |
| warm-container-plusone | 20.4ns | 797376.7ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 778197.7-805224.2 ns)
  778197.7 |
  779549.0 |########
  780900.4 |########################
  782251.7 |
  783603.0 |########################
  784954.3 |########################
  786305.7 |########################
  787657.0 |########################################
  789008.3 |########
  790359.6 |########
  791711.0 |################
  793062.3 |########
  794413.6 |########
  795764.9 |################
  797116.3 |########################
  798467.6 |########
  799818.9 |
  801170.2 |
  802521.6 |
  803872.9 |
  (5 below, 5 above range)

warm-container-minimum (n=40, range 321606.2-347144.2 ns)
  321606.2 |########################################
  322883.1 |##############################
  324160.0 |####################
  325436.9 |##############################
  326713.8 |####################
  327990.7 |####################
  329267.6 |##############################
  330544.5 |
  331821.4 |##########
  333098.3 |####################
  334375.2 |##############################
  335652.1 |##############################
  336929.0 |
  338205.9 |####################
  339482.8 |##########
  340759.7 |
  342036.6 |####################
  343313.5 |
  344590.4 |
  345867.3 |
  (3 below, 4 above range)

warm-container-native (n=40, range 315716.0-347651.2 ns)
  315716.0 |#####
  317312.8 |#####
  318909.5 |########################################
  320506.3 |#########################
  322103.0 |#####
  323699.8 |###############
  325296.6 |##########
  326893.3 |#####
  328490.1 |##########
  330086.8 |#####
  331683.6 |#####
  333280.3 |
  334877.1 |#####
  336473.9 |##########
  338070.6 |#####
  339667.4 |#####
  341264.1 |#####
  342860.9 |
  344457.6 |
  346054.4 |
  (6 below, 2 above range)

warm-container-plusone (n=40, range 780718.8-837672.5 ns)
  780718.8 |########################################
  783566.5 |######################
  786414.2 |##################################
  789261.9 |##################################
  792109.5 |######################
  794957.2 |#####
  797804.9 |#####
  800652.6 |
  803500.3 |#################
  806348.0 |#####
  809195.6 |
  812043.3 |
  814891.0 |
  817738.7 |
  820586.4 |
  823434.1 |
  826281.8 |
  829129.4 |
  831977.1 |
  834824.8 |
  (3 below, 4 above range)

```
