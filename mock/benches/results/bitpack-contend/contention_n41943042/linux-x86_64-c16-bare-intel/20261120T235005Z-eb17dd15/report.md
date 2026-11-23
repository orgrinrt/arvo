# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 23% (significant)

bitpack-contend-packed is -94.72 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.9x slower than the field

bitpack-contend-d64 (874.43 us) is 2.9x the fastest (304.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-packed-simd shows alternating (throttle bounce) (autocorr -0.70)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} vs {bitpack-contend-d64} (102% apart)

The field splits into a fast tier {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16-control, bitpack-contend-d16, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 102% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-contend-packed vs stability leader bitpack-contend-packed-simd (+8% speed for 4.6x steadier)

bitpack-contend-packed is fastest (304.74 us, CV 11.0%); bitpack-contend-packed-simd gives up 8.4% median for 4.6x lower variance (CV 2.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-packed** at 304743.8 ns median (-26.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.87x (fastest 304743.8 ns, slowest 874429.0 ns)

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
| bitpack-contend-d16 | 403741ns | 412713ns | 330624ns | 408552ns | 462426ns | base |
| bitpack-contend-d16-control | 416987ns | 405535ns | 380688ns | 411599ns | 469450ns | +3.28% |
| bitpack-contend-d32 | 457943ns | 432882ns | 417840ns | 444744ns | 537642ns | +13.42% |
| bitpack-contend-d64 | 891959ns | 876151ns | 863025ns | 883572ns | 946051ns | +120.92% |
| bitpack-contend-packed | 300848ns | 304959ns | 260109ns | 301834ns | 338629ns | -25.48% |
| bitpack-contend-packed-simd | 333523ns | 330483ns | 324596ns | 332273ns | 346199ns | -17.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 403374ns | 330326ns | 461818ns | base | 10.398 |
| bitpack-contend-d16-control | 416645ns | 380348ns | 469051ns | +3.29% | 10.067 |
| bitpack-contend-d32 | 457005ns | 416941ns | 536666ns | +13.30% | 9.178 |
| bitpack-contend-d64 | 890400ns | 861484ns | 944548ns | +120.74% | 4.711 |
| bitpack-contend-packed | 300642ns | 259925ns | 338396ns | -25.47% | 13.951 |
| bitpack-contend-packed-simd | 333292ns | 324404ns | 345940ns | -17.37% | 12.584 |

## Performance model

- Peak throughput: **16.137 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 10.170 | 63.0% |
| bitpack-contend-d16-control | 10.349 | 64.1% |
| bitpack-contend-d32 | 9.713 | 60.2% |
| bitpack-contend-d64 | 4.797 | 29.7% |
| bitpack-contend-packed | 13.763 | 85.3% |
| bitpack-contend-packed-simd | 12.700 | 78.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 403741ns | 403741ns | base |
| bitpack-contend-d16-control | 416987ns | 416987ns | +3.28% |
| bitpack-contend-d32 | 457943ns | 457943ns | +13.42% |
| bitpack-contend-d64 | 891959ns | 891959ns | +120.92% |
| bitpack-contend-packed | 300848ns | 300848ns | -25.48% |
| bitpack-contend-packed-simd | 333523ns | 333523ns | -17.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 412424ns | base | --- | [410258, 416655] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 405268ns | no significant difference | [-7923, +52896]ns | [400218, 413286] | no | 0.6358 | 0.6358 | 0 |
| bitpack-contend-d32 | 431836ns | +42335.6ns (+10.3%) | [+14176, +82988]ns | [429046, 451137] | YES | 0.0008 | 0.0007 | 0 |
| bitpack-contend-d64 | 874429ns | +463222.0ns (+112.3%) | [+456443, +525789]ns | [870331, 889741] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 304744ns | -94715.4ns (-23.0%) | [-108007, -84179]ns | [273916, 328574] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 330271ns | -85165.2ns (-20.6%) | [-88722, -83432]ns | [328579, 335359] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 410877ns | -1.7% | +18.9% | +118.1% | -32.8% | -20.9% |
| 1 | 2 | 444608ns | -14.0% | -3.4% | +96.1% | -24.2% | -22.5% |
| 1 | 3 | 413199ns | +12.7% | +8.0% | +112.1% | -20.5% | -20.4% |
| 1 | 4 | 334890ns | +23.3% | +24.8% | +163.0% | -22.3% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.025 | ok |
| bitpack-contend-d16-control | -0.381 | moderate- |
| bitpack-contend-d32 | -0.251 | moderate- |
| bitpack-contend-d64 | -0.280 | moderate- |
| bitpack-contend-packed | -0.229 | moderate- |
| bitpack-contend-packed-simd | -0.698 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-d32**: won 1/4, lost 3/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 30.0ns | 403374.2ns | 0.0% |  |
| bitpack-contend-d16-control | 34.1ns | 416645.1ns | 0.0% |  |
| bitpack-contend-d32 | 137.0ns | 457005.4ns | 0.0% |  |
| bitpack-contend-d64 | 279.5ns | 890400.4ns | 0.0% |  |
| bitpack-contend-packed | 32.5ns | 300641.6ns | 0.0% |  |
| bitpack-contend-packed-simd | 41.7ns | 333291.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 330326.1-461818.1 ns)
  330326.1 |##
  336900.7 |##############
  343475.3 |
  350049.9 |
  356624.5 |
  363199.1 |
  369773.7 |
  376348.3 |
  382922.9 |
  389497.5 |
  396072.1 |##
  402646.7 |##
  409221.3 |########################################
  415795.9 |###########
  422370.5 |
  428945.1 |##
  435519.7 |#####
  442094.3 |###########
  448668.9 |
  455243.5 |
  (4 below, 3 above range)

bitpack-contend-d16-control (n=40, range 380348.2-469051.5 ns)
  380348.2 |###############
  384783.4 |##########
  389218.6 |#####
  393653.7 |###############
  398088.9 |####################
  402524.0 |####################
  406959.2 |#####
  411394.4 |#########################
  415829.5 |#####
  420264.7 |#####
  424699.8 |
  429135.0 |
  433570.2 |#####
  438005.3 |
  442440.5 |
  446875.7 |
  451310.8 |
  455746.0 |
  460181.1 |
  464616.3 |########################################
  (4 below, 2 above range)

bitpack-contend-d32 (n=40, range 416941.2-536665.8 ns)
  416941.2 |##############################
  422927.4 |#########################
  428913.6 |########################################
  434899.9 |#####
  440886.1 |##########
  446872.3 |###############
  452858.5 |#####
  458844.8 |
  464831.0 |
  470817.2 |
  476803.5 |#####
  482789.7 |##########
  488775.9 |#####
  494762.2 |
  500748.4 |
  506734.6 |
  512720.8 |##########
  518707.1 |#####
  524693.3 |###############
  530679.5 |
  (2 below, 2 above range)

bitpack-contend-d64 (n=40, range 861484.5-944547.6 ns)
  861484.5 |##########
  865637.7 |#########################
  869790.8 |########################################
  873944.0 |##############################
  878097.1 |#####
  882250.3 |
  886403.4 |
  890556.6 |
  894709.7 |#####
  898862.9 |##########
  903016.1 |
  907169.2 |
  911322.4 |#####
  915475.5 |
  919628.7 |##########
  923781.8 |#####
  927935.0 |
  932088.2 |
  936241.3 |##########
  940394.5 |##########
  (4 below, 3 above range)

bitpack-contend-packed (n=40, range 259925.2-338395.7 ns)
  259925.2 |###############################
  263848.8 |
  267772.3 |
  271695.8 |######################
  275619.3 |#################
  279542.9 |####
  283466.4 |
  287389.9 |
  291313.4 |
  295236.9 |
  299160.5 |
  303084.0 |
  307007.5 |
  310931.0 |
  314854.6 |
  318778.1 |
  322701.6 |
  326625.1 |########################################
  330548.6 |####
  334472.2 |###################################
  (3 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 324403.7-345940.0 ns)
  324403.7 |################################
  325480.5 |
  326557.4 |########
  327634.2 |########################################
  328711.0 |########################################
  329787.8 |
  330864.6 |########
  331941.4 |
  333018.2 |
  334095.0 |################################
  335171.9 |################################
  336248.7 |
  337325.5 |########
  338402.3 |
  339479.1 |########################
  340555.9 |
  341632.7 |
  342709.6 |
  343786.4 |################
  344863.2 |################
  (5 below, 3 above range)

```
