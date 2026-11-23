# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 21% (significant)

bitpack-contend-packed is -17.19 us (21%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-packed, bitpack-contend-d32) are a dead heat (<1%)

bitpack-contend-packed (65.44 us) and bitpack-contend-d32 (65.80 us) differ by 0.56%, inside the noise, even though the wider field spreads 71.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16-control shows alternating (throttle bounce) (autocorr -0.52)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-packed vs stability leader bitpack-contend-packed-simd (+1% speed for 1.2x steadier)

bitpack-contend-packed is fastest (65.44 us, CV 10.7%); bitpack-contend-packed-simd gives up 0.7% median for 1.2x lower variance (CV 8.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-packed** at 65435.1 ns median (-20.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.72x (fastest 65435.1 ns, slowest 112266.1 ns)

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
| bitpack-contend-d16 | 88483ns | 82826ns | 81883ns | 85918ns | 102780ns | base |
| bitpack-contend-d16-control | 88330ns | 84450ns | 81566ns | 86236ns | 101375ns | -0.17% |
| bitpack-contend-d32 | 72115ns | 66020ns | 63899ns | 69706ns | 87557ns | -18.50% |
| bitpack-contend-d64 | 107555ns | 112614ns | 83406ns | 111488ns | 119905ns | +21.55% |
| bitpack-contend-packed | 69336ns | 65504ns | 64969ns | 66676ns | 81682ns | -21.64% |
| bitpack-contend-packed-simd | 69188ns | 65983ns | 65294ns | 67082ns | 79399ns | -21.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 88399ns | 81811ns | 102673ns | base | 11.862 |
| bitpack-contend-d16-control | 88253ns | 81494ns | 101291ns | -0.16% | 11.881 |
| bitpack-contend-d32 | 71856ns | 63722ns | 87228ns | -18.71% | 14.593 |
| bitpack-contend-d64 | 107244ns | 83167ns | 119592ns | +21.32% | 9.777 |
| bitpack-contend-packed | 69259ns | 64898ns | 81599ns | -21.65% | 15.140 |
| bitpack-contend-packed-simd | 69113ns | 65219ns | 79321ns | -21.82% | 15.172 |

## Performance model

- Peak throughput: **16.455 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 12.671 | 77.0% |
| bitpack-contend-d16-control | 12.431 | 75.5% |
| bitpack-contend-d32 | 15.935 | 96.8% |
| bitpack-contend-d64 | 9.340 | 56.8% |
| bitpack-contend-packed | 16.025 | 97.4% |
| bitpack-contend-packed-simd | 15.908 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 88483ns | 88483ns | base |
| bitpack-contend-d16-control | 88330ns | 88330ns | -0.17% |
| bitpack-contend-d32 | 72115ns | 72115ns | -18.50% |
| bitpack-contend-d64 | 107555ns | 107555ns | +21.55% |
| bitpack-contend-packed | 69336ns | 69336ns | -21.64% |
| bitpack-contend-packed-simd | 69188ns | 69188ns | -21.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 82752ns | base | --- | [82241, 85786] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 84354ns | no significant difference | [-2901, +1618]ns | [82557, 87204] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d32 | 65803ns | -17044.7ns (-20.6%) | [-21242, -15341]ns | [65547, 67254] | YES | 0.0080 | 0.0064 | 0 |
| bitpack-contend-d64 | 112266ns | +29301.6ns (+35.4%) | [+24936, +35442]ns | [108845, 117576] | YES | 0.0037 | 0.0022 | 0 |
| bitpack-contend-packed | 65435ns | -17186.2ns (-20.8%) | [-17916, -16963]ns | [65118, 65761] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 65914ns | -16550.6ns (-20.0%) | [-17863, -16110]ns | [65668, 66369] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 82862ns | -0.7% | -21.0% | +33.1% | -2.4% | -4.5% |
| 1 | 2 | 82103ns | +19.3% | -20.5% | +43.9% | -20.7% | -19.8% |
| 1 | 3 | 82332ns | -0.2% | +5.3% | +43.9% | -21.1% | -20.7% |
| 1 | 4 | 99330ns | -15.1% | -33.8% | -16.3% | -33.9% | -33.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.083 | ok |
| bitpack-contend-d16-control | -0.523 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.411 | moderate- |
| bitpack-contend-d64 | -0.146 | ok |
| bitpack-contend-packed | -0.089 | ok |
| bitpack-contend-packed-simd | -0.060 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 3/4, lost 1/4
- **bitpack-contend-d32**: won 3/4, lost 1/4
- **bitpack-contend-d64**: won 1/4, lost 3/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 22.9ns | 88399.1ns | 0.0% |  |
| bitpack-contend-d16-control | 20.2ns | 88253.3ns | 0.0% |  |
| bitpack-contend-d32 | 29.2ns | 71856.1ns | 0.0% |  |
| bitpack-contend-d64 | 41.0ns | 107244.1ns | 0.0% |  |
| bitpack-contend-packed | 20.0ns | 69259.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 20.1ns | 69112.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 81810.8-102672.6 ns)
  81810.8 |########################################
  82853.8 |########
  83896.9 |
  84940.0 |
  85983.1 |
  87026.2 |##
  88069.3 |
  89112.4 |##
  90155.5 |
  91198.6 |
  92241.7 |
  93284.8 |
  94327.9 |
  95371.0 |
  96414.1 |
  97457.2 |
  98500.3 |#############
  99543.4 |######
  100586.4 |##
  101629.5 |
  (4 below, 2 above range)

bitpack-contend-d16-control (n=40, range 81494.2-101291.4 ns)
  81494.2 |####################################
  82484.1 |####################
  83473.9 |############
  84463.8 |############
  85453.7 |########
  86443.5 |
  87433.4 |####
  88423.2 |
  89413.1 |####
  90403.0 |
  91392.8 |####
  92382.7 |
  93372.5 |
  94362.4 |
  95352.3 |
  96342.1 |
  97332.0 |########################################
  98321.9 |
  99311.7 |
  100301.6 |
  (4 below, 1 above range)

bitpack-contend-d32 (n=40, range 63722.2-87228.2 ns)
  63722.2 |#
  64897.6 |########################################
  66072.9 |#
  67248.1 |#
  68423.4 |
  69598.8 |
  70774.1 |
  71949.4 |
  73124.6 |
  74299.9 |#
  75475.2 |
  76650.6 |
  77825.9 |
  79001.1 |
  80176.4 |
  81351.8 |
  82527.1 |#
  83702.4 |#
  84877.6 |#
  86052.9 |###############
  (3 below, 1 above range)

bitpack-contend-d64 (n=40, range 83167.0-119591.6 ns)
  83167.0 |#####################
  84988.2 |
  86809.4 |
  88630.7 |
  90451.9 |
  92273.1 |
  94094.4 |
  95915.6 |
  97736.8 |
  99558.1 |
  101379.3 |
  103200.5 |
  105021.8 |
  106843.0 |##############
  108664.2 |###
  110485.5 |##################
  112306.7 |#######
  114127.9 |###
  115949.2 |##############
  117770.4 |########################################
  (4 below, 2 above range)

bitpack-contend-packed (n=40, range 64898.5-81599.2 ns)
  64898.5 |########################################
  65733.5 |######
  66568.6 |
  67403.6 |
  68238.6 |
  69073.7 |
  69908.7 |
  70743.7 |
  71578.8 |
  72413.8 |
  73248.9 |
  74083.9 |
  74918.9 |
  75754.0 |
  76589.0 |
  77424.0 |
  78259.1 |
  79094.1 |###
  79929.1 |###
  80764.2 |######
  (3 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 65218.6-79321.5 ns)
  65218.6 |########################################
  65923.8 |############
  66628.9 |####
  67334.0 |
  68039.2 |
  68744.3 |
  69449.5 |
  70154.6 |
  70859.8 |
  71564.9 |
  72270.0 |
  72975.2 |
  73680.3 |
  74385.5 |
  75090.6 |
  75795.8 |
  76500.9 |
  77206.0 |
  77911.2 |
  78616.3 |##############
  (3 below, 3 above range)

```
