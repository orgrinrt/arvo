# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe2 beats baseline by 23% (significant)

bitpack-contend-pipe2 is -57.59 us (23%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-packed-simd shows alternating (throttle bounce) (autocorr -0.69)

bitpack-contend-packed-simd's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe2, bitpack-contend-pipe4, bitpack-contend-packed-simd} vs {bitpack-contend-d16, bitpack-contend-d16-control} (25% apart)

The field splits into a fast tier {bitpack-contend-pipe2, bitpack-contend-pipe4, bitpack-contend-packed-simd} and a slow tier {bitpack-contend-d16, bitpack-contend-d16-control} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-contend-pipe2 vs stability leader bitpack-contend-pipe4 (+2% speed for 1.9x steadier)

bitpack-contend-pipe2 is fastest (196.52 us, CV 5.7%); bitpack-contend-pipe4 gives up 1.8% median for 1.9x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 196518.5 ns median (-21.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.32x (fastest 196518.5 ns, slowest 259387.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112755232 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 248387ns | 251340ns | 223144ns | 246947ns | 277953ns | base |
| bitpack-contend-d16-control | 263542ns | 259571ns | 254355ns | 260618ns | 281503ns | +6.10% |
| bitpack-contend-packed-simd | 206498ns | 200722ns | 194213ns | 204642ns | 224353ns | -16.86% |
| bitpack-contend-pipe2 | 191649ns | 196706ns | 171909ns | 195461ns | 199955ns | -22.84% |
| bitpack-contend-pipe4 | 199159ns | 200276ns | 190977ns | 200392ns | 203644ns | -19.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 248143ns | 222902ns | 277667ns | base | 16.903 |
| bitpack-contend-d16-control | 263299ns | 254085ns | 281191ns | +6.11% | 15.930 |
| bitpack-contend-packed-simd | 206244ns | 194011ns | 224003ns | -16.89% | 20.337 |
| bitpack-contend-pipe2 | 191444ns | 171687ns | 199761ns | -22.85% | 21.909 |
| bitpack-contend-pipe4 | 198947ns | 190770ns | 203425ns | -19.83% | 21.083 |

## Performance model

- Peak throughput: **24.430 Gops/s** (bitpack-contend-pipe2; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 16.703 | 68.4% |
| bitpack-contend-d16-control | 16.170 | 66.2% |
| bitpack-contend-packed-simd | 20.918 | 85.6% |
| bitpack-contend-pipe2 | 21.343 | 87.4% |
| bitpack-contend-pipe4 | 20.964 | 85.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 248387ns | 248387ns | base |
| bitpack-contend-d16-control | 263542ns | 263542ns | +6.10% |
| bitpack-contend-packed-simd | 206498ns | 206498ns | -16.86% |
| bitpack-contend-pipe2 | 191649ns | 191649ns | -22.84% |
| bitpack-contend-pipe4 | 199159ns | 199159ns | -19.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 251105ns | base | --- | [249220, 251844] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 259387ns | +8274.9ns (+3.3%) | [+4708, +11666]ns | [257009, 262433] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 200511ns | -40748.1ns (-16.2%) | [-50214, -31518]ns | [199546, 211739] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 196518ns | -57592.7ns (-22.9%) | [-63808, -50881]ns | [194424, 198954] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 200067ns | -51615.1ns (-20.6%) | [-52412, -49071]ns | [199586, 200318] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 251927ns | +3.9% | -17.4% | -22.8% | -20.5% |
| 1 | 2 | 223733ns | +24.5% | -10.9% | -10.8% | -9.1% |
| 1 | 3 | 272891ns | -5.5% | -18.0% | -27.0% | -26.9% |
| 1 | 4 | 250927ns | +1.9% | -21.3% | -30.5% | -21.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.518 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.210 | moderate- |
| bitpack-contend-packed-simd | -0.690 | HIGH- (thermal bounce) |
| bitpack-contend-pipe2 | -0.126 | ok |
| bitpack-contend-pipe4 | -0.061 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 35.8ns | 248143.0ns | 0.0% |  |
| bitpack-contend-d16-control | 42.4ns | 263299.4ns | 0.0% |  |
| bitpack-contend-packed-simd | 35.9ns | 206243.8ns | 0.0% |  |
| bitpack-contend-pipe2 | 23.7ns | 191444.3ns | 0.0% |  |
| bitpack-contend-pipe4 | 26.4ns | 198947.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 222902.4-277667.3 ns)
  222902.4 |#####################
  225640.7 |##
  228378.9 |
  231117.2 |
  233855.4 |
  236593.7 |
  239331.9 |
  242070.2 |
  244808.4 |##
  247546.6 |####
  250284.9 |########################################
  253023.1 |
  255761.4 |##
  258499.6 |##
  261237.9 |
  263976.1 |##
  266714.4 |
  269452.6 |
  272190.8 |
  274929.1 |
  (2 below, 5 above range)

bitpack-contend-d16-control (n=40, range 254084.6-281191.0 ns)
  254084.6 |##########################
  255439.9 |########################################
  256795.2 |##########################
  258150.6 |##########################
  259505.9 |######
  260861.2 |##########################
  262216.5 |####################
  263571.8 |#############
  264927.2 |
  266282.5 |
  267637.8 |
  268993.1 |
  270348.5 |
  271703.8 |
  273059.1 |
  274414.4 |
  275769.8 |######
  277125.1 |####################
  278480.4 |######
  279835.7 |
  (3 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 194011.3-224003.3 ns)
  194011.3 |
  195510.9 |##
  197010.5 |
  198510.1 |########################################
  200009.7 |########
  201509.3 |##
  203008.9 |
  204508.5 |##
  206008.1 |
  207507.7 |
  209007.3 |##
  210506.9 |
  212006.5 |########
  213506.1 |##
  215005.7 |
  216505.3 |##
  218004.9 |##
  219504.5 |
  221004.1 |##
  222503.7 |##############
  (5 below, 2 above range)

bitpack-contend-pipe2 (n=40, range 171687.0-199760.7 ns)
  171687.0 |
  173090.7 |
  174494.4 |
  175898.1 |
  177301.8 |###########
  178705.5 |
  180109.1 |
  181512.8 |
  182916.5 |
  184320.2 |
  185723.9 |
  187127.6 |
  188531.3 |
  189934.9 |
  191338.6 |##
  192742.3 |##
  194146.0 |##################
  195549.7 |
  196953.4 |
  198357.1 |########################################
  (5 below, 3 above range)

bitpack-contend-pipe4 (n=40, range 190769.8-203425.5 ns)
  190769.8 |
  191402.6 |
  192035.4 |
  192668.2 |
  193300.9 |
  193933.7 |
  194566.5 |
  195199.3 |
  195832.1 |
  196464.9 |####
  197097.6 |
  197730.4 |########
  198363.2 |####
  198996.0 |################################
  199628.8 |########################################
  200261.5 |####################
  200894.3 |
  201527.1 |
  202159.9 |
  202792.7 |########################
  (3 below, 4 above range)

```
