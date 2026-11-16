# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd beats baseline by 21% (significant)

bitpack-contend-packed-simd is -12.01 us (21%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-packed, bitpack-contend-packed-simd) are a dead heat (<1%)

bitpack-contend-packed (44.81 us) and bitpack-contend-packed-simd (44.84 us) differ by 0.05%, inside the noise, even though the wider field spreads 40.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16-control shows alternating (throttle bounce) (autocorr -0.73)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-d64 is inconsistent: worst-20% is 1.7x its best-20%

bitpack-contend-d64's best 20% of batches run at 50.73 us but its worst 20% at 87.64 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-packed** at 44814.2 ns median (-20.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 44814.2 ns, slowest 62990.8 ns)

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
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 57678ns | 56214ns | 55484ns | 57118ns | 61554ns | base |
| bitpack-contend-d16-control | 63588ns | 63092ns | 62505ns | 63351ns | 65383ns | +10.25% |
| bitpack-contend-d32 | 44054ns | 45228ns | 38144ns | 45240ns | 46404ns | -23.62% |
| bitpack-contend-d64 | 62822ns | 52369ns | 51001ns | 58389ns | 87942ns | +8.92% |
| bitpack-contend-packed | 44933ns | 44890ns | 44843ns | 44902ns | 45118ns | -22.10% |
| bitpack-contend-packed-simd | 44956ns | 44912ns | 43006ns | 44912ns | 47038ns | -22.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 57594ns | 55402ns | 61459ns | base | 18.206 |
| bitpack-contend-d16-control | 63504ns | 62422ns | 65304ns | +10.26% | 16.512 |
| bitpack-contend-d32 | 43964ns | 38051ns | 46307ns | -23.67% | 23.851 |
| bitpack-contend-d64 | 62552ns | 50729ns | 87638ns | +8.61% | 16.763 |
| bitpack-contend-packed | 44853ns | 44752ns | 45035ns | -22.12% | 23.378 |
| bitpack-contend-packed-simd | 44876ns | 42922ns | 46953ns | -22.08% | 23.366 |

## Performance model

- Peak throughput: **27.557 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 18.677 | 67.8% |
| bitpack-contend-d16-control | 16.646 | 60.4% |
| bitpack-contend-d32 | 23.221 | 84.3% |
| bitpack-contend-d64 | 20.137 | 73.1% |
| bitpack-contend-packed | 23.398 | 84.9% |
| bitpack-contend-packed-simd | 23.386 | 84.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 57678ns | 57678ns | base |
| bitpack-contend-d16-control | 63588ns | 63588ns | +10.25% |
| bitpack-contend-d32 | 44054ns | 44054ns | -23.62% |
| bitpack-contend-d64 | 62822ns | 62822ns | +8.92% |
| bitpack-contend-packed | 44933ns | 44933ns | -22.10% |
| bitpack-contend-packed-simd | 44956ns | 44956ns | -22.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 56143ns | base | --- | [56006, 57110] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 62991ns | +6426.7ns (+11.4%) | [+5443, +6753]ns | [62572, 64010] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 45156ns | -11338.4ns (-20.2%) | [-16045, -10433]ns | [45146, 45200] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 52072ns | no significant difference | [-4881, +6448]ns | [51034, 62194] | no | 0.2682 | 0.2682 | 0 |
| bitpack-contend-packed | 44814ns | -11288.5ns (-20.1%) | [-12307, -11136]ns | [44806, 44854] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 44838ns | -12010.5ns (-21.4%) | [-13842, -11250]ns | [44818, 44846] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 56056ns | +11.6% | -18.3% | -8.9% | -20.0% | -20.0% |
| 1 | 2 | 55595ns | +17.1% | -18.8% | -8.1% | -19.5% | -19.5% |
| 1 | 3 | 56684ns | +11.1% | -33.0% | +40.8% | -20.8% | -20.9% |
| 1 | 4 | 61445ns | +4.2% | -26.5% | -17.3% | -27.1% | -24.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.043 | ok |
| bitpack-contend-d16-control | -0.726 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.351 | moderate- |
| bitpack-contend-d64 | -0.421 | moderate- |
| bitpack-contend-packed | -0.674 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.069 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 0/4, lost 4/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 3/4, lost 1/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 23.2ns | 57593.9ns | 0.0% |  |
| bitpack-contend-d16-control | 22.8ns | 63503.7ns | 0.0% |  |
| bitpack-contend-d32 | 23.2ns | 43964.1ns | 0.1% |  |
| bitpack-contend-d64 | 34.3ns | 62552.3ns | 0.1% |  |
| bitpack-contend-packed | 21.3ns | 44853.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 22.3ns | 44876.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 55402.3-61458.8 ns)
  55402.3 |#########################
  55705.1 |#########################
  56007.9 |########################################
  56310.7 |#####
  56613.6 |###############
  56916.4 |
  57219.2 |#####
  57522.1 |
  57824.9 |
  58127.7 |
  58430.5 |
  58733.4 |
  59036.2 |#####
  59339.0 |##########
  59641.9 |
  59944.7 |
  60247.5 |
  60550.3 |
  60853.2 |###############
  61156.0 |##########
  (4 below, 5 above range)

