// p2 diagnostic: why did C1 fail? Splits the 256 bad pairs by cause.
// Not a finding. It exists so the C1 failure is diagnosed rather than relaxed.
include!("p2_joins_exist_meets_do_not_core.rs");

fn main() {
    let c = catalogue();
    let consts: Vec<usize> = (0..c.len()).filter(|&i| c[i].kind == Kind::Constant).collect();
    let mut empty_inter = 0;
    let mut one_inter = 0;
    let mut many_max = 0;
    let mut ok = 0;
    let mut ex_many = String::new();
    let mut ex_empty = String::new();
    for (a, b) in pairs(&consts) {
        let inter: Set = c[a].vals.intersection(&c[b].vals).copied().collect();
        let lb: Vec<usize> = consts.iter().copied()
            .filter(|&i| c[i].vals.is_subset(&c[a].vals) && c[i].vals.is_subset(&c[b].vals))
            .collect();
        let g = maximal(&c, &lb);
        match g.len() {
            1 => ok += 1,
            0 => {
                if inter.is_empty() { empty_inter += 1;
                    if ex_empty.is_empty() { ex_empty = format!("{} /\\ {} = {{}}", c[a].name, c[b].name); }
                } else { one_inter += 1;
                    if ex_empty.is_empty() { ex_empty = format!("{} /\\ {} = {} values, no catalogue shape inside", c[a].name, c[b].name, inter.len()); }
                }
            }
            _ => { many_max += 1;
                if ex_many.is_empty() {
                    ex_many = format!("{} /\\ {}  -> maximal lower bounds: {}", c[a].name, c[b].name,
                        g.iter().map(|&i| c[i].name.clone()).collect::<Vec<_>>().join(" | "));
                }
            }
        }
    }
    println!("### p2d. diagnosis of the C1 failure, constant-quantum family only");
    println!("  pairs with a unique meet                       {ok}");
    println!("  pairs with NO lower bound, intersection empty  {empty_inter}");
    println!("  pairs with NO lower bound, intersection small  {one_inter}");
    println!("  pairs with SEVERAL maximal lower bounds        {many_max}");
    println!();
    println!("  example of the no-lower-bound case:  {ex_empty}");
    println!("  example of the several case:         {}", if ex_many.is_empty() { "none".into() } else { ex_many });
}
