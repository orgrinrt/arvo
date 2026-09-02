#!/usr/bin/env python3
"""Test-gate instrument: parse every `#[test]` body in the bench variant tree and
report the ones whose own body contains no assertion or panic.

NEGATIVE CONTROL. A scanner that reports "0 tests with no assert" is
indistinguishable from a scanner that found no tests at all, or from one whose
body extraction is broken and always returns the whole file. So this script
plants two synthetic cases in a temp tree and REQUIRES that it find exactly the
one that has no assertion and NOT the one that has. If the control fails the
scan result is not printed at all.
"""
import os, sys, tempfile

def parse(text):
    """Yield (line_no, fn_signature, body_text) for every #[test] in `text`."""
    lines = text.split('\n')
    out, i = [], 0
    while i < len(lines):
        if lines[i].strip().startswith('#[test]'):
            j = i + 1
            while j < len(lines) and not lines[j].strip().startswith('fn '):
                j += 1
            if j >= len(lines):
                break
            depth, started, body, k = 0, False, [], j
            while k < len(lines):
                depth += lines[k].count('{') - lines[k].count('}')
                body.append(lines[k])
                if '{' in lines[k]:
                    started = True
                if started and depth <= 0:
                    break
                k += 1
            # The signature line is EXCLUDED from the body. A test named
            # `..._no_assert` contains the substring "assert" in its own name,
            # so scanning the signature makes such a test invisible to this
            # instrument. The negative control below is what found that; the
            # first version of this script scored 18 bare tests in the real tree
            # with the bug live, and the control refused to let the number print.
            out.append((j + 1, lines[j].strip(), '\n'.join(body[1:])))
            i = k
        else:
            i += 1
    return out

CONTROL = '''
#[cfg(test)]
mod t {
    #[test]
    fn control_has_an_assert() {
        let x = 2 + 2;
        assert_eq!(x, 4, "arithmetic");
    }

    #[test]
    fn control_has_no_assert() {
        let _x = 2 + 2;
    }
}
'''

def run_control():
    found = parse(CONTROL)
    names = [s for (_, s, _) in found]
    if len(found) != 2:
        return False, f"control: expected 2 tests, parsed {len(found)}: {names}"
    bare = [s for (_, s, b) in found if 'assert' not in b and 'panic' not in b]
    if bare != ['fn control_has_no_assert() {']:
        return False, f"control: expected exactly the assert-free test, got {bare}"
    # And the harder half: the parser must not swallow the whole module.
    withassert = [b for (_, s, b) in found if s.startswith('fn control_has_an_assert')][0]
    if 'control_has_no_assert' in withassert:
        return False, "control: body extraction ran past the end of the first fn"
    return True, "control: ok (2 parsed, 1 bare, no run-on)"

def main(root):
    ok, msg = run_control()
    print(msg)
    if not ok:
        print("NEGATIVE CONTROL FAILED -- scan result suppressed")
        sys.exit(1)
    total, bare = 0, []
    for dirpath, _, files in os.walk(root):
        for f in files:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(dirpath, f)
            text = open(p, encoding='utf-8', errors='replace').read()
            if '#[test]' not in text:
                continue
            for (ln, sig, body) in parse(text):
                total += 1
                if 'assert' not in body and 'panic' not in body:
                    bare.append((p, ln, sig))
    print(f"root: {root}")
    print(f"tests parsed: {total}")
    print(f"tests whose own body has no assert/panic: {len(bare)}")
    for (p, ln, sig) in sorted(bare):
        print(f"  {p}:{ln}  {sig}")

if __name__ == '__main__':
    main(sys.argv[1] if len(sys.argv) > 1 else '.')
