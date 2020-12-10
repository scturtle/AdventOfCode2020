use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(10).unwrap();
    let mut vs = txt
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .sorted()
        .collect_vec();
    vs.insert(0, 0);
    vs.push(vs.last().unwrap() + 3);
    // part 1
    let mut cnt1 = 0;
    let mut cnt3 = 0;
    let mut last = 0;
    for v in &vs[1..] {
        if v - last == 1 {
            cnt1 += 1;
        } else if v - last == 3 {
            cnt3 += 1;
        }
        last = *v;
    }
    dbg!(cnt1 * cnt3);
    // part 2
    let mut dp = vec![0u64; vs.len()];
    dp[0] = 1;
    for i in 1..vs.len() {
        for j in (0..i).rev() {
            if vs[i] - vs[j] > 3 {
                break;
            }
            dp[i] += dp[j];
        }
    }
    dbg!(dp.last().unwrap());
}
