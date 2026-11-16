# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd beats baseline by 29% (significant)

bitpack-contend-packed-simd is -80.15 us (29%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-packed-simd, bitpack-contend-pipe4) are a dead heat (<1%)

bitpack-contend-packed-simd (198.46 us) and bitpack-contend-pipe4 (199.88 us) differ by 0.71%, inside the noise, even though the wider field spreads 58.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d32 shows alternating (throttle bounce) (autocorr -0.55)

bitpack-contend-d32's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed-simd, bitpack-contend-pipe4} vs {bitpack-contend-d16-padal, bitpack-contend-d16, bitpack-contend-d32-padal, bitpack-contend-d32} (28% apart)

The field splits into a fast tier {bitpack-contend-packed-simd, bitpack-contend-pipe4} and a slow tier {bitpack-contend-d16-padal, bitpack-contend-d16, bitpack-contend-d32-padal, bitpack-contend-d32} with a 28% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-packed-simd** at 198459.1 ns median (-28.7% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.58x (fastest 198459.1 ns, slowest 314127.5 ns)

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
| Counter frequency | 2112792238 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 276248ns | 278595ns | 247780ns | 278865ns | 296864ns | base |
| bitpack-contend-d16-padal | 251145ns | 256550ns | 219897ns | 253235ns | 276124ns | -9.09% |
| bitpack-contend-d32 | 320778ns | 314775ns | 293762ns | 314684ns | 366075ns | +16.12% |
| bitpack-contend-d32-padal | 316145ns | 307007ns | 289564ns | 310853ns | 358603ns | +14.44% |
| bitpack-contend-packed-simd | 196261ns | 198650ns | 182216ns | 198635ns | 203188ns | -28.95% |
| bitpack-contend-pipe4 | 195279ns | 200100ns | 175538ns | 198542ns | 205230ns | -29.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 275862ns | 247411ns | 296439ns | base | 15.204 |
| bitpack-contend-d16-padal | 250849ns | 219585ns | 275733ns | -9.07% | 16.720 |
| bitpack-contend-d32 | 320004ns | 292936ns | 365075ns | +16.00% | 13.107 |
| bitpack-contend-d32-padal | 315346ns | 288820ns | 357727ns | +14.31% | 13.301 |
| bitpack-contend-packed-simd | 196057ns | 181998ns | 202992ns | -28.93% | 21.393 |
| bitpack-contend-pipe4 | 195060ns | 175360ns | 204972ns | -29.29% | 21.503 |

## Performance model

- Peak throughput: **23.918 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 15.075 | 63.0% |
| bitpack-contend-d16-padal | 16.363 | 68.4% |
| bitpack-contend-d32 | 13.352 | 55.8% |
| bitpack-contend-d32-padal | 13.696 | 57.3% |
| bitpack-contend-packed-simd | 21.134 | 88.4% |
| bitpack-contend-pipe4 | 20.985 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 276248ns | 276248ns | base |
| bitpack-contend-d16-padal | 251145ns | 251145ns | -9.09% |
| bitpack-contend-d32 | 320778ns | 320778ns | +16.12% |
| bitpack-contend-d32-padal | 316145ns | 316145ns | +14.44% |
| bitpack-contend-packed-simd | 196261ns | 196261ns | -28.95% |
| bitpack-contend-pipe4 | 195279ns | 195279ns | -29.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 278234ns | base | --- | [275600, 282020] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 256326ns | -24052.2ns (-8.6%) | [-38068, -16698]ns | [255198, 259051] | YES | 0.0064 | 0.0064 | 0 |
| bitpack-contend-d32 | 314128ns | +41420.6ns (+14.9%) | [+30197, +48905]ns | [302698, 328015] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 306253ns | +29934.8ns (+10.8%) | [+22871, +43796]ns | [302565, 318006] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 198459ns | -80148.1ns (-28.8%) | [-86215, -77913]ns | [197877, 199486] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 199875ns | -79249.9ns (-28.5%) | [-88542, -77179]ns | [199393, 200223] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 283791ns | -9.8% | +5.2% | +16.6% | -29.6% | -37.2% |
| 1 | 2 | 284832ns | -21.2% | +18.5% | +10.1% | -30.0% | -29.8% |
| 1 | 3 | 269207ns | -4.4% | +14.0% | +13.2% | -28.1% | -24.3% |
| 1 | 4 | 277246ns | -2.9% | +11.3% | +11.4% | -28.6% | -27.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.083 | ok |
| bitpack-contend-d16-padal | -0.157 | ok |
| bitpack-contend-d32 | -0.548 | HIGH- (thermal bounce) |
| bitpack-contend-d32-padal | 0.123 | ok |
| bitpack-contend-packed-simd | -0.217 | moderate- |
| bitpack-contend-pipe4 | -0.007 | ok |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 4/4, lost 0/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d32-padal**: won 0/4, lost 4/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 61.0ns | 275862.4ns | 0.0% |  |
| bitpack-contend-d16-padal | 35.5ns | 250848.6ns | 0.0% |  |
| bitpack-contend-d32 | 85.0ns | 320003.8ns | 0.0% |  |
| bitpack-contend-d32-padal | 61.1ns | 315345.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 35.0ns | 196057.0ns | 0.0% |  |
| bitpack-contend-pipe4 | 23.7ns | 195060.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 247410.9-296438.6 ns)
  247410.9 |#############
  249862.3 |
  252313.7 |######
  254765.1 |#############
  257216.5 |######
  259667.9 |
  262119.2 |######
  264570.6 |
  267022.0 |
  269473.4 |######
  271924.8 |#############
  274376.2 |##########################
  276827.5 |########################################
  279278.9 |##########################
  281730.3 |#############
  284181.7 |##########################
  286633.1 |
  289084.5 |######
  291535.8 |#############
  293987.2 |#############
  (2 below, 3 above range)

