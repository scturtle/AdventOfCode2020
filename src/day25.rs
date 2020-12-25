use itertools::Itertools;

fn tran(v: u64, s: u64) -> u64 {
    (v * s) % 20201227
}

pub fn run() {
    let txt = crate::common::get_input(25).unwrap();
    let (pub1, pub2) = txt
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut l1 = 0;
    let mut v = 1;
    while v != pub1 {
        l1 += 1;
        v = tran(v, 7);
    }
    let mut e = 1;
    for _ in 0..l1 {
        e = tran(e, pub2);
    }
    dbg!(e);
}
