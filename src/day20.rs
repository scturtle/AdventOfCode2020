use itertools::iproduct;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::convert::TryInto;

fn flip(b: u16) -> u16 {
    let mut b2 = 0;
    for i in 0..10 {
        if b & (1 << i) != 0 {
            b2 |= 1 << (9 - i);
        }
    }
    b2
}

fn all_variations(t: [u16; 4]) -> [[u16; 4]; 8] {
    let (u, d, l, r) = (t[0], t[1], t[2], t[3]);
    [
        [u, d, l, r],
        [flip(l), flip(r), d, u],
        [flip(d), flip(u), flip(r), flip(l)],
        [r, l, flip(u), flip(d)],
        [d, u, flip(l), flip(r)],
        [l, r, u, d],
        [flip(u), flip(d), r, l],
        [flip(r), flip(l), flip(d), flip(u)],
    ]
}

struct Tile {
    id: u64,
    borders: [[u16; 4]; 8],
    matrices: [[[u8; 8]; 8]; 8],
}

fn parse_tile(ls: &str) -> Tile {
    let ls = ls.lines().collect_vec();
    let id = ls[0][5..9].parse::<u64>().unwrap();
    fn b2u16(cs: &str) -> u16 {
        u16::from_str_radix(cs, 2).unwrap()
    }
    let bs = [
        b2u16(ls[1]),
        b2u16(ls[10]),
        b2u16(
            &ls[1..]
                .iter()
                .map(|l| l.chars().next().unwrap())
                .collect::<String>(),
        ),
        b2u16(
            &ls[1..]
                .iter()
                .map(|l| l.chars().last().unwrap())
                .collect::<String>(),
        ),
    ];
    fn rot(m: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
        let mut m2 = [[0u8; 8]; 8];
        for (i, j) in iproduct!(0..8, 0..8) {
            m2[i][j] = m[8 - 1 - j][i];
        }
        m2
    }
    let m: [[u8; 8]; 8] = ls[2..10]
        .iter()
        .map(|l| {
            l.bytes().map(|b| b - b'0').collect_vec()[1..9]
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    let m2 = m.iter().cloned().rev().collect_vec().try_into().unwrap();
    let matrices = [
        m,
        rot(m),
        rot(rot(m)),
        rot(rot(rot(m))),
        m2,
        rot(m2),
        rot(rot(m2)),
        rot(rot(rot(m2))),
    ];
    let borders = all_variations(bs);
    Tile {
        id,
        borders,
        matrices,
    }
}

type Order = (u64, usize);

fn find_left_top(tiles: &BTreeMap<u64, Tile>) -> Order {
    for t0 in tiles.values() {
        for i in 0..8 {
            let mut cnt = [0; 4];
            for t1 in tiles.values() {
                if t0.id != t1.id {
                    for (j, c) in cnt.iter_mut().enumerate() {
                        let b0 = t0.borders[i][j];
                        for &b1 in &t1.borders[0] {
                            if b0 == b1 || b0 == flip(b1) {
                                *c += 1;
                            }
                        }
                    }
                }
            }
            if cnt == [0, 1, 0, 1] {
                return (t0.id, i);
            }
        }
    }
    panic!()
}

fn find_right(tile_map: &BTreeMap<u64, Tile>, this: Order) -> Order {
    let bs0 = tile_map[&this.0].borders[this.1];
    for t1 in tile_map.values() {
        if t1.id != this.0 {
            for i in 0..8 {
                if bs0[3] == t1.borders[i][2] {
                    return (t1.id, i);
                }
            }
        }
    }
    panic!()
}

fn find_down(tile_map: &BTreeMap<u64, Tile>, this: Order) -> Order {
    let bs0 = tile_map[&this.0].borders[this.1];
    for t1 in tile_map.values() {
        if t1.id != this.0 {
            for i in 0..8 {
                if bs0[1] == t1.borders[i][0] {
                    return (t1.id, i);
                }
            }
        }
    }
    panic!()
}

fn gather_up(tile_map: &BTreeMap<u64, Tile>, n: usize, orders: &[Order]) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for i in 0..n {
        res.extend(vec![vec![]; 8]);
        for j in 0..n {
            let (id, k) = orders[i * n + j];
            let m = tile_map[&id].matrices[k];
            for ii in 0..8 {
                res[i * 8 + ii].extend(m[ii].iter());
            }
        }
    }
    res
}

fn count_dragon(m: Vec<Vec<u8>>) -> usize {
    let dragon: Vec<Vec<u8>> = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
        .split('\n')
        .map(|l| {
            l.bytes()
                .map(|b| if b == b'#' { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec();
    fn rot(m: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let n = m.len();
        let mut m2 = vec![vec![0u8; n]; n];
        for (i, j) in iproduct!(0..n, 0..n) {
            m2[i][j] = m[n - 1 - j][i];
        }
        m2
    }
    let m2 = m.iter().cloned().rev().collect_vec();
    for m in &[
        m.clone(),
        rot(&m),
        rot(&rot(&m)),
        rot(&rot(&rot(&m))),
        m2.clone(),
        rot(&m2),
        rot(&rot(&m2)),
        rot(&rot(&rot(&m2))),
    ] {
        let mut found = false;
        let mut m2 = m.clone();
        for (i, j) in iproduct!(0..m.len() - 3 + 1, 0..m[0].len() - 20 + 1) {
            let mut is_dragon = true;
            for (ii, jj) in iproduct!(0..3, 0..20) {
                if dragon[ii][jj] == 1 && m[i + ii][j + jj] == 0 {
                    is_dragon = false;
                }
            }
            if is_dragon {
                found = true;
                for (ii, jj) in iproduct!(0..3, 0..20) {
                    if dragon[ii][jj] == 1 {
                        m2[i + ii][j + jj] = 0;
                    }
                }
            }
        }
        if found {
            return m2.into_iter().flatten().map(|i| i as usize).sum::<usize>();
        }
    }
    panic!()
}

pub fn run() {
    let txt = crate::common::get_input(20).unwrap();
    let tile_map: BTreeMap<u64, Tile> = txt
        .replace('#', "1")
        .replace('.', "0")
        .trim()
        .split("\n\n")
        .map(parse_tile)
        .map(|t| (t.id, t))
        .collect();
    let n = (tile_map.len() as f32).sqrt() as usize;
    let mut orders = vec![];
    for (i, j) in iproduct!(0..n, 0..n) {
        if i == 0 && j == 0 {
            orders.push(find_left_top(&tile_map));
        } else if i == 0 {
            orders.push(find_right(&tile_map, orders[j - 1]));
        } else {
            orders.push(find_down(&tile_map, orders[(i - 1) * n + j]));
        }
    }
    let m = gather_up(&tile_map, n, &orders);
    let cnt = count_dragon(m);
    dbg!(cnt);
}
