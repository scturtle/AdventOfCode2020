use itertools::Itertools;

fn count_trees(m: &[Vec<usize>], j_step: usize, i_step: usize) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut cnt = 0;
    loop {
        i += i_step;
        if i >= m.len() {
            break;
        }
        j = (j + j_step) % m[0].len();
        cnt += m[i][j];
    }
    cnt
}

pub fn run() {
    let m = crate::common::get_input(3)
        .unwrap()
        .trim()
        .split('\n')
        .map(|l| l.bytes().map(|b| (b == b'#') as usize).collect_vec())
        .collect_vec();
    let c0 = count_trees(&m, 1, 1);
    let c1 = count_trees(&m, 3, 1);
    let c2 = count_trees(&m, 5, 1);
    let c3 = count_trees(&m, 7, 1);
    let c4 = count_trees(&m, 1, 2);
    dbg!(c1);
    dbg!(c0 * c1 * c2 * c3 * c4);
}
