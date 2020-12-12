use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(12).unwrap();
    let insts = txt.lines().collect_vec();
    // part 1
    let (mut x, mut y) = (0, 0);
    let dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut d: i32 = 0;
    for inst in &insts {
        let n: i32 = inst[1..].parse().unwrap();
        #[rustfmt::skip]
        match inst.chars().next().unwrap() {
            'R' => { d = (d + n / 90) % 4; }
            'L' => { d = (d + n / 90 * 3) % 4; }
            'N' => { x -= n; }
            'S' => { x += n; }
            'E' => { y += n; }
            'W' => { y -= n; }
            'F' => {
                x += dirs[d as usize].0 * n;
                y += dirs[d as usize].1 * n;
            }
            _ => unreachable!(),
        }
    }
    dbg!(x.abs() + y.abs());
    // part 2
    let (mut wx, mut wy) = (-1, 10);
    let (mut x, mut y) = (0, 0);
    for inst in &insts {
        let n: i32 = inst[1..].parse().unwrap();
        #[rustfmt::skip]
        match inst.chars().next().unwrap() {
            'F' => { x += wx * n; y += wy * n; }
            'N' => { wx -= n; }
            'S' => { wx += n; }
            'E' => { wy += n; }
            'W' => { wy -= n; }
            'L' => {
                for _ in 0..n / 90 {
                    let wx0 = wx; wx = -wy; wy = wx0;
                }
            }
            'R' => {
                for _ in 0..n / 90 {
                    let wx0 = wx; wx = wy; wy = -wx0;
                }
            }
            _ => unreachable!(),
        }
    }
    dbg!(x.abs() + y.abs());
}
