# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 349% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (180.09 us) leads bitpack-footprint-packed-naive (807.98 us) by 349%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 8.7%)

bitpack-footprint-packed wins on median (180.09 us) yet has the highest variance (CV 8.7%), while bitpack-footprint-packed-naive is the steadiest (CV 2.8%, 807.98 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (180.09 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.5x the fastest

Fastest bitpack-footprint-packed (180.09 us) to slowest bitpack-footprint-packed-naive (807.98 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 180089.6 ns median
- 1 variant significantly slower than baseline
- Spread: 4.49x (fastest 180089.6 ns, slowest 807976.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 186987ns | 180499ns | 175115ns | 181899ns | 214122ns | base |
| bitpack-footprint-packed-naive | 810243ns | 809118ns | 781986ns | 808340ns | 844208ns | +333.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 186638ns | 174827ns | 213654ns | base | 5.618 |
| bitpack-footprint-packed-naive | 809491ns | 781249ns | 843426ns | +333.72% | 1.295 |

## Performance model

- Peak throughput: **5.998 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 5.823 | 97.1% |
| bitpack-footprint-packed-naive | 1.298 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 186987ns | 186987ns | base |
| bitpack-footprint-packed-naive | 810243ns | 810243ns | +333.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 180090ns | base | --- | [177265, 185314] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 807977ns | +621570.4ns (+345.1%) | [+616941, +625654]ns | [798643, 815064] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|
| 1 | 177526ns | +364.5% |
| 2 | 177004ns | +368.3% |
| 3 | 217205ns | +287.3% |
| 4 | 249316ns | +212.8% |
| 5 | 210339ns | +289.7% |
| 6 | 179909ns | +341.8% |
| 7 | 188682ns | +318.0% |
| 8 | 176918ns | +383.8% |
| 9 | 186153ns | +341.8% |
| 10 | 177528ns | +394.5% |
| 11 | 176343ns | +379.2% |
| 12 | 175319ns | +352.9% |
| 13 | 171278ns | +374.5% |
| 14 | 184475ns | +339.3% |
| 15 | 175323ns | +360.1% |
| 16 | 175633ns | +352.1% |
| 17 | 182550ns | +338.8% |
| 18 | 178507ns | +345.0% |
| 19 | 175130ns | +342.8% |
| 20 | 175125ns | +352.9% |
| 21 | 175182ns | +355.1% |
| 22 | 175628ns | +356.1% |
| 23 | 176340ns | +368.4% |
| 24 | 199509ns | +306.9% |
| 25 | 176534ns | +353.2% |
| 26 | 181922ns | +359.3% |
| 27 | 213478ns | +280.1% |
| 28 | 214569ns | +263.7% |
| 29 | 179335ns | +331.1% |
| 30 | 180270ns | +361.5% |
| 31 | 175908ns | +344.2% |
| 32 | 194308ns | +302.0% |
| 33 | 182019ns | +341.2% |
| 34 | 206018ns | +297.4% |
| 35 | 179519ns | +349.8% |
| 36 | 198798ns | +311.2% |
| 37 | 187219ns | +333.5% |
| 38 | 186186ns | +324.2% |
| 39 | 183975ns | +339.4% |
| 40 | 188527ns | +340.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.418 | moderate+ |
| bitpack-footprint-packed-naive | 0.194 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 17.7ns | 186637.7ns | 0.0% |  |
| bitpack-footprint-packed-naive | 39.0ns | 809490.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 174827.3-213654.0 ns)
  174827.3 |########################################
  176768.7 |##################
  178710.0 |##############
  180651.3 |##########
  182592.7 |#######
  184534.0 |#######
  186475.3 |###
  188416.6 |#######
  190358.0 |
  192299.3 |
  194240.6 |###
  196182.0 |
  198123.3 |#######
  200064.6 |
  202006.0 |
  203947.3 |
  205888.6 |###
  207830.0 |
  209771.3 |###
  211712.6 |###
  (1 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 781248.9-843426.4 ns)
  781248.9 |##########
  784357.8 |
  787466.7 |####################
  790575.6 |##########
  793684.4 |########################################
  796793.3 |##########
  799902.2 |##############################
  803011.0 |##########
  806119.9 |##############################
  809228.8 |########################################
  812337.7 |##########
  815446.5 |##########
  818555.4 |####################
  821664.3 |####################
  824773.1 |##########
  827882.0 |####################
  830990.9 |##########
  834099.7 |##########
  837208.6 |
  840317.5 |##########
  (5 below, 3 above range)

```
