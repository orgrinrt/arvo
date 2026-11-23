# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-contend-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-contend-d16 has the worst median (62.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-contend-packed at 44.80 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-contend-packed beats baseline by 27% (significant)

bitpack-contend-packed is -17.11 us (27%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-packed is fastest but the noisiest (CV 9.2%)

bitpack-contend-packed wins on median (44.80 us) yet has the highest variance (CV 9.2%), while bitpack-contend-packed-simd is the steadiest (CV 3.8%, 47.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (bitpack-contend-packed, bitpack-contend-d32) are a dead heat (<1%)

bitpack-contend-packed (44.80 us) and bitpack-contend-d32 (45.11 us) differ by 0.68%, inside the noise, even though the wider field spreads 39.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.55)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-packed vs stability leader bitpack-contend-packed-simd (+6% speed for 2.4x steadier)

bitpack-contend-packed is fastest (44.80 us, CV 9.2%); bitpack-contend-packed-simd gives up 6.5% median for 2.4x lower variance (CV 3.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-packed** at 44799.7 ns median (-28.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.40x (fastest 44799.7 ns, slowest 62553.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 60655ns | 62633ns | 51652ns | 62108ns | 65297ns | base |
| bitpack-contend-d16-control | 59974ns | 62638ns | 51804ns | 61131ns | 64670ns | -1.12% |
| bitpack-contend-d32 | 44492ns | 45189ns | 38107ns | 45071ns | 49143ns | -26.65% |
| bitpack-contend-d64 | 51670ns | 50833ns | 49534ns | 50891ns | 56145ns | -14.81% |
| bitpack-contend-packed | 45445ns | 44880ns | 41895ns | 45043ns | 50202ns | -25.08% |
| bitpack-contend-packed-simd | 47717ns | 47813ns | 44874ns | 48037ns | 49602ns | -21.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 60569ns | 51567ns | 65215ns | base | 17.312 |
| bitpack-contend-d16-control | 59884ns | 51716ns | 64587ns | -1.13% | 17.510 |
| bitpack-contend-d32 | 44407ns | 38022ns | 49053ns | -26.68% | 23.613 |
| bitpack-contend-d64 | 51431ns | 49295ns | 55884ns | -15.09% | 20.388 |
| bitpack-contend-packed | 45360ns | 41808ns | 50118ns | -25.11% | 23.117 |
| bitpack-contend-packed-simd | 47629ns | 44794ns | 49513ns | -21.36% | 22.016 |

## Performance model

- Peak throughput: **27.578 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 16.763 | 60.8% |
| bitpack-contend-d16-control | 16.763 | 60.8% |
| bitpack-contend-d32 | 23.247 | 84.3% |
| bitpack-contend-d64 | 20.731 | 75.2% |
| bitpack-contend-packed | 23.406 | 84.9% |
| bitpack-contend-packed-simd | 21.979 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 60655ns | 60655ns | base |
| bitpack-contend-d16-control | 59974ns | 59974ns | -1.12% |
| bitpack-contend-d32 | 44492ns | 44492ns | -26.65% |
| bitpack-contend-d64 | 51670ns | 51670ns | -14.81% |
| bitpack-contend-packed | 45445ns | 45445ns | -25.08% |
| bitpack-contend-packed-simd | 47717ns | 47717ns | -21.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 62554ns | base | --- | [61698, 62589] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 62553ns | no significant difference | [-2572, +2450]ns | [62544, 62565] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d32 | 45106ns | -16627.8ns (-26.6%) | [-18253, -16090]ns | [45097, 45125] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 50581ns | -12077.6ns (-19.3%) | [-12335, -10502]ns | [50472, 50863] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 44800ns | -17110.1ns (-27.4%) | [-19711, -14162]ns | [44757, 44873] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 47707ns | -14208.9ns (-22.7%) | [-16955, -13066]ns | [47498, 49233] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 61701ns | +2.2% | -26.9% | -18.9% | -23.0% | -21.5% |
| 1 | 2 | 62562ns | -17.7% | -39.3% | -19.4% | -28.5% | -28.4% |
| 1 | 3 | 51172ns | +22.2% | -11.8% | -0.8% | -12.5% | -3.9% |
| 1 | 4 | 65118ns | -3.9% | -30.7% | -20.9% | -31.3% | -25.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.551 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.442 | moderate- |
| bitpack-contend-d32 | -0.413 | moderate- |
| bitpack-contend-d64 | 0.189 | ok |
| bitpack-contend-packed | -0.089 | ok |
| bitpack-contend-packed-simd | -0.427 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 4/4, lost 0/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 23.6ns | 60568.6ns | 0.0% |  |
| bitpack-contend-d16-control | 24.3ns | 59884.1ns | 0.0% |  |
| bitpack-contend-d32 | 22.7ns | 44407.3ns | 0.1% |  |
| bitpack-contend-d64 | 52.3ns | 51431.1ns | 0.1% |  |
| bitpack-contend-packed | 22.9ns | 45359.6ns | 0.1% |  |
| bitpack-contend-packed-simd | 23.6ns | 47628.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 51566.6-65214.6 ns)
  51566.6 |
  52249.0 |###
  52931.4 |###
  53613.8 |###
  54296.2 |
  54978.6 |
  55661.0 |
  56343.4 |
  57025.8 |
  57708.2 |###
  58390.6 |
  59073.0 |
  59755.4 |
  60437.8 |
  61120.2 |#########################
  61802.6 |###
  62485.0 |########################################
  63167.4 |
  63849.8 |###
  64532.2 |################################
  (6 below, 1 above range)

bitpack-contend-d16-control (n=40, range 51716.5-64587.4 ns)
  51716.5 |####
  52360.0 |
  53003.5 |
  53647.1 |##
  54290.6 |####
  54934.2 |####
  55577.7 |##
  56221.3 |
  56864.8 |
  57508.4 |
  58151.9 |
  58795.4 |
  59439.0 |
  60082.5 |
  60726.1 |
  61369.6 |
  62013.2 |########################################
  62656.7 |##
  63300.3 |##
  63943.8 |
  (5 below, 5 above range)

bitpack-contend-d32 (n=40, range 38021.8-49052.6 ns)
  38021.8 |#
  38573.4 |
  39124.9 |
  39676.4 |
  40228.0 |
  40779.5 |
  41331.1 |
  41882.6 |#
  42434.1 |
  42985.7 |
  43537.2 |
  44088.7 |
  44640.3 |########################################
  45191.8 |
  45743.4 |
  46294.9 |#
  46846.4 |#
  47398.0 |#
  47949.5 |
  48501.0 |
  (7 below, 5 above range)

bitpack-contend-d64 (n=40, range 49294.5-55884.1 ns)
  49294.5 |
  49624.0 |##
  49953.5 |
  50283.0 |########################################
  50612.5 |####################
  50941.9 |#######
  51271.4 |#####
  51600.9 |##
  51930.4 |
  52259.9 |##
  52589.3 |
  52918.8 |
  53248.3 |##
  53577.8 |
  53907.3 |
  54236.7 |
  54566.2 |
  54895.7 |
  55225.2 |
  55554.7 |
  (4 below, 3 above range)

bitpack-contend-packed (n=40, range 41807.7-50118.2 ns)
  41807.7 |
  42223.2 |
  42638.7 |
  43054.2 |
  43469.8 |
  43885.3 |######
  44300.8 |
  44716.3 |########################################
  45131.9 |
  45547.4 |
  45962.9 |####
  46378.5 |##
  46794.0 |##
  47209.5 |####
  47625.0 |########
  48040.6 |
  48456.1 |
  48871.6 |
  49287.2 |
  49702.7 |
  (6 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 44794.0-49513.3 ns)
  44794.0 |##############################
  45029.9 |
  45265.9 |
  45501.9 |#####
  45737.8 |
  45973.8 |##########
  46209.8 |
  46445.7 |
  46681.7 |
  46917.7 |
  47153.6 |
  47389.6 |####################
  47625.6 |##############################
  47861.5 |
  48097.5 |
  48333.5 |
  48569.4 |
  48805.4 |#####
  49041.4 |##############################
  49277.3 |########################################
  (3 below, 3 above range)

```
