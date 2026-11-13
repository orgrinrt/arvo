# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 65% faster than the next best (bitpack-carrier-d16)

bitpack-carrier-packed (178.01 us) leads bitpack-carrier-d16 (294.48 us) by 65%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 38% (significant)

bitpack-carrier-packed is -112.31 us (38%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d64 is an outlier: 2.4x slower than the field

bitpack-carrier-d64 (426.08 us) is 2.4x the fastest (178.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-carrier-packed is fastest but the noisiest (CV 15.5%)

bitpack-carrier-packed wins on median (178.01 us) yet has the highest variance (CV 15.5%), while bitpack-carrier-d32 is the steadiest (CV 6.9%, 306.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: bitpack-carrier-packed** at 178008.6 ns median (-39.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.39x (fastest 178008.6 ns, slowest 426075.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 299298ns | 295319ns | 282452ns | 295578ns | 327306ns | base |
| bitpack-carrier-d16-control | 306717ns | 303501ns | 276279ns | 302895ns | 348622ns | +2.48% |
| bitpack-carrier-d32 | 304035ns | 307844ns | 274746ns | 303997ns | 333439ns | +1.58% |
| bitpack-carrier-d64 | 438477ns | 428280ns | 387667ns | 436204ns | 496104ns | +46.50% |
| bitpack-carrier-packed | 182369ns | 178462ns | 150573ns | 179356ns | 223200ns | -39.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 298472ns | 281814ns | 326311ns | base | 3.513 |
| bitpack-carrier-d16-control | 305914ns | 275703ns | 347542ns | +2.49% | 3.428 |
| bitpack-carrier-d32 | 302711ns | 273656ns | 331780ns | +1.42% | 3.464 |
| bitpack-carrier-d64 | 436570ns | 386358ns | 493622ns | +46.27% | 2.402 |
| bitpack-carrier-packed | 181858ns | 150124ns | 222454ns | -39.07% | 5.766 |

## Performance model

- Peak throughput: **6.985 Gops/s** (bitpack-carrier-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 3.561 | 51.0% |
| bitpack-carrier-d16-control | 3.464 | 49.6% |
| bitpack-carrier-d32 | 3.421 | 49.0% |
| bitpack-carrier-d64 | 2.461 | 35.2% |
| bitpack-carrier-packed | 5.891 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 299298ns | 299298ns | base |
| bitpack-carrier-d16-control | 306717ns | 306717ns | +2.48% |
| bitpack-carrier-d32 | 304035ns | 304035ns | +1.58% |
| bitpack-carrier-d64 | 438477ns | 438477ns | +46.50% |
| bitpack-carrier-packed | 182369ns | 182369ns | -39.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 294480ns | base | --- | [289449, 298435] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 302676ns | no significant difference | [-2687, +14321]ns | [298152, 306149] | no | 0.2051 | 0.1539 | 0 |
| bitpack-carrier-d32 | 306468ns | no significant difference | [-4753, +18051]ns | [290273, 312673] | no | 0.2682 | 0.2682 | 0 |
| bitpack-carrier-d64 | 426075ns | +133826.9ns (+45.4%) | [+127798, +145222]ns | [419823, 448973] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 178009ns | -112305.6ns (-38.1%) | [-125584, -106227]ns | [176304, 183412] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 289403ns | +3.3% | +7.2% | +45.1% | -35.5% |
| 1 | 2 | 288429ns | +6.5% | +8.2% | +34.0% | -38.3% |
| 1 | 3 | 302105ns | +0.5% | -6.5% | +49.8% | -50.2% |
| 1 | 4 | 295992ns | -5.8% | -3.3% | +58.6% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.028 | ok |
| bitpack-carrier-d16-control | -0.076 | ok |
| bitpack-carrier-d32 | 0.186 | ok |
| bitpack-carrier-d64 | 0.097 | ok |
| bitpack-carrier-packed | -0.203 | moderate- |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 3/4
- **bitpack-carrier-d32**: won 2/4, lost 2/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 216.1ns | 298472.1ns | 0.1% |  |
| bitpack-carrier-d16-control | 210.1ns | 305913.8ns | 0.1% |  |
| bitpack-carrier-d32 | 331.3ns | 302711.3ns | 0.1% |  |
| bitpack-carrier-d64 | 539.6ns | 436570.1ns | 0.1% |  |
| bitpack-carrier-packed | 138.4ns | 181858.5ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 281814.1-326310.7 ns)
  281814.1 |########
  284038.9 |################
  286263.7 |########################################
  288488.5 |########################################
  290713.4 |################
  292938.2 |################################
  295163.0 |################
  297387.9 |########################
  299612.7 |########
  301837.5 |################################
  304062.4 |
  306287.2 |
  308512.0 |################
  310736.8 |########################
  312961.7 |################
  315186.5 |
  317411.3 |
  319636.2 |
  321861.0 |
  324085.8 |
  (3 below, 1 above range)

bitpack-carrier-d16-control (n=40, range 275702.7-347542.0 ns)
  275702.7 |#####
  279294.7 |###########
  282886.7 |
  286478.6 |#####
  290070.6 |
  293662.6 |############################
  297254.5 |######################
  300846.5 |########################################
  304438.5 |############################
  308030.4 |#################
  311622.4 |#####
  315214.3 |###########
  318806.3 |
  322398.3 |
  325990.2 |
  329582.2 |
  333174.2 |#####
  336766.1 |
  340358.1 |
  343950.1 |
  (4 below, 4 above range)

bitpack-carrier-d32 (n=40, range 273656.4-331780.0 ns)
  273656.4 |########################################
  276562.6 |########
  279468.8 |
  282375.0 |################
  285281.1 |########
  288187.3 |########################
  291093.5 |
  293999.7 |########
  296905.9 |
  299812.0 |################
  302718.2 |########
  305624.4 |################################
  308530.6 |################
  311436.8 |################
  314342.9 |########
  317249.1 |################################
  320155.3 |########################
  323061.5 |########
  325967.6 |
  328873.8 |
  (3 below, 4 above range)

bitpack-carrier-d64 (n=40, range 386357.6-493622.2 ns)
  386357.6 |##########
  391720.9 |
  397084.1 |
  402447.3 |#####
  407810.6 |
  413173.8 |#####
  418537.0 |########################################
  423900.2 |####################
  429263.5 |#####
  434626.7 |
  439989.9 |
  445353.2 |#########################
  450716.4 |####################
  456079.6 |#####
  461442.8 |
  466806.1 |
  472169.3 |
  477532.5 |##########
  482895.8 |##########
  488259.0 |#####
  (5 below, 3 above range)

bitpack-carrier-packed (n=40, range 150124.2-222454.4 ns)
  150124.2 |###
  153740.7 |###
  157357.2 |
  160973.7 |###
  164590.2 |######
  168206.7 |
  171823.3 |
  175439.8 |########################################
  179056.3 |#########
  182672.8 |############
  186289.3 |############
  189905.8 |###
  193522.3 |###
  197138.8 |
  200755.4 |
  204371.9 |
  207988.4 |
  211604.9 |
  215221.4 |
  218837.9 |
  (5 below, 4 above range)

```
