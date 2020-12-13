use itertools::Itertools;

/// x = a ^(p - 2) (mod p)
fn inv(mut a: u128, p: u128) -> u128 {
    let mut ans = 1;
    let mut b = p - 2;
    while b > 0 {
        if b & 1 != 0 {
            ans = (a * ans) % p;
        }
        a = (a * a) % p;
        b >>= 1;
    }
    ans
}

pub fn run() {
    let txt = crate::common::get_input(13).unwrap();
    let lines = txt.lines().collect_vec();
    // part 1
    let est: i32 = lines[0].parse().unwrap();
    let ids: Vec<i32> = lines[1]
        .split(',')
        .filter(|s| *s != "x")
        .map(|i| i.parse().unwrap())
        .collect_vec();
    let (ts, id) = ids
        .into_iter()
        .map(|id| {
            let t = (est + id - 1) / id * id;
            (t, id)
        })
        .min()
        .unwrap();
    dbg!((ts - est) * id);
    // part 2
    let aps: Vec<(u128, u128)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, p)| (i as u128, p.parse().unwrap()))
        .collect_vec();
    // the chinese remainder theorem
    let n: u128 = aps.iter().map(|(_, p)| p).product();
    let ans = aps
        .into_iter()
        .map(|(a, p)| {
            let m = n / p;
            let b = inv(m, p);
            let a = p - (a % p); // NOTE
            a * m * b
        })
        .sum::<u128>()
        % n;
    dbg!(ans);
}
