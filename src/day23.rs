use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(23).unwrap();
    let val: Vec<i32> = txt.trim().bytes().map(|b| (b - b'0') as i32).collect();

    // for part 1
    // const STEP: i32 = 100;

    // for part2
    const STEP: i32 = 1000_0000;
    let mut val = val;
    for i in 10..=100_0000 {
        val.push(i);
    }

    let max = *val.iter().max().unwrap();

    // pos: val -> index in nxt/val
    let mut pos: Vec<usize> = vec![0; val.len() + 1];
    for (i, &t) in val.iter().enumerate() {
        pos[t as usize] = i;
    }
    // nxt: index -> next index
    let mut nxt: Vec<usize> = (1..val.len()).chain(0..=0).collect_vec();

    let mut cur = 0;
    for _ in 0..STEP {
        // remember next three
        let t0 = nxt[cur];
        let t1 = nxt[t0];
        let t2 = nxt[t1];
        // pop that three
        nxt[cur] = nxt[t2];
        // find dest
        let mut dest = val[cur] - 1;
        loop {
            if dest == val[t0] || dest == val[t1] || dest == val[t2] {
                dest -= 1;
            } else if dest == 0 {
                dest = max;
            } else {
                break;
            }
        }
        // dest val to index
        let dest = pos[dest as usize];
        // insert that three after dest
        let bak = nxt[dest];
        nxt[dest] = t0;
        nxt[t2] = bak;
        // move on
        cur = nxt[cur];
    }

    // let mut ans1 = 0;
    // let mut p = nxt[pos[1]];
    // for _ in 0..8 {
    //     ans1 = ans1 * 10 + val[p];
    //     p = nxt[p];
    // }
    // dbg!(ans1);

    let ans2 = val[nxt[pos[1]]] * val[nxt[nxt[pos[1]]]];
    dbg!(ans2);
}
