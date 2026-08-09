# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1951% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (311.14 us) leads bitpack-write-unsound (6.38 ms) by 1951%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 22.3x slower than the field

bitpack-write-guarded (6.95 ms) is 22.3x the fastest (311.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.73)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.73, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (311.14 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 22.3x the fastest

Fastest bitpack-write-dense (311.14 us) to slowest bitpack-write-guarded (6.95 ms): 22.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 311142.9 ns median
- 2 variants significantly slower than baseline
- Spread: 22.35x (fastest 311142.9 ns, slowest 6953439.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 316824ns | 311478ns | 276965ns | 311812ns | 371720ns | base |
| bitpack-write-guarded | 7287306ns | 6955158ns | 6759075ns | 7073777ns | 8456123ns | +2200.11% |
| bitpack-write-unsound | 6876615ns | 6384520ns | 6291697ns | 6599865ns | 8291781ns | +2070.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 316065ns | 276534ns | 369839ns | base | 6.635 |
| bitpack-write-guarded | 7285012ns | 6757308ns | 8453117ns | +2204.91% | 0.288 |
| bitpack-write-unsound | 6874272ns | 6289677ns | 8288709ns | +2074.95% | 0.305 |

## Performance model

- Peak throughput: **7.584 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 6.740 | 88.9% |
| bitpack-write-guarded | 0.302 | 4.0% |
| bitpack-write-unsound | 0.329 | 4.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 316824ns | 316824ns | base |
| bitpack-write-guarded | 7287306ns | 7287306ns | +2200.11% |
| bitpack-write-unsound | 6876615ns | 6876615ns | +2070.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 311143ns | base | --- | [303001, 318684] | --- | --- | --- | --- |
| bitpack-write-guarded | 6953439ns | +6666046.5ns (+2142.4%) | [+6587163, +6937726]ns | [6897511, 7230017] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 6382456ns | +6079300.8ns (+1953.9%) | [+6017351, +6318411]ns | [6329330, 6625716] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|
| 1 | 273490ns | +2509.3% | +2684.6% |
| 2 | 266733ns | +2495.5% | +2707.4% |
| 3 | 270008ns | +2477.2% | +2915.0% |
| 4 | 283069ns | +2321.0% | +2973.5% |
| 5 | 272630ns | +2400.6% | +2713.7% |
| 6 | 293843ns | +2293.3% | +2461.2% |
| 7 | 300197ns | +2229.2% | +2483.2% |
| 8 | 284842ns | +2339.3% | +2695.2% |
| 9 | 284977ns | +2324.1% | +2183.1% |
| 10 | 292456ns | +2250.2% | +2134.4% |
| 11 | 344609ns | +1840.7% | +2925.8% |
| 12 | 347782ns | +1860.3% | +1708.9% |
| 13 | 336349ns | +1935.0% | +1777.6% |
| 14 | 332032ns | +1940.9% | +1785.8% |
| 15 | 323066ns | +2039.7% | +2123.3% |
| 16 | 307987ns | +2138.0% | +2031.4% |
| 17 | 306622ns | +2131.4% | +2080.8% |
| 18 | 311256ns | +2060.9% | +1974.2% |
| 19 | 331319ns | +1921.9% | +2092.8% |
| 20 | 305805ns | +2089.0% | +1970.2% |
| 21 | 319156ns | +2198.6% | +1882.5% |
| 22 | 329868ns | +2186.8% | +1816.7% |
| 23 | 311261ns | +2294.7% | +1918.1% |
| 24 | 329552ns | +2332.7% | +1801.3% |
| 25 | 310898ns | +2391.5% | +1931.3% |
| 26 | 318213ns | +2356.6% | +1882.5% |
| 27 | 383675ns | +2717.0% | +1558.2% |
| 28 | 408858ns | +1854.9% | +1668.2% |
| 29 | 441375ns | +1995.2% | +1331.8% |
| 30 | 364037ns | +2081.9% | +1634.0% |
| 31 | 311093ns | +2254.2% | +1946.2% |
| 32 | 311909ns | +2107.1% | +1938.9% |
| 33 | 308107ns | +2120.0% | +1966.8% |
| 34 | 298203ns | +2409.5% | +2045.2% |
| 35 | 299819ns | +2277.9% | +2016.9% |
| 36 | 311193ns | +2487.9% | +1933.4% |
| 37 | 313612ns | +2329.8% | +2463.1% |
| 38 | 329793ns | +2016.0% | +1811.0% |
| 39 | 296398ns | +2228.7% | +2074.9% |
| 40 | 276520ns | +2411.7% | +2176.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.731 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.492 | moderate+ |
| bitpack-write-unsound | 0.217 | moderate+ |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40
- **bitpack-write-unsound**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 23.4ns | 316065.2ns | 0.0% |  |
| bitpack-write-guarded | 140.4ns | 7285012.2ns | 0.0% |  |
| bitpack-write-unsound | 156.5ns | 6874271.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 276533.5-369839.5 ns)
  276533.5 |
  281198.8 |#################
  285864.1 |
  290529.4 |###########
  295194.7 |#################
  299860.0 |#####
  304525.3 |######################
  309190.6 |########################################
  313855.9 |#####
  318521.2 |###########
  323186.5 |
  327851.8 |############################
  332517.1 |#####
  337182.4 |
  341847.7 |#####
  346513.0 |#####
  351178.3 |
  355843.6 |
  360508.9 |#####
  365174.2 |
  (5 below, 3 above range)

bitpack-write-guarded (n=40, range 6757307.9-8453117.5 ns)
  6757307.9 |######################
  6842098.4 |########################################
  6926888.9 |######################
  7011679.4 |####
  7096469.8 |########
  7181260.3 |
  7266050.8 |########
  7350841.3 |
  7435631.7 |########
  7520422.2 |####
  7605212.7 |####
  7690003.2 |####
  7774793.7 |####
  7859584.1 |####
  7944374.6 |########
  8029165.1 |####
  8113955.6 |
  8198746.1 |
  8283536.5 |
  8368327.0 |
  (4 below, 2 above range)

bitpack-write-unsound (n=40, range 6289676.9-8288709.1 ns)
  6289676.9 |########################################
  6389628.5 |#######
  6489580.1 |#######
  6589531.7 |##
  6689483.3 |
  6789434.9 |
  6889386.5 |
  6989338.1 |
  7089289.8 |##
  7189241.4 |####
  7289193.0 |
  7389144.6 |##
  7489096.2 |##
  7589047.8 |####
  7688999.4 |##
  7788951.0 |
  7888902.6 |##
  7988854.2 |##
  8088805.9 |##
  8188757.5 |
  (3 below, 2 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.73 (measurement drift or warm-up artifact)
