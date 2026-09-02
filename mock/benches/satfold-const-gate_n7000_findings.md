# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-gate-true beats baseline by 85% (significant)

satfold-gate-true is -24.29 us (85%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 7.1x slower than the field

satfold-seq (28.45 us) is 7.1x the fastest (4.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-seq shows warm-up / thermal drift (autocorr +0.78)

satfold-seq's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16, satfold-lanes16-3, satfold-gate-true} vs {satfold-gate-false, satfold-seq} (599% apart)

The field splits into a fast tier {satfold-lanes16, satfold-lanes16-3, satfold-gate-true} and a slow tier {satfold-gate-false, satfold-seq} with a 599% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.1x the fastest

Fastest satfold-lanes16 (4.02 us) to slowest satfold-seq (28.45 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-lanes16** at 4023.1 ns median (-85.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.07x (fastest 4023.1 ns, slowest 28450.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 28813ns | 28506ns | 28269ns | 28649ns | 29849ns | base |
| satfold-gate-true | 4158ns | 4133ns | 4066ns | 4139ns | 4305ns | -85.57% |
| satfold-lanes16 | 4138ns | 4087ns | 4067ns | 4109ns | 4298ns | -85.64% |
| satfold-lanes16-3 | 4160ns | 4131ns | 4065ns | 4136ns | 4327ns | -85.56% |
| satfold-seq | 28855ns | 28538ns | 28301ns | 28672ns | 29961ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 28720ns | 28195ns | 29750ns | base | 1.141 |
| satfold-gate-true | 4091ns | 4005ns | 4235ns | -85.76% | 8.011 |
| satfold-lanes16 | 4076ns | 4006ns | 4230ns | -85.81% | 8.040 |
| satfold-lanes16-3 | 4097ns | 4003ns | 4261ns | -85.73% | 7.998 |
| satfold-seq | 28768ns | 28230ns | 29865ns | +0.17% | 1.139 |

## Performance model

- Peak throughput: **8.185 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.153 | 14.1% |
| satfold-gate-true | 8.054 | 98.4% |
| satfold-lanes16 | 8.145 | 99.5% |
| satfold-lanes16-3 | 8.056 | 98.4% |
| satfold-seq | 1.152 | 14.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 28813ns | 28813ns | base |
| satfold-gate-true | 4158ns | 4158ns | -85.57% |
| satfold-lanes16 | 4138ns | 4138ns | -85.64% |
| satfold-lanes16-3 | 4160ns | 4160ns | -85.56% |
| satfold-seq | 28855ns | 28855ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 28419ns | base | --- | [28254, 28880] | --- | --- | --- | --- |
| satfold-gate-true | 4069ns | -24400.4ns (-85.9%) | [-24817, -24230]ns | [4014, 4098] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 4023ns | -24406.5ns (-85.9%) | [-24762, -24232]ns | [4015, 4065] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 4068ns | -24315.0ns (-85.6%) | [-24844, -24221]ns | [4011, 4139] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 28450ns | +62.5ns (+0.2%) | [+26, +112]ns | [28300, 28776] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|
| 1 | 28295ns | -85.8% | -85.8% | -85.8% | +0.0% |
| 2 | 28234ns | -85.8% | -85.8% | -85.8% | +0.4% |
| 3 | 28191ns | -85.8% | -85.7% | -85.3% | +0.3% |
| 4 | 28223ns | -85.8% | -85.8% | -85.8% | +1.9% |
| 5 | 28266ns | -85.8% | -85.8% | -85.8% | +0.1% |
| 6 | 28222ns | -85.8% | -85.8% | -85.8% | +0.2% |
| 7 | 28195ns | -85.5% | -85.8% | -85.8% | -0.1% |
| 8 | 28193ns | -85.8% | -85.8% | -85.8% | +0.2% |
| 9 | 28582ns | -86.0% | -86.0% | -86.0% | -1.1% |
| 10 | 28269ns | -85.8% | -85.8% | -85.8% | +1.7% |
| 11 | 29584ns | -85.7% | -85.8% | -85.7% | +0.9% |
| 12 | 30871ns | -86.2% | -86.3% | -86.4% | -3.1% |
| 13 | 29635ns | -86.0% | -86.3% | -85.7% | +0.3% |
| 14 | 29468ns | -85.6% | -85.8% | -85.6% | +0.8% |
| 15 | 29488ns | -85.6% | -85.9% | -85.6% | +0.2% |
| 16 | 29269ns | -85.6% | -86.0% | -85.8% | -0.2% |
| 17 | 29250ns | -85.5% | -85.8% | -85.5% | +1.0% |
| 18 | 28985ns | -85.8% | -85.5% | -85.6% | +3.6% |
| 19 | 29121ns | -85.5% | -85.6% | -85.5% | +4.3% |
| 20 | 29638ns | -85.7% | -85.9% | -85.7% | +0.5% |
| 21 | 29857ns | -86.3% | -86.5% | -86.6% | -4.6% |
| 22 | 29462ns | -85.9% | -84.8% | -86.4% | -4.0% |
| 23 | 28900ns | -85.9% | -85.7% | -86.2% | -2.3% |
| 24 | 28889ns | -85.9% | -86.1% | -86.1% | -1.6% |
| 25 | 28718ns | -85.8% | -85.8% | -86.0% | +0.9% |
| 26 | 28497ns | -85.7% | -85.7% | -85.9% | +0.3% |
| 27 | 28210ns | -85.5% | -85.8% | -85.3% | +0.2% |
| 28 | 28222ns | -85.6% | -85.7% | -85.8% | +0.1% |
| 29 | 28872ns | -85.9% | -86.1% | -86.1% | -0.6% |
| 30 | 28467ns | -85.7% | -85.9% | -85.5% | +1.3% |
| 31 | 28411ns | -85.9% | -85.9% | -85.7% | -0.6% |
| 32 | 28428ns | -85.9% | -85.9% | -84.6% | -0.4% |
| 33 | 28317ns | -85.5% | -85.5% | -85.6% | -0.4% |
| 34 | 28201ns | -85.1% | -85.7% | -85.5% | +0.1% |
| 35 | 28230ns | -85.7% | -85.8% | -85.6% | +0.2% |
| 36 | 28242ns | -85.8% | -85.8% | -84.7% | +1.9% |
| 37 | 28170ns | -85.8% | -85.1% | -85.5% | +1.6% |
| 38 | 28200ns | -85.3% | -85.8% | -85.5% | +2.8% |
| 39 | 28313ns | -85.9% | -85.9% | -85.8% | +0.1% |
| 40 | 28198ns | -85.8% | -85.8% | -85.7% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.767 | HIGH+ (drift/warm-up) |
| satfold-gate-true | 0.615 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.270 | moderate+ |
| satfold-lanes16-3 | 0.361 | moderate+ |
| satfold-seq | 0.778 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-gate-true**: won 40/40, lost 0/40
- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-3**: won 40/40, lost 0/40
- **satfold-seq**: won 11/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 1.7ns | 28719.6ns | 0.0% |  |
| satfold-gate-true | 1.2ns | 4090.6ns | 0.0% |  |
| satfold-lanes16 | 2.6ns | 4075.5ns | 0.1% |  |
| satfold-lanes16-3 | 1.1ns | 4096.9ns | 0.0% |  |
| satfold-seq | 3.0ns | 28768.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 28194.8-29750.3 ns)
  28194.8 |########################################
  28272.5 |##########
  28350.3 |######
  28428.1 |######
  28505.9 |###
  28583.6 |
  28661.4 |###
  28739.2 |
  28817.0 |######
  28894.7 |###
  28972.5 |###
  29050.3 |###
  29128.1 |
  29205.8 |######
  29283.6 |
  29361.4 |
  29439.2 |##########
  29516.9 |###
  29594.7 |######
  29672.5 |
  (4 below, 2 above range)

satfold-gate-true (n=40, range 4005.4-4235.2 ns)
   4005.4 |########################################
   4016.9 |
   4028.4 |
   4039.9 |####
   4051.4 |####
   4062.9 |##########################
   4074.3 |####
   4085.8 |########
   4097.3 |####
   4108.8 |
   4120.3 |####
   4131.8 |########
   4143.3 |####
   4154.8 |
   4166.2 |
   4177.7 |
   4189.2 |####
   4200.7 |
   4212.2 |####
   4223.7 |#############
  (6 below, 4 above range)

satfold-lanes16 (n=40, range 4006.3-4230.4 ns)
   4006.3 |########################################
   4017.5 |############
   4028.7 |######
   4039.9 |
   4051.1 |###
   4062.3 |######
   4073.5 |
   4084.7 |###
   4095.9 |###
   4107.1 |
   4118.4 |
   4129.6 |
   4140.8 |#########
   4152.0 |
   4163.2 |###
   4174.4 |######
   4185.6 |###
   4196.8 |
   4208.0 |######
   4219.2 |###
  (5 below, 1 above range)

satfold-lanes16-3 (n=40, range 4003.3-4261.1 ns)
   4003.3 |########################################
   4016.2 |#####
   4029.1 |
   4042.0 |
   4054.9 |##
   4067.8 |###########
   4080.7 |
   4093.5 |##
   4106.4 |
   4119.3 |
   4132.2 |##
   4145.1 |#####
   4158.0 |#####
   4170.9 |
   4183.7 |##
   4196.6 |
   4209.5 |
   4222.4 |#################
   4235.3 |
   4248.2 |##
  (3 below, 2 above range)

satfold-seq (n=40, range 28229.5-29864.6 ns)
  28229.5 |########################################
  28311.3 |#####
  28393.0 |##
  28474.8 |##
  28556.5 |#####
  28638.3 |##
  28720.0 |########
  28801.8 |##
  28883.5 |
  28965.3 |#####
  29047.1 |
  29128.8 |
  29210.6 |##
  29292.3 |
  29374.1 |
  29455.8 |
  29537.6 |#####
  29619.3 |
  29701.1 |########
  29782.8 |##
  (3 below, 3 above range)

```

## Diagnostics

- **satfold-gate-false**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **satfold-gate-true**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.78 (measurement drift or warm-up artifact)
