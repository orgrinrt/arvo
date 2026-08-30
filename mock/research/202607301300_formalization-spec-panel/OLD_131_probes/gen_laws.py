import sys
k = int(sys.argv[1])
out = ['#![no_std]','use arvocore::*;']
for i in range(k):
    out.append(f'''pub struct Law{i}<const A: u32, const B: u32, const C: u32>;
impl<const A: u32, const B: u32, const C: u32> Law{i}<A, B, C> {{
    pub const HOLDS: () = assert!(C == A + B + {i}, "arvo: law {i} does not follow from its inputs. The law: C equals A plus B plus {i}. The line above prints Law{i}::<A, B, C> with the actual digit counts, in that order.");
}}
pub fn op{i}<const A: u32, const B: u32, const C: u32>() {{ let () = Law{i}::<A, B, C>::HOLDS; }}''')
# 64 compositions per law at four-digit widths
out.append('pub fn drive() {')
for i in range(k):
    for j in range(64):
        a, b = 1000 + j, 1200 + j
        out.append(f'  op{i}::<{a}, {b}, {a+b+i}>();')
out.append('}')
open(f'lawsweep_{k}.rs','w').write('\n'.join(out))