bitpack-contend-d16-padal (n=40, range 219585.1-275733.1 ns)
  219585.1 |#####
  222392.5 |#################
  225199.9 |######################
  228007.3 |
  230814.7 |
  233622.1 |
  236429.5 |
  239236.9 |
  242044.3 |
  244851.7 |
  247659.1 |
  250466.5 |
  253273.9 |########################################
  256081.3 |########################################
  258888.7 |#################
  261696.1 |
  264503.5 |############################
  267310.9 |
  270118.3 |#####
  272925.7 |
  (4 below, 5 above range)

bitpack-contend-d32 (n=40, range 292936.4-365074.8 ns)
  292936.4 |########################################
  296543.3 |##########
  300150.3 |##########
  303757.2 |#####
  307364.1 |##########
  310971.0 |###############
  314577.9 |####################
  318184.9 |
  321791.8 |
  325398.7 |####################
  329005.6 |##########
  332612.5 |
  336219.4 |
  339826.4 |
  343433.3 |###############
  347040.2 |##########
  350647.1 |
  354254.0 |
  357861.0 |
  361467.9 |
  (3 below, 4 above range)

bitpack-contend-d32-padal (n=40, range 288820.4-357726.6 ns)
  288820.4 |
  292265.7 |####################
  295711.0 |####################
  299156.3 |#################################
  302601.6 |########################################
  306046.9 |######
  309492.3 |
  312937.6 |#################################
  316382.9 |
  319828.2 |##########################
  323273.5 |#############
  326718.8 |####################
  330164.1 |
  333609.4 |
  337054.7 |
  340500.0 |
  343945.4 |
  347390.7 |
  350836.0 |
  354281.3 |
  (3 below, 5 above range)

bitpack-contend-packed-simd (n=40, range 181998.4-202992.4 ns)
  181998.4 |
  183048.1 |
  184097.8 |
  185147.5 |
  186197.2 |
  187246.9 |
  188296.6 |
  189346.3 |
  190396.0 |###
  191445.7 |
  192495.4 |###
  193545.1 |###
  194594.8 |
  195644.5 |
  196694.2 |#########
  197743.9 |###########################
  198793.6 |########################################
  199843.3 |###############
  200893.0 |
  201942.7 |
  (6 below, 1 above range)

bitpack-contend-pipe4 (n=40, range 175360.5-204972.3 ns)
  175360.5 |
  176841.0 |##########
  178321.6 |##########
  179802.2 |
  181282.8 |
  182763.4 |
  184244.0 |
  185724.6 |
  187205.2 |
  188685.8 |
  190166.4 |
  191647.0 |
  193127.6 |
  194608.1 |##
  196088.7 |
  197569.3 |#####
  199049.9 |########################################
  200530.5 |
  202011.1 |##
  203491.7 |######################
  (2 below, 1 above range)

```
