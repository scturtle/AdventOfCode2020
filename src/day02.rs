use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(2).unwrap();
    let lines = txt.lines().collect_vec();
    let mut ok1 = 0;
    let mut ok2 = 0;
    for line in &lines {
        let t = line.split(' ').collect_vec();
        let cnts = t[0]
            .split('-')
            .map(|i| i.parse::<usize>().unwrap())
            .collect_vec();
        let that = t[1].chars().next().unwrap();
        let cnt = t[2].chars().filter(|c| *c == that).count();
        if cnt >= cnts[0] && cnt <= cnts[1] {
            ok1 += 1;
        }
        let cnt2 = (t[2].chars().nth(cnts[0] - 1) == Some(that)) as i32
            + (t[2].chars().nth(cnts[1] - 1) == Some(that)) as i32;
        if cnt2 == 1 {
            ok2 += 1;
        }
    }
    println!("{}", ok1);
    println!("{}", ok2);
}
