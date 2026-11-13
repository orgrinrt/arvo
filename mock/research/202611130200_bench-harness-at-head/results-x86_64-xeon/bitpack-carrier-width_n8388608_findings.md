# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 11% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-packed (2.14 ms) leads bitpack-carrier-d32 (2.37 ms) by 11%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 32% (significant)

bitpack-carrier-packed is -984.92 us (32%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d16 shows alternating (throttle bounce) (autocorr -0.80)

bitpack-carrier-d16's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed, bitpack-carrier-d32} vs {bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d64} (30% apart)

The field splits into a fast tier {bitpack-carrier-packed, bitpack-carrier-d32} and a slow tier {bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d64} with a 30% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-packed** at 2137077.1 ns median (-30.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.57x (fastest 2137077.1 ns, slowest 3360238.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3066390ns | 3069242ns | 2842206ns | 3062371ns | 3302634ns | base |
| bitpack-carrier-d16-control | 3201467ns | 3169790ns | 2795100ns | 3169419ns | 3703977ns | +4.41% |
| bitpack-carrier-d32 | 2273227ns | 2368719ns | 1861932ns | 2311249ns | 2570454ns | -25.87% |
| bitpack-carrier-d64 | 3389650ns | 3362384ns | 3080465ns | 3382692ns | 3719710ns | +10.54% |
| bitpack-carrier-packed | 2065195ns | 2139423ns | 1763269ns | 2107583ns | 2239957ns | -32.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3064113ns | 2840002ns | 3300195ns | base | 2.738 |
| bitpack-carrier-d16-control | 3199339ns | 2793215ns | 3701427ns | +4.41% | 2.622 |
| bitpack-carrier-d32 | 2271121ns | 1860083ns | 2568121ns | -25.88% | 3.694 |
| bitpack-carrier-d64 | 3387336ns | 3078421ns | 3717064ns | +10.55% | 2.476 |
| bitpack-carrier-packed | 2063035ns | 1761518ns | 2237564ns | -32.67% | 4.066 |

## Performance model

- Peak throughput: **4.762 Gops/s** (bitpack-carrier-packed; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 2.735 | 57.4% |
| bitpack-carrier-d16-control | 2.648 | 55.6% |
| bitpack-carrier-d32 | 3.545 | 74.4% |
| bitpack-carrier-d64 | 2.496 | 52.4% |
| bitpack-carrier-packed | 3.925 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 3066390ns | 3066390ns | base |
| bitpack-carrier-d16-control | 3201467ns | 3201467ns | +4.41% |
| bitpack-carrier-d32 | 2273227ns | 2273227ns | -25.87% |
| bitpack-carrier-d64 | 3389650ns | 3389650ns | +10.54% |
| bitpack-carrier-packed | 2065195ns | 2065195ns | -32.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 3066866ns | base | --- | [2991016, 3121851] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 3167879ns | no significant difference | [-10580, +272830]ns | [3060706, 3227276] | no | 0.0807 | 0.0807 | 0 |
| bitpack-carrier-d32 | 2366526ns | -740902.3ns (-24.2%) | [-908335, -649973]ns | [2298954, 2411919] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 3360239ns | +322463.4ns (+10.5%) | [+219402, +379981]ns | [3316043, 3440169] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-carrier-packed | 2137077ns | -984923.5ns (-32.1%) | [-1152610, -870455]ns | [2095231, 2159355] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 3002782ns | -2.4% | -31.9% | +12.1% | -29.6% |
| 1 | 2 | 3131796ns | -0.9% | -24.0% | +6.3% | -31.6% |
| 1 | 3 | 2987674ns | +6.7% | -20.0% | +19.4% | -40.9% |
| 1 | 4 | 3098248ns | +13.9% | -23.1% | +6.4% | -30.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.805 | HIGH- (thermal bounce) |
| bitpack-carrier-d16-control | 0.111 | ok |
| bitpack-carrier-d32 | -0.068 | ok |
| bitpack-carrier-d64 | -0.584 | HIGH- (thermal bounce) |
| bitpack-carrier-packed | -0.509 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 2/4, lost 2/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 1045.6ns | 3064113.0ns | 0.0% |  |
| bitpack-carrier-d16-control | 807.9ns | 3199338.8ns | 0.0% |  |
| bitpack-carrier-d32 | 931.9ns | 2271121.5ns | 0.0% |  |
| bitpack-carrier-d64 | 955.3ns | 3387335.5ns | 0.0% |  |
| bitpack-carrier-packed | 870.9ns | 2063035.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 2840002.2-3300195.2 ns)
  2840002.2 |
  2863011.8 |########################
  2886021.5 |
  2909031.1 |################
  2932040.8 |
  2955050.4 |########################
  2978060.1 |########################################
  3001069.7 |################
  3024079.4 |
  3047089.0 |################################
  3070098.7 |########
  3093108.4 |########################
  3116118.0 |################
  3139127.7 |################
  3162137.3 |################
  3185147.0 |########
  3208156.6 |########
  3231166.3 |
  3254175.9 |########
  3277185.6 |########
  (3 below, 4 above range)

bitpack-carrier-d16-control (n=40, range 2793215.2-3701427.0 ns)
  2793215.2 |######
  2838625.7 |#############
  2884036.3 |#############
  2929446.9 |
  2974857.5 |####################
  3020268.1 |####################
  3065678.7 |#############
  3111089.3 |####################
  3156499.9 |########################################
  3201910.5 |#############
  3247321.1 |######
  3292731.7 |######
  3338142.3 |#############
  3383552.9 |
  3428963.5 |####################
  3474374.0 |######
  3519784.6 |
  3565195.2 |######
  3610605.8 |
  3656016.4 |
  (3 below, 4 above range)

bitpack-carrier-d32 (n=40, range 1860083.2-2568121.3 ns)
  1860083.2 |
  1895485.1 |########################
  1930887.0 |
  1966288.9 |########
  2001690.8 |########################
  2037092.7 |########
  2072494.6 |
  2107896.5 |
  2143298.4 |
  2178700.4 |
  2214102.3 |
  2249504.2 |########
  2284906.1 |
  2320308.0 |########################################
  2355709.9 |########################################
  2391111.8 |########################
  2426513.7 |################################
  2461915.6 |################
  2497317.5 |########
  2532719.4 |################################
  (5 below, 2 above range)

bitpack-carrier-d64 (n=40, range 3078420.6-3717063.9 ns)
  3078420.6 |
  3110352.8 |
  3142285.0 |
  3174217.1 |########
  3206149.3 |################
  3238081.5 |########################
  3270013.6 |################
  3301945.8 |################################
  3333877.9 |########################################
  3365810.1 |########################
  3397742.3 |################
  3429674.4 |########
  3461606.6 |################
  3493538.8 |
  3525470.9 |################
  3557403.1 |########################
  3589335.3 |########
  3621267.4 |########
  3653199.6 |
  3685131.8 |
  (4 below, 4 above range)

bitpack-carrier-packed (n=40, range 1761518.3-2237564.5 ns)
  1761518.3 |####################
  1785320.6 |#####
  1809122.9 |
  1832925.2 |
  1856727.5 |
  1880529.8 |
  1904332.1 |
  1928134.4 |
  1951936.7 |##########
  1975739.1 |
  1999541.4 |
  2023343.7 |
  2047146.0 |##########
  2070948.3 |#####
  2094750.6 |#####
  2118552.9 |##############################
  2142355.2 |########################################
  2166157.5 |###############
  2189959.8 |###############
  2213762.2 |#####
  (4 below, 4 above range)

```