bitpack-contend-d16-control (n=40, range 62421.8-65303.6 ns)
  62421.8 |########################################
  62565.8 |#######################
  62709.9 |
  62854.0 |
  62998.1 |
  63142.2 |
  63286.3 |######
  63430.4 |###
  63574.5 |###
  63718.6 |
  63862.7 |######
  64006.8 |######
  64150.9 |
  64295.0 |##########
  64439.1 |###
  64583.2 |
  64727.3 |
  64871.4 |
  65015.5 |#############
  65159.6 |###
  (1 below, 3 above range)

bitpack-contend-d32 (n=40, range 38051.3-46307.3 ns)
  38051.3 |
  38464.1 |##
  38876.9 |
  39289.7 |
  39702.5 |
  40115.3 |
  40528.1 |
  40940.9 |
  41353.7 |
  41766.5 |
  42179.3 |
  42592.1 |##
  43004.9 |
  43417.7 |
  43830.5 |
  44243.3 |
  44656.1 |
  45068.9 |########################################
  45481.7 |################
  45894.5 |
  (7 below, 3 above range)

bitpack-contend-d64 (n=40, range 50728.6-87637.5 ns)
  50728.6 |########################################
  52574.1 |####
  54419.5 |
  56265.0 |
  58110.4 |##
  59955.9 |##
  61801.3 |##
  63646.7 |
  65492.2 |
  67337.6 |
  69183.1 |##
  71028.5 |
  72874.0 |##
  74719.4 |##
  76564.9 |
  78410.3 |
  80255.8 |
  82101.2 |
  83946.6 |#################
  85792.1 |
  (4 below, 2 above range)

bitpack-contend-packed (n=40, range 44752.3-45035.2 ns)
  44752.3 |##########
  44766.4 |##########
  44780.6 |##########
  44794.7 |########################################
  44808.9 |#########################
  44823.0 |#####
  44837.1 |#####
  44851.3 |###############
  44865.4 |##########
  44879.6 |##############################
  44893.7 |#####
  44907.9 |#####
  44922.0 |#####
  44936.2 |
  44950.3 |
  44964.5 |
  44978.6 |
  44992.8 |
  45006.9 |
  45021.1 |
  (4 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 42921.8-46952.9 ns)
  42921.8 |
  43123.3 |
  43324.9 |
  43526.4 |
  43728.0 |
  43929.5 |
  44131.1 |
  44332.7 |
  44534.2 |
  44735.8 |########################################
  44937.3 |
  45138.9 |
  45340.4 |
  45542.0 |#
  45743.6 |
  45945.1 |#
  46146.7 |
  46348.2 |
  46549.8 |
  46751.3 |
  (4 below, 5 above range)

```

## Diagnostics

- **bitpack-contend-d64**: CV=24.8% (high variance, measurements may be unstable)
