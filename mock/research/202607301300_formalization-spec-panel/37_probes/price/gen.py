#!/usr/bin/env python3
"""Generate the two law-shapes at N compositions.

Shape A: file 33's atom list as separate derived marker traits. Nine views need
five markers (value agreement, cause presence, cause exact, event presence,
event exact), so a composition carries up to five impls and a consumer states
the conjunction it needs as trait bounds.

Shape B: one const fn per law returning the finest view, one const item per
composition asserting what that consumer needs.

Both do the same job: establish, for each of N compositions, that the fold law
holds at the detail a consumer requires. Every composition is distinct, so
neither shape gets a trait-selection or const-eval cache hit; these are
worst-case numbers.
"""
import sys, random

def shape_a(n, seed=7):
    rnd = random.Random(seed)
    out = ["pub struct Add;",
           "pub trait ValueAgree<Op> {}",
           "pub trait CausePresence<Op> {}",
           "pub trait CauseExact<Op> {}",
           "pub trait EventPresence<Op> {}",
           "pub trait EventExact<Op> {}",
           "pub fn need_weak<T: ValueAgree<Add>>() {}",
           "pub fn need_kleene<T: ValueAgree<Add> + CausePresence<Add>>() {}",
           "pub fn need_modulo_refusal<T: ValueAgree<Add> + EventExact<Add>>() {}",
           "pub fn need_graded<T: ValueAgree<Add> + CauseExact<Add> + CausePresence<Add> + EventExact<Add> + EventPresence<Add>>() {}",
           ]
    calls = []
    for i in range(n):
        out.append(f"pub struct C{i};")
        kind = rnd.randrange(4)
        out.append(f"impl ValueAgree<Add> for C{i} {{}}")
        if kind == 0:      # graded
            out += [f"impl CausePresence<Add> for C{i} {{}}", f"impl CauseExact<Add> for C{i} {{}}",
                    f"impl EventPresence<Add> for C{i} {{}}", f"impl EventExact<Add> for C{i} {{}}"]
            calls.append(f"    need_graded::<C{i}>();")
        elif kind == 1:    # kleene
            out += [f"impl CausePresence<Add> for C{i} {{}}", f"impl CauseExact<Add> for C{i} {{}}"]
            calls.append(f"    need_kleene::<C{i}>();")
        elif kind == 2:    # modulo refusal
            out += [f"impl EventPresence<Add> for C{i} {{}}", f"impl EventExact<Add> for C{i} {{}}"]
            calls.append(f"    need_modulo_refusal::<C{i}>();")
        else:              # weak only
            calls.append(f"    need_weak::<C{i}>();")
    out.append("pub fn use_all() {")
    out += calls
    out.append("}")
    return "\n".join(out) + "\n"

PRELUDE_B = '''
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Detail { Ignore = 0, Presence = 1, Exact = 2 }
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct View { pub causes: Detail, pub events: Detail }
impl View {
    pub const fn new(c: Detail, e: Detail) -> Self { View { causes: c, events: e } }
    pub const WEAK: View = View::new(Detail::Ignore, Detail::Ignore);
    pub const KLEENE: View = View::new(Detail::Presence, Detail::Ignore);
    pub const GRADED: View = View::new(Detail::Exact, Detail::Exact);
    pub const MODULO_REFUSAL: View = View::new(Detail::Ignore, Detail::Exact);
    pub const fn at_least(self, o: View) -> bool {
        (self.causes as u8) >= (o.causes as u8) && (self.events as u8) >= (o.events as u8)
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LawView { Never, Finest(View) }
pub const fn add_assoc_view(top: u8, bot: u8, dom: u8, arity: u32, headroom: u32) -> LawView {
    if headroom + 1 >= arity { return LawView::Finest(View::GRADED); }
    if top == 2 && bot == 2 {
        if dom == 0 { LawView::Finest(View::GRADED) }
        else { LawView::Finest(View::new(Detail::Exact, Detail::Ignore)) }
    } else if top == 0 && bot == 0 {
        LawView::Finest(View::MODULO_REFUSAL)
    } else if (top == 0 && bot == 2) || (top == 2 && bot == 0) {
        LawView::Finest(View::WEAK)
    } else { LawView::Never }
}
pub const fn licensed(l: LawView, req: View) -> bool {
    match l { LawView::Never => false, LawView::Finest(v) => v.at_least(req) }
}
'''

