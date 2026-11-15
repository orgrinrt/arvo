# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn dominates: 32% faster than the next best (warm-clamp-accfit)

warm-clamp-accfit-dyn (197.79 us) leads warm-clamp-accfit (261.46 us) by 32%, a clear separation rather than a photo finish. CV 19.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit-dyn beats baseline by 49% (significant)

warm-clamp-accfit-dyn is -202.83 us (49%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.3x slower than the field

warm-clamp-minimum (451.94 us) is 2.3x the fastest (197.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 19.3%)

warm-clamp-accfit-dyn wins on median (197.79 us) yet has the highest variance (CV 19.3%), while warm-clamp-acc64 is the steadiest (CV 4.9%, 414.79 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.60)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit is inconsistent: worst-20% is 1.5x its best-20%

warm-clamp-accfit's best 20% of batches run at 201.15 us but its worst 20% at 308.45 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 197794.0 ns median (-52.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.28x (fastest 197794.0 ns, slowest 451939.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 417389ns | 415011ns | 394406ns | 414876ns | 447910ns | base |
| warm-clamp-accfit | 252334ns | 261764ns | 201510ns | 250428ns | 308876ns | -39.54% |
| warm-clamp-accfit-dyn | 218285ns | 197950ns | 196129ns | 203013ns | 286255ns | -47.70% |
| warm-clamp-head | 461534ns | 440798ns | 411607ns | 441175ns | 572535ns | +10.58% |
| warm-clamp-min-lanes | 443048ns | 436888ns | 363650ns | 445985ns | 513636ns | +6.15% |
| warm-clamp-minimum | 448963ns | 452227ns | 415433ns | 447243ns | 487654ns | +7.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 417191ns | 394226ns | 447715ns | base | 2.513 |
| warm-clamp-accfit | 251979ns | 201149ns | 308453ns | -39.60% | 4.161 |
| warm-clamp-accfit-dyn | 218104ns | 195965ns | 286010ns | -47.72% | 4.808 |
| warm-clamp-head | 460601ns | 410818ns | 571594ns | +10.41% | 2.277 |
| warm-clamp-min-lanes | 442692ns | 363428ns | 513160ns | +6.11% | 2.369 |
| warm-clamp-minimum | 448652ns | 415150ns | 487311ns | +7.54% | 2.337 |

## Performance model

- Peak throughput: **5.351 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.528 | 47.2% |
| warm-clamp-accfit | 4.010 | 75.0% |
| warm-clamp-accfit-dyn | 5.301 | 99.1% |
| warm-clamp-head | 2.385 | 44.6% |
| warm-clamp-min-lanes | 2.402 | 44.9% |
| warm-clamp-minimum | 2.320 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 417389ns | 417389ns | base |
| warm-clamp-accfit | 252334ns | 252334ns | -39.54% |
| warm-clamp-accfit-dyn | 218285ns | 218285ns | -47.70% |
| warm-clamp-head | 461534ns | 461534ns | +10.58% |
| warm-clamp-min-lanes | 443048ns | 443048ns | +6.15% |
| warm-clamp-minimum | 448963ns | 448963ns | +7.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 414793ns | base | --- | [404022, 426025] | --- | --- | --- | --- |
| warm-clamp-accfit | 261458ns | -169502.4ns (-40.9%) | [-182129, -137290]ns | [231197, 269542] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 197794ns | -202825.9ns (-48.9%) | [-208726, -195173]ns | [196253, 199166] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 439727ns | +17450.5ns (+4.2%) | [+14410, +35978]ns | [435954, 441388] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 436518ns | +34150.2ns (+8.2%) | [+12447, +39408]ns | [429650, 475196] | YES | 0.0022 | 0.0022 | 0 |
| warm-clamp-minimum | 451939ns | +42290.7ns (+10.2%) | [+30171, +44907]ns | [436234, 455619] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 442284ns | -37.6% | -40.7% | -1.5% | +13.5% | +9.0% |
| 1 | 2 | 428419ns | -53.0% | -54.3% | +2.6% | +3.0% | -2.9% |
| 1 | 3 | 404405ns | -36.3% | -51.5% | +9.1% | -10.1% | +12.7% |
| 1 | 4 | 394931ns | -29.9% | -49.6% | +4.0% | +9.1% | +10.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.299 | moderate+ |
| warm-clamp-accfit | -0.347 | moderate- |
| warm-clamp-accfit-dyn | -0.096 | ok |
| warm-clamp-head | -0.157 | ok |
| warm-clamp-min-lanes | 0.023 | ok |
| warm-clamp-minimum | -0.604 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 1/4, lost 3/4
- **warm-clamp-min-lanes**: won 1/4, lost 3/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 53.1ns | 417190.6ns | 0.0% |  |
| warm-clamp-accfit | 37.1ns | 251979.1ns | 0.0% |  |
| warm-clamp-accfit-dyn | 32.7ns | 218103.9ns | 0.0% |  |
| warm-clamp-head | 187.7ns | 460601.0ns | 0.0% |  |
| warm-clamp-min-lanes | 47.7ns | 442691.9ns | 0.0% |  |
| warm-clamp-minimum | 68.7ns | 448652.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 394226.1-447714.9 ns)
  394226.1 |########################################
  396900.5 |########
  399575.0 |########
  402249.4 |########################################
  404923.9 |################################
  407598.3 |
  410272.7 |
  412947.2 |
  415621.6 |
  418296.1 |
  420970.5 |########################################
  423644.9 |########
  426319.4 |########################################
  428993.8 |########################
  431668.3 |########
  434342.7 |
  437017.1 |
  439691.6 |
  442366.0 |
  445040.5 |
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 201148.9-308452.8 ns)
  201148.9 |########################################
  206514.1 |
  211879.3 |#################
  217244.5 |
  222609.7 |
  227974.9 |
  233340.1 |
  238705.3 |
  244070.5 |######################
  249435.7 |#####
  254800.8 |
  260166.0 |############################
  265531.2 |#################
  270896.4 |#################
  276261.6 |
  281626.8 |#####
  286992.0 |######################
  292357.2 |#####
  297722.4 |
  303087.6 |
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 195965.4-286010.1 ns)
  195965.4 |########################################
  200467.6 |
  204969.9 |
  209472.1 |
  213974.3 |
  218476.6 |
  222978.8 |
  227481.0 |
  231983.3 |
  236485.5 |
  240987.8 |
  245490.0 |
  249992.2 |
  254494.5 |
  258996.7 |#########
  263498.9 |
  268001.2 |
  272503.4 |
  277005.6 |
  281507.9 |
  (5 below, 4 above range)

