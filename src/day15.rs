use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let txt = crate::common::get_input(15).unwrap();
    let start_numbers: Vec<usize> = txt
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect_vec();
    let mut lastlast = HashMap::new();
    let mut last: HashMap<_, _> = start_numbers
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    let mut n = *start_numbers.iter().last().unwrap();
    for i in start_numbers.len()..30000000 {
        if let Some(l) = last.get(&n) {
            if let Some(ll) = lastlast.get(&n) {
                n = l - ll;
            } else {
                n = 0;
            }
        } else {
            n = 0;
        }
        if let Some(&l) = last.get(&n) {
            lastlast.insert(n, l);
        }
        last.insert(n, i);
        if i + 1 == 2020 {
            dbg!(n);
        }
    }
    dbg!(n);
}