def shape_b(n, seed=7):
    rnd = random.Random(seed)
    out = [PRELUDE_B]
    for i in range(n):
        kind = rnd.randrange(4)
        if kind == 0:   top, bot, dom, hr, req = 2, 2, 0, 0, "View::GRADED"
        elif kind == 1: top, bot, dom, hr, req = 2, 2, 1, 0, "View::KLEENE"
        elif kind == 2: top, bot, dom, hr, req = 0, 0, 1, 0, "View::MODULO_REFUSAL"
        else:           top, bot, dom, hr, req = 0, 2, 1, 0, "View::WEAK"
        out.append(f"pub const V{i}: bool = licensed(add_assoc_view({top}, {bot}, {dom}, {4 + i % 3}, {hr}), {req});")
        out.append(f"const _: () = assert!(V{i});")
    return "\n".join(out) + "\n"


def shape_c(n, seed=7):
    """The shape D51 actually requires: markers DERIVED, not asserted.

    A marker trait impl is a claim about a composition. D51 rules that law
    markers are derived by blanket impl and that a derived property cannot lie
    (`33:428-430`). Shape A as generated above asserts them: nothing checks that
    `impl EventExact<Add> for C7` is true of C7. Deriving it means computing the
    view, which is shape B, and THEN carrying the marker impls on top. So the
    honest comparison for the marker shape is this one.
    """
    rnd = random.Random(seed)
    out = [PRELUDE_B,
           "pub struct Add;",
           "pub trait ValueAgree<Op> {}",
           "pub trait CausePresence<Op> {}",
           "pub trait CauseExact<Op> {}",
           "pub trait EventPresence<Op> {}",
           "pub trait EventExact<Op> {}",
           "pub fn need_weak<T: ValueAgree<Add>>() {}",
           "pub fn need_kleene<T: ValueAgree<Add> + CausePresence<Add>>() {}",
           "pub fn need_modulo_refusal<T: ValueAgree<Add> + EventExact<Add>>() {}",
           "pub fn need_graded<T: ValueAgree<Add> + CauseExact<Add> + CausePresence<Add> + EventExact<Add> + EventPresence<Add>>() {}",
           ]
    calls = []
    for i in range(n):
        kind = rnd.randrange(4)
        if kind == 0:   top, bot, dom, hr, req, call = 2, 2, 0, 0, "View::GRADED", "need_graded"
        elif kind == 1: top, bot, dom, hr, req, call = 2, 2, 1, 0, "View::KLEENE", "need_kleene"
        elif kind == 2: top, bot, dom, hr, req, call = 0, 0, 1, 0, "View::MODULO_REFUSAL", "need_modulo_refusal"
        else:           top, bot, dom, hr, req, call = 0, 2, 1, 0, "View::WEAK", "need_weak"
        arity = 4 + i % 3
        out.append(f"pub struct C{i};")
        out.append(f"pub const V{i}: bool = licensed(add_assoc_view({top}, {bot}, {dom}, {arity}, {hr}), {req});")
        out.append(f"const _: () = assert!(V{i});")
        out.append(f"impl ValueAgree<Add> for C{i} {{}}")
        if call == "need_graded":
            out += [f"impl CausePresence<Add> for C{i} {{}}", f"impl CauseExact<Add> for C{i} {{}}",
                    f"impl EventPresence<Add> for C{i} {{}}", f"impl EventExact<Add> for C{i} {{}}"]
        elif call == "need_kleene":
            out += [f"impl CausePresence<Add> for C{i} {{}}", f"impl CauseExact<Add> for C{i} {{}}"]
        elif call == "need_modulo_refusal":
            out += [f"impl EventPresence<Add> for C{i} {{}}", f"impl EventExact<Add> for C{i} {{}}"]
        calls.append(f"    {call}::<C{i}>();")
    out.append("pub fn use_all() {")
    out += calls
    out.append("}")
    return "\n".join(out) + "\n"

if __name__ == "__main__":
    shape, n, path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    open(path, "w").write({"a": shape_a, "b": shape_b, "c": shape_c}[shape](n))
