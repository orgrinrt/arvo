# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32 beats baseline by 23% (significant)

bitpack-contend-d32 is -37.50 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d16-control shows alternating (throttle bounce) (autocorr -0.78)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-d32 vs stability leader bitpack-contend-packed (+2% speed for 2.6x steadier)

bitpack-contend-d32 is fastest (129.30 us, CV 3.2%); bitpack-contend-packed gives up 2.4% median for 2.6x lower variance (CV 1.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-contend-d64 is inconsistent: worst-20% is 1.9x its best-20%

bitpack-contend-d64's best 20% of batches run at 169.66 us but its worst 20% at 323.94 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d32** at 129296.8 ns median (-22.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.67x (fastest 129296.8 ns, slowest 215726.0 ns)

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
| bitpack-contend-d16 | 165019ns | 166724ns | 160000ns | 165758ns | 167824ns | base |
| bitpack-contend-d16-control | 164224ns | 165865ns | 159829ns | 164206ns | 168670ns | -0.48% |
| bitpack-contend-d32 | 127435ns | 129501ns | 120098ns | 128848ns | 130535ns | -22.78% |
| bitpack-contend-d64 | 232529ns | 216016ns | 169905ns | 222802ns | 324335ns | +40.91% |
| bitpack-contend-packed | 131776ns | 132610ns | 128901ns | 132312ns | 133040ns | -20.15% |
| bitpack-contend-packed-simd | 130931ns | 130894ns | 129102ns | 130907ns | 132830ns | -20.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 164854ns | 159824ns | 167662ns | base | 6.361 |
| bitpack-contend-d16-control | 163977ns | 159655ns | 168210ns | -0.53% | 6.395 |
| bitpack-contend-d32 | 127264ns | 119906ns | 130379ns | -22.80% | 8.239 |
| bitpack-contend-d64 | 232218ns | 169659ns | 323942ns | +40.86% | 4.515 |
| bitpack-contend-packed | 131570ns | 128711ns | 132854ns | -20.19% | 7.970 |
| bitpack-contend-packed-simd | 130769ns | 128944ns | 132671ns | -20.68% | 8.019 |

## Performance model

- Peak throughput: **8.745 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.296 | 72.0% |
| bitpack-contend-d16-control | 6.333 | 72.4% |
| bitpack-contend-d32 | 8.110 | 92.7% |
| bitpack-contend-d64 | 4.861 | 55.6% |
| bitpack-contend-packed | 7.921 | 90.6% |
| bitpack-contend-packed-simd | 8.020 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 165019ns | 165019ns | base |
| bitpack-contend-d16-control | 164224ns | 164224ns | -0.48% |
| bitpack-contend-d32 | 127435ns | 127435ns | -22.78% |
| bitpack-contend-d64 | 232529ns | 232529ns | +40.91% |
| bitpack-contend-packed | 131776ns | 131776ns | -20.15% |
| bitpack-contend-packed-simd | 130931ns | 130931ns | -20.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 166540ns | base | --- | [166409, 166787] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 165586ns | no significant difference | [-6064, +2614]ns | [160645, 166845] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-d32 | 129297ns | -37499.3ns (-22.5%) | [-37985, -37039]ns | [129288, 129484] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 215726ns | +51954.2ns (+31.2%) | [+39971, +81559]ns | [207177, 248373] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 132379ns | -34124.0ns (-20.5%) | [-34296, -33601]ns | [132300, 132443] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 130739ns | -34269.9ns (-20.6%) | [-34696, -33809]ns | [128980, 132467] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 166724ns | -3.8% | -28.1% | +67.6% | -20.6% | -22.7% |
| 1 | 2 | 166561ns | +0.6% | -22.3% | +25.9% | -20.4% | -20.3% |
| 1 | 3 | 166822ns | -4.0% | -22.5% | +2.4% | -20.6% | -20.6% |
| 1 | 4 | 159882ns | +4.3% | -18.7% | +75.7% | -19.5% | -19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.108 | ok |
| bitpack-contend-d16-control | -0.783 | HIGH- (thermal bounce) |
| bitpack-contend-d32 | -0.080 | ok |
| bitpack-contend-d64 | -0.275 | moderate- |
| bitpack-contend-packed | -0.067 | ok |
| bitpack-contend-packed-simd | -0.251 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 29.5ns | 164854.3ns | 0.0% |  |
| bitpack-contend-d16-control | 32.9ns | 163976.7ns | 0.0% |  |
| bitpack-contend-d32 | 28.8ns | 127263.7ns | 0.0% |  |
| bitpack-contend-d64 | 55.7ns | 232218.5ns | 0.0% |  |
| bitpack-contend-packed | 32.7ns | 131569.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 32.6ns | 130769.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 159824.2-167662.3 ns)
  159824.2 |##########
  160216.1 |#######
  160608.0 |###
  160999.9 |
  161391.8 |
  161783.7 |
  162175.6 |
  162567.6 |###
  162959.5 |###
  163351.4 |
  163743.3 |
  164135.2 |
  164527.1 |
  164919.0 |
  165310.9 |
  165702.8 |
  166094.7 |##################
  166486.6 |########################################
  166878.5 |##############
  167270.4 |##########
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 159655.0-168210.3 ns)
  159655.0 |########################
  160082.7 |################################
  160510.5 |########################
  160938.3 |################
  161366.0 |################
  161793.8 |
  162221.6 |
  162649.3 |
  163077.1 |
  163504.9 |
  163932.6 |
  164360.4 |
  164788.2 |########
  165216.0 |
  165643.7 |
  166071.5 |########################
  166499.3 |################################
  166927.0 |################################
  167354.8 |########################################
  167782.6 |########
  (5 below, 3 above range)

