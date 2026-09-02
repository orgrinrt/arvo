# Layout::Dense footprint: sequential sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-footprint-dense) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-footprint-dense has the worst median (810.24 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-footprint-dense-alt at 805.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.81 us) is smaller than the fastest variant's own run-to-run std-dev (32.87 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bitpack-footprint-dense-alt vs stability leader bitpack-footprint-dense (+1% speed for 1.0x steadier)

bitpack-footprint-dense-alt is fastest (805.43 us, CV 4.1%); bitpack-footprint-dense gives up 0.6% median for 1.0x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 805426.7 ns median (-0.6% vs baseline)
- Spread: 1.01x (fastest 805426.7 ns, slowest 810235.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 814990ns | 811470ns | 783842ns | 809277ns | 863279ns | base |
| bitpack-footprint-dense-alt | 816494ns | 806464ns | 781177ns | 810262ns | 870507ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 813679ns | 782488ns | 862051ns | base | 8.603 |
| bitpack-footprint-dense-alt | 815162ns | 779976ns | 869035ns | +0.18% | 8.587 |

## Performance model

- Peak throughput: **8.975 Gops/s** (bitpack-footprint-dense-alt; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 8.639 | 96.3% |
| bitpack-footprint-dense-alt | 8.691 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 814990ns | 814990ns | base |
| bitpack-footprint-dense-alt | 816494ns | 816494ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 810235ns | base | --- | [794904, 816515] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 805427ns | no significant difference | [-13082, +9802]ns | [794849, 823395] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt |
|---|---|---|
| 1 | 838272ns | -6.2% |
| 2 | 827668ns | -4.3% |
| 3 | 825426ns | -2.6% |
| 4 | 792938ns | +2.0% |
| 5 | 782362ns | +3.3% |
| 6 | 773909ns | +15.6% |
| 7 | 798723ns | +2.4% |
| 8 | 787919ns | +1.2% |
| 9 | 817513ns | +6.8% |
| 10 | 792775ns | +5.0% |
| 11 | 789342ns | +0.9% |
| 12 | 815752ns | -2.0% |
| 13 | 785418ns | -1.6% |
| 14 | 810096ns | -3.4% |
| 15 | 817278ns | -3.2% |
| 16 | 793913ns | -0.8% |
| 17 | 792348ns | +1.5% |
| 18 | 795894ns | -0.9% |
| 19 | 789601ns | -1.6% |
| 20 | 806357ns | -4.0% |
| 21 | 835620ns | +0.5% |
| 22 | 849990ns | -1.0% |
| 23 | 851111ns | +4.6% |
| 24 | 824649ns | -0.2% |
| 25 | 859228ns | -0.2% |
| 26 | 811935ns | +7.2% |
| 27 | 950131ns | -10.8% |
| 28 | 866653ns | -2.2% |
| 29 | 833115ns | +0.5% |
| 30 | 845401ns | +3.0% |
| 31 | 768350ns | +5.3% |
| 32 | 791875ns | +4.3% |
| 33 | 813426ns | +1.3% |
| 34 | 799677ns | -2.3% |
| 35 | 801156ns | -2.2% |
| 36 | 815508ns | -3.2% |
| 37 | 810374ns | -1.7% |
| 38 | 790579ns | -1.1% |
| 39 | 811863ns | -2.3% |
| 40 | 783003ns | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.393 | moderate+ |
| bitpack-footprint-dense-alt | 0.466 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 22/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 53.6ns | 813678.7ns | 0.0% |  |
| bitpack-footprint-dense-alt | 59.9ns | 815161.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 782488.0-862050.9 ns)
  782488.0 |#############
  786466.2 |####################
  790444.3 |########################################
  794422.5 |######
  798400.6 |####################
  802378.7 |
  806356.9 |#############
  810335.0 |##########################
  814313.2 |##########################
  818291.3 |
  822269.4 |#############
  826247.6 |######
  830225.7 |######
  834203.9 |######
  838182.0 |######
  842160.2 |######
  846138.3 |######
  850116.4 |######
  854094.6 |
  858072.7 |######
  (3 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 779976.5-869034.9 ns)
  779976.5 |########################################
  784429.4 |##############################
  788882.3 |##############################
  793335.3 |########################################
  797788.2 |##########
  802241.1 |##############################
  806694.0 |##############################
  811146.9 |
  815599.9 |##########
  820052.8 |####################
  824505.7 |##########
  828958.6 |##########
  833411.5 |##########
  837864.5 |####################
  842317.4 |
  846770.3 |####################
  851223.2 |
  855676.2 |##########
  860129.1 |
  864582.0 |
  (3 below, 5 above range)

```
