use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[rustfmt::skip]
#[derive(PartialEq, Debug, Clone, Copy)]
enum D { E, SE, SW, W, NW, NE }

fn to_dirs(s: &str) -> Vec<D> {
    let mut ds = vec![];
    let bs = s.bytes().collect_vec();
    let mut i = 0;
    while i < bs.len() {
        #[rustfmt::skip]
        match bs[i] {
            b'e' => { ds.push(D::E); i += 1; }
            b'w' => { ds.push(D::W); i += 1; }
            b's' => match bs[i + 1] {
                b'e' => { ds.push(D::SE); i += 2; }
                b'w' => { ds.push(D::SW); i += 2; }
                _ => panic!(),
            }
            b'n' => match bs[i + 1] {
                b'e' => { ds.push(D::NE); i += 2; }
                b'w' => { ds.push(D::NW); i += 2; }
                _ => panic!(),
            }
            _ => panic!(),
        }
    }
    ds
}

fn mv((x, y): (i32, i32), d: D) -> (i32, i32) {
    #[rustfmt::skip]
    match d {
        D::E => (x + 1, y + 1),
        D::W => (x - 1, y - 1),
        D::SE => (x + 1, y),
        D::SW => (x, y - 1),
        D::NE => (x, y + 1),
        D::NW => (x - 1, y),
    }
}

fn flip(blacks: BTreeSet<(i32, i32)>) -> BTreeSet<(i32, i32)> {
    let mut cnt = BTreeMap::<_, usize>::new();
    for (x, y) in &blacks {
        for (xx, yy) in &[(1, 1), (-1, -1), (0, 1), (1, 0), (0, -1), (-1, 0)] {
            *cnt.entry((x + xx, y + yy)).or_default() += 1;
        }
    }
    let mut new_blacks = BTreeSet::new();
    for (p, c) in &cnt {
        if *c == 2 || (*c == 1 && blacks.contains(p)) {
            new_blacks.insert(*p);
        }
    }
    new_blacks
}

pub fn run() {
    let txt = crate::common::get_input(24).unwrap();
    let tiles = txt.trim().lines().map(to_dirs).collect_vec();
    let mut blacks = BTreeSet::new();
    for t in &tiles {
        let mut p = (0, 0);
        for &d in t {
            p = mv(p, d);
        }
        if !blacks.insert(p) {
            blacks.remove(&p);
        }
    }
    dbg!(blacks.len());
    for _ in 0..100 {
        blacks = flip(blacks);
    }
    dbg!(blacks.len());
}
