# Wide rung, bare column walk, 458752 elements (1 wide op/element, past L2)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (308.64 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 289.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (wide-rung-ragged, wide-rung-wordround-alias) are a dead heat (<1%)

wide-rung-ragged (289.57 us) and wide-rung-wordround-alias (289.67 us) differ by 0.03%, inside the noise, even though the wider field spreads 6.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged-overread shows warm-up / thermal drift (autocorr +0.56)

wide-rung-ragged-overread's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader wide-rung-ragged vs stability leader wide-rung-wordround-alias (+0% speed for 2.7x steadier)

wide-rung-ragged is fastest (289.57 us, CV 3.7%); wide-rung-wordround-alias gives up 0.0% median for 2.7x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: wide-rung-ragged** at 289569.8 ns median (-6.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.07x (fastest 289569.8 ns, slowest 308644.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 313636ns | 309156ns | 300807ns | 310684ns | 335318ns | base |
| wide-rung-ragged | 294835ns | 289862ns | 286557ns | 291631ns | 312726ns | -5.99% |
| wide-rung-ragged-overread | 296471ns | 291986ns | 286722ns | 294082ns | 313387ns | -5.47% |
| wide-rung-wordround | 301565ns | 299872ns | 288594ns | 298854ns | 322670ns | -3.85% |
| wide-rung-wordround-alias | 291965ns | 290985ns | 286975ns | 291621ns | 297987ns | -6.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 312483ns | 299152ns | 334360ns | base | 1.468 |
| wide-rung-ragged | 294287ns | 285961ns | 312064ns | -5.82% | 1.559 |
| wide-rung-ragged-overread | 295721ns | 285031ns | 312933ns | -5.36% | 1.551 |
| wide-rung-wordround | 300713ns | 287498ns | 322047ns | -3.77% | 1.526 |
| wide-rung-wordround-alias | 290897ns | 286424ns | 297118ns | -6.91% | 1.577 |

## Performance model

- Peak throughput: **1.609 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 458752

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.486 | 92.3% |
| wide-rung-ragged | 1.584 | 98.4% |
| wide-rung-ragged-overread | 1.575 | 97.9% |
| wide-rung-wordround | 1.534 | 95.3% |
| wide-rung-wordround-alias | 1.584 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 313636ns | 313636ns | base |
| wide-rung-ragged | 294835ns | 294835ns | -5.99% |
| wide-rung-ragged-overread | 296471ns | 296471ns | -5.47% |
| wide-rung-wordround | 301565ns | 301565ns | -3.85% |
| wide-rung-wordround-alias | 291965ns | 291965ns | -6.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 308644ns | base | --- | [303712, 313788] | --- | --- | --- | --- |
| wide-rung-ragged | 289570ns | -18382.5ns (-6.0%) | [-23092, -13506]ns | [287061, 294810] | YES | 0.0001 | 0.0000 | 0 |
| wide-rung-ragged-overread | 291208ns | -11270.4ns (-3.7%) | [-20422, -7366]ns | [289715, 298596] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 299113ns | -9651.9ns (-3.1%) | [-15028, -2531]ns | [292375, 302304] | YES | 0.0022 | 0.0022 | 0 |
| wide-rung-wordround-alias | 289669ns | -18503.5ns (-6.0%) | [-24229, -13073]ns | [288199, 292220] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 345902ns | -15.7% | -13.8% | -16.7% | -17.7% |
| 2 | 321230ns | -7.7% | -6.9% | -10.4% | -10.4% |
| 3 | 300297ns | -4.7% | -2.9% | -3.9% | -0.1% |
| 4 | 299725ns | -4.6% | -2.4% | -0.8% | -1.2% |
| 5 | 301025ns | -4.9% | -4.6% | +4.8% | -1.3% |
| 6 | 310855ns | -8.0% | -7.3% | +20.6% | -7.3% |
| 7 | 320027ns | -10.7% | -10.1% | +0.5% | -10.2% |
| 8 | 305656ns | -5.9% | +2.7% | +2.6% | -6.0% |
| 9 | 301218ns | -4.8% | +6.2% | +4.1% | -3.6% |
| 10 | 301813ns | -5.1% | -3.6% | +1.3% | -3.0% |
| 11 | 325337ns | -9.9% | +0.1% | -7.9% | -12.0% |
| 12 | 313070ns | -7.6% | -2.3% | -2.9% | -8.6% |
| 13 | 309230ns | -3.8% | +0.2% | -0.0% | -7.4% |
| 14 | 308163ns | -6.2% | -0.4% | -0.8% | -5.3% |
| 15 | 309338ns | -6.5% | -0.0% | -2.2% | -5.1% |
| 16 | 324056ns | -11.2% | -6.4% | -7.5% | -11.7% |
| 17 | 305972ns | -2.8% | -1.1% | -2.1% | -4.8% |
| 18 | 302465ns | +6.8% | -0.2% | -0.8% | -2.4% |
| 19 | 314505ns | -7.1% | -0.9% | -4.0% | -5.9% |
| 20 | 319957ns | -9.4% | -5.2% | -4.1% | -8.1% |
| 21 | 296842ns | +1.1% | +0.8% | +0.6% | -0.8% |
| 22 | 301520ns | -3.2% | -3.8% | -3.5% | -4.3% |
| 23 | 301156ns | +8.5% | -3.4% | -3.4% | -4.2% |
| 24 | 299732ns | +7.4% | -3.4% | -2.9% | -3.8% |
| 25 | 298768ns | +2.0% | -1.4% | -2.0% | -3.6% |
| 26 | 298388ns | +1.0% | -1.4% | +0.9% | -3.6% |
| 27 | 300163ns | +0.5% | -3.7% | +1.0% | -2.3% |
| 28 | 305842ns | -3.0% | -5.1% | -0.3% | -5.9% |
| 29 | 307204ns | -0.9% | -5.3% | -5.0% | -6.2% |
| 30 | 320505ns | -6.0% | -9.6% | -9.0% | -10.4% |
| 31 | 299297ns | -4.4% | -2.5% | -4.9% | -1.6% |
| 32 | 304959ns | -6.1% | -6.5% | -6.5% | -4.4% |
| 33 | 324762ns | -11.9% | -12.1% | -8.3% | -8.6% |
| 34 | 309125ns | -7.2% | -7.8% | +3.9% | -6.1% |
| 35 | 323962ns | -11.3% | -12.0% | -9.3% | -7.5% |
| 36 | 331492ns | -13.8% | -12.9% | -13.1% | -10.7% |
| 37 | 330134ns | -13.3% | -13.4% | -10.0% | -12.3% |
| 38 | 369232ns | -22.6% | -22.9% | -21.9% | -21.6% |
| 39 | 323933ns | -10.4% | -12.2% | -10.6% | -10.5% |
| 40 | 312465ns | -0.1% | -8.9% | -7.1% | -7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.410 | moderate+ |
| wide-rung-ragged | 0.464 | moderate+ |
| wide-rung-ragged-overread | 0.559 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.491 | moderate+ |
| wide-rung-wordround-alias | 0.318 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 32/40, lost 7/40
- **wide-rung-ragged-overread**: won 34/40, lost 5/40
- **wide-rung-wordround**: won 29/40, lost 10/40
- **wide-rung-wordround-alias**: won 39/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 5.4ns | 312483.1ns | 0.0% |  |
| wide-rung-ragged | 6.1ns | 294286.6ns | 0.0% |  |
| wide-rung-ragged-overread | 5.4ns | 295720.8ns | 0.0% |  |
| wide-rung-wordround | 7.2ns | 300713.2ns | 0.0% |  |
| wide-rung-wordround-alias | 4.2ns | 290897.5ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 299151.7-334359.8 ns)
  299151.7 |#################################
  300912.1 |########################################
  302672.5 |
  304432.9 |##########################
  306193.3 |######
  307953.7 |##########################
  309714.1 |######
  311474.5 |#############
  313234.9 |######
  314995.3 |
  316755.7 |
  318516.1 |#############
  320276.5 |#############
  322037.0 |
  323797.4 |#################################
  325557.8 |
  327318.2 |
  329078.6 |######
  330839.0 |######
  332599.4 |
  (3 below, 2 above range)

wide-rung-ragged (n=40, range 285961.0-312063.7 ns)
  285961.0 |########################################
  287266.1 |#######
  288571.3 |##########
  289876.4 |#######
  291181.5 |##########
  292486.7 |###
  293791.8 |
  295096.9 |
  296402.1 |##############
  297707.2 |
  299012.3 |###
  300317.5 |#######
  301622.6 |###
  302927.7 |
  304232.9 |#######
  305538.0 |
  306843.1 |
  308148.3 |
  309453.4 |
  310758.5 |
  (4 below, 4 above range)

wide-rung-ragged-overread (n=40, range 285030.8-312933.1 ns)
  285030.8 |########################
  286425.9 |################
  287821.1 |########################
  289216.2 |################################
  290611.3 |########################################
  292006.4 |########
  293401.5 |################
  294796.6 |
  296191.8 |
  297586.9 |########
  298982.0 |################
  300377.1 |
  301772.2 |########################
  303167.3 |########
  304562.5 |
  305957.6 |################
  307352.7 |
  308747.8 |################
  310142.9 |
  311538.0 |########
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 287498.0-322047.0 ns)
  287498.0 |########################################
  289225.4 |################################
  290952.9 |########################
  292680.3 |################
  294407.8 |
  296135.2 |########################
  297862.7 |################
  299590.1 |################################
  301317.6 |################
  303045.0 |################
  304772.5 |########################
  306499.9 |########
  308227.4 |########
  309954.8 |
  311682.3 |
  313409.7 |################
  315137.2 |########
  316864.6 |
  318592.1 |
  320319.5 |################
  (2 below, 1 above range)

wide-rung-wordround-alias (n=40, range 286424.4-297118.3 ns)
  286424.4 |##########
  286959.1 |##############################
  287493.8 |########################################
  288028.4 |########################################
  288563.1 |####################
  289097.8 |####################
  289632.5 |##########
  290167.2 |####################
  290701.9 |
  291236.6 |##############################
  291771.3 |
  292306.0 |##########
  292840.7 |##########
  293375.4 |##########
  293910.1 |####################
  294444.8 |##########
  294979.5 |##########
  295514.2 |####################
  296048.9 |##########
  296583.6 |##########
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: autocorrelation=0.56 (measurement drift or warm-up artifact)
