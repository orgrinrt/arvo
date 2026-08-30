#!/usr/bin/env python3
"""t5. The C1 perimeter instrument, built rather than described.

Clause one of the acceptance criterion says the consumer expresses usage in bits and
bytes, and SETTLED.md:93 ratifies that the container is never written by a consumer.
Together they are a claim about the whole observation surface of the numeral type, and a
claim of that shape holds only over the operations through which the type can be
observed. A single public constructor taking the container, a single From impl, a single
public field on a neighbouring type, and the claim holds only up to that hole.

Nothing in this panel enumerates that surface. This does. It walks rustdoc's JSON for a
crate and reports every consumer-reachable position whose type is a container.

  rustdoc +nightly-2026-05-28 --edition 2021 -Z unstable-options \
    --output-format json --out-dir json t5_perimeter.rs
  python3 t5_perimeter.py json/t5_perimeter.json

The rustdoc JSON format is unstable and is a TOOL surface. Nothing the design ships
depends on it, so it is not a feature-gate question; it is the same kind of dependency as
depending on rustc's diagnostics being readable.

Spike. Presume it flawed. It follows type aliases, because the first version did not and
a one-line alias evaded it; that failure is preserved as the `alias/` variant. It still
does not resolve associated-type projections or transitive Deref targets, and the "known
gaps" section prints those rather than hiding them.
"""
import json
import sys

# The containers the derivation may pick. A consumer-facing position holding one of these
# is a hole in clause one, because it is a position through which the container is
# written or read.
CONTAINERS = {
    "u8", "u16", "u32", "u64", "u128",
    "i8", "i16", "i32", "i64", "i128",
}

# Positions that are legitimately primitive-shaped. These are the documented exceptions,
# and listing them here is what makes the rest of the report meaningful: an instrument
# with no exception list flags everything and gets switched off.
ALLOWED_RETURN_PRIMITIVES = {"usize", "bool"}


# Alias resolution, filled in main() before any walk. A container reached through a type
# alias is the same hole as one written directly, and rustdoc records the alias target in
# the same index, so following it costs one dictionary.
ALIASES = {}


def walk_type(t, out, path, depth=0):
    """Collect (primitive-name, path) for every primitive appearing in a type."""
    if t is None or depth > 16:
        return
    if not isinstance(t, dict):
        return
    for k, v in t.items():
        if k == "primitive":
            out.append((v, path))
        elif k == "borrowed_ref":
            walk_type(v.get("type"), out, path + "&", depth + 1)
        elif k == "slice":
            walk_type(v, out, path + "[]", depth + 1)
        elif k == "array":
            walk_type(v.get("type"), out, path + "[N]", depth + 1)
        elif k == "raw_pointer":
            walk_type(v.get("type"), out, path + "*", depth + 1)
        elif k == "tuple":
            for i, e in enumerate(v or []):
                walk_type(e, out, path + f".{i}", depth + 1)
        elif k == "resolved_path":
            tid = str(v.get("id"))
            if tid in ALIASES:
                walk_type(ALIASES[tid], out, path + f" via alias {v.get('path')}", depth + 1)
            args = (v.get("args") or {}).get("angle_bracketed", {})
            for a in args.get("args", []) or []:
                if isinstance(a, dict) and "type" in a:
                    walk_type(a["type"], out, path + f"<{v.get('path')}>", depth + 1)
        elif k == "qualified_path":
            walk_type(v.get("self_type"), out, path + "::", depth + 1)


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "json/t5_perimeter.json"
    d = json.load(open(path))
    idx = d["index"]
    for iid, item in idx.items():
        inner = item.get("inner") or {}
        if "type_alias" in inner:
            ALIASES[str(iid)] = inner["type_alias"].get("type")

    def local(item):
        return item.get("crate_id", 1) == 0

    findings = []
    surface = []

    for iid, item in idx.items():
        if not local(item):
            continue
        inner = item.get("inner") or {}
        name = item.get("name")

        if "function" in inner:
            sig = inner["function"]["sig"]
            for argname, ty in sig.get("inputs", []):
                prims = []
                walk_type(ty, prims, "")
                surface.append(("fn-arg", f"{name}({argname})"))
                for p, where in prims:
                    if p in CONTAINERS:
                        findings.append(("fn argument", f"{name}({argname}{where})", p))
            prims = []
            walk_type(sig.get("output"), prims, "")
            surface.append(("fn-ret", f"{name}() ->"))
            for p, where in prims:
                if p in CONTAINERS:
                    findings.append(("fn return", f"{name}(){where}", p))

        if "struct_field" in inner:
            prims = []
            walk_type(inner["struct_field"], prims, "")
            surface.append(("pub-field", str(name)))
            for p, where in prims:
                if p in CONTAINERS:
                    findings.append(("public field", f"{name}{where}", p))

        if "assoc_type" in inner:
            surface.append(("assoc-type", str(name)))

    print("t5. the C1 perimeter: every public observation, and which admit a container")
    print(f"    crate JSON : {path}")
    print(f"    containers the derivation may pick: {' '.join(sorted(CONTAINERS))}")
    print()
    print(f"    public positions enumerated : {len(surface)}")
    kinds = {}
    for k, _ in surface:
        kinds[k] = kinds.get(k, 0) + 1
    for k in sorted(kinds):
        print(f"      {k:12} {kinds[k]}")
    print()

    if findings:
        print(f"    HOLES IN CLAUSE ONE: {len(findings)}")
        print()
        print(f"      {'kind':16}{'position':34}{'container'}")
        print("      " + "-" * 58)
        for kind, pos, p in sorted(findings):
            print(f"      {kind:16}{pos:34}{p}")
    else:
        print("    no public position admits a container.")

    print()
    print(f"    type aliases resolved: {len(ALIASES)}")
    print()
    print("    known gaps in this instrument, printed rather than assumed:")
    print("      - associated-type projections are not resolved, so a container reached")
    print("        through <T as Trait>::Assoc is a false negative")
    print("      - Deref targets are not followed transitively")
    print("      - trait impls from other crates are excluded by the crate_id filter,")
    print("        which is right for authorship and wrong for reachability")
    print()
    print("    So a clean report from this instrument is necessary and not sufficient.")
    print("    A dirty one is decisive: every row above is a position a consumer can")
    print("    reach, holding a type the criterion says a consumer never writes.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
