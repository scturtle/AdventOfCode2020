use itertools::Itertools;

pub fn run() {
    let txt = crate::common::get_input(5).unwrap();
    let ids = txt
        .lines()
        .map(|l| {
            isize::from_str_radix(
                &l.replace('F', "0")
                    .replace('B', "1")
                    .replace('L', "0")
                    .replace('R', "1"),
                2,
            )
            .unwrap()
        })
        .sorted()
        .collect_vec();
    let max_id = ids.iter().last().unwrap();
    dbg!(max_id);
    let missing_id = (ids[0]..=*max_id).sum::<isize>() - ids.iter().sum::<isize>();
    dbg!(missing_id);
}