bitpack-contend-d32 (n=40, range 119905.9-130378.7 ns)
  119905.9 |##################
  120429.5 |
  120953.2 |###
  121476.8 |
  122000.5 |
  122524.1 |
  123047.7 |
  123571.4 |
  124095.0 |
  124618.7 |
  125142.3 |
  125665.9 |
  126189.6 |
  126713.2 |
  127236.9 |
  127760.5 |
  128284.1 |
  128807.8 |########################################
  129331.4 |################################
  129855.1 |#########################
  (4 below, 3 above range)

bitpack-contend-d64 (n=40, range 169659.0-323941.8 ns)
  169659.0 |########################################
  177373.1 |
  185087.2 |####
  192801.4 |
  200515.5 |####
  208229.7 |########################
  215943.8 |################
  223657.9 |
  231372.1 |
  239086.2 |
  246800.4 |####################
  254514.5 |
  262228.7 |
  269942.8 |####
  277656.9 |
  285371.1 |####################
  293085.2 |
  300799.4 |####
  308513.5 |
  316227.6 |
  (2 below, 4 above range)

bitpack-contend-packed (n=40, range 128711.5-132853.5 ns)
  128711.5 |##########
  128918.6 |
  129125.7 |
  129332.8 |
  129539.9 |
  129747.0 |
  129954.1 |
  130161.2 |
  130368.3 |
  130575.4 |
  130782.5 |
  130989.6 |
  131196.7 |
  131403.8 |
  131610.9 |
  131818.0 |
  132025.1 |
  132232.2 |########################################
  132439.3 |##############################
  132646.4 |
  (6 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 128944.3-132671.0 ns)
  128944.3 |########################################
  129130.6 |
  129317.0 |
  129503.3 |
  129689.7 |
  129876.0 |
  130062.3 |
  130248.7 |
  130435.0 |
  130621.3 |
  130807.7 |
  130994.0 |
  131180.3 |
  131366.7 |
  131553.0 |
  131739.4 |
  131925.7 |
  132112.0 |
  132298.4 |####################
  132484.7 |####################
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d64**: CV=24.9% (high variance, measurements may be unstable)
