// t6. Option 5's bridge, in the only shape that needs no arithmetic: one impl per width. The
// question is not whether it compiles but what it costs, so the file is deliberately three
// widths long and the cost is the row count.
trait Nat { const USIZE: usize; }
struct N8; struct N13; struct N16;
impl Nat for N8 { const USIZE: usize = 8; }
impl Nat for N13 { const USIZE: usize = 13; }
impl Nat for N16 { const USIZE: usize = 16; }

trait WidthOf { type Nat: Nat; }
struct W<const N: usize>;
impl WidthOf for W<8> { type Nat = N8; }
impl WidthOf for W<13> { type Nat = N13; }
impl WidthOf for W<16> { type Nat = N16; }

fn main() { println!("{}", <W<13> as WidthOf>::Nat::USIZE); }
