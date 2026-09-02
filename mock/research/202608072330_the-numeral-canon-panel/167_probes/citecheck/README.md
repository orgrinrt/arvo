# Citation check for 167

```sh
python3 mock/research/202608072330_the-numeral-canon-panel/167_probes/citecheck/check.py
```

Opens every citation `167` makes and tests its **content** at the cited location, not merely that the
file resolves. Output in `check.out`.

**Result: 27 citations, 0 failures, 2 of 2 negative controls caught.**

The two negative controls are what makes the zero mean anything: a citation naming a file that does not
exist, and a citation naming a real file at a real line with text that is not there. Both are caught, so
the checker can fail.

## One real finding about the instrument, from running it

Two of my own citations failed on the first run and **both turned out to be accurate**. The quoted
phrases spanned a line break in the source, so a substring match against the raw text could not find
them:

```
83:schedule is also the index set of the error analysis. Fewer adaptation points is not merely
28-something must bring it back, and that something, the adaptation, must be placed somewhere. **A
29:chain is a composition of exact operations together with a schedule of adaptation points.**
```

A quotation is a claim about words rather than about where the source happened to wrap, so the checker
now normalises whitespace on both sides before matching. **A checker that reports a false failure on a
wrapped quotation trains its user to dismiss its failures**, which is worse than not having one. The
negative controls were rerun after the change and still fire, so the normalisation did not disable the
instrument.