warm-clamp-head (n=40, range 410817.6-571594.2 ns)
  410817.6 |####
  418856.5 |
  426895.3 |####
  434934.1 |########################################
  442972.9 |##
  451011.8 |
  459050.6 |##
  467089.4 |##
  475128.2 |
  483167.1 |##
  491205.9 |
  499244.7 |
  507283.5 |
  515322.4 |
  523361.2 |
  531400.0 |##
  539438.8 |##
  547477.7 |
  555516.5 |##
  563555.3 |
  (6 below, 3 above range)

warm-clamp-min-lanes (n=40, range 363428.4-513160.0 ns)
  363428.4 |###################################
  370915.0 |
  378401.6 |
  385888.2 |
  393374.7 |
  400861.3 |
  408347.9 |
  415834.5 |
  423321.1 |#########################
  430807.6 |##############################
  438294.2 |####################
  445780.8 |#####
  453267.4 |
  460754.0 |
  468240.5 |
  475727.1 |
  483213.7 |
  490700.3 |
  498186.9 |########################################
  505673.4 |#####
  (3 below, 5 above range)

warm-clamp-minimum (n=40, range 415149.7-487311.2 ns)
  415149.7 |##########################
  418757.8 |#############
  422365.9 |
  425973.9 |
  429582.0 |######
  433190.1 |####################
  436798.2 |##########################
  440406.2 |
  444014.3 |
  447622.4 |######
  451230.5 |########################################
  454838.5 |####################
  458446.6 |#############
  462054.7 |######
  465662.8 |
  469270.8 |
  472878.9 |######
  476487.0 |######
  480095.1 |##########################
  483703.1 |
  (4 below, 3 above range)

```
