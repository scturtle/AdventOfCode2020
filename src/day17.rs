use itertools::iproduct;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub fn run() {
    let txt = crate::common::get_input(17).unwrap();
    let m: BTreeSet<_> = txt
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.bytes().enumerate().filter_map(move |(j, c)| {
                if c == b'#' {
                    Some((i as i32, j as i32, 0i32, 0i32))
                } else {
                    None
                }
            })
        })
        .collect();
    // part 2
    let mut cur = m;
    for _ in 0..6 {
        let mut cnt = BTreeMap::<_, usize>::new();
        for (x, y, z, q) in &cur {
            for (&i, &j, &k, &l) in iproduct!(&[-1, 0, 1], &[-1, 0, 1], &[-1, 0, 1], &[-1, 0, 1]) {
                if i == 0 && j == 0 && k == 0 && l == 0 {
                    continue;
                }
                *cnt.entry((x + i, y + j, z + k, q + l)).or_default() += 1;
            }
        }
        let mut nxt = BTreeSet::new();
        for (item, c) in cnt {
            if c == 3 || (c == 2 && cur.contains(&item)) {
                nxt.insert(item);
            }
        }
        cur = nxt;
    }
    dbg!(&cur.len());
}
