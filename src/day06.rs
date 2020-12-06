use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(6).unwrap();
    let groups = &txt
        .lines()
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(is_empty, lines)| {
            if !is_empty {
                Some(lines.collect_vec())
            } else {
                None
            }
        })
        .collect_vec();
    let cnt0: usize = groups
        .iter()
        .map(|g| g.join("").chars().sorted().dedup().count())
        .sum();
    dbg!(cnt0);
    let cnt1: usize = groups
        .iter()
        .map(|g| {
            ('a'..='z')
                .filter(|c| g.iter().all(|l| l.contains(*c)))
                .count()
        })
        .sum();
    dbg!(cnt1);
}
