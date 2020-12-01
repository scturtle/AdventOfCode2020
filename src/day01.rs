pub fn run() {
    let entries = crate::common::get_input(1)
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..entries.len() - 1 {
        for j in i + 1..entries.len() {
            if entries[i] + entries[j] == 2020 {
                println!("{}", entries[i] * entries[j]);
            }
        }
    }
    for i in 0..entries.len() - 2 {
        for j in i + 1..entries.len() - 1 {
            for k in j + 1..entries.len() {
                if entries[i] + entries[j] + entries[k] == 2020 {
                    println!("{}", entries[i] * entries[j] * entries[k]);
                }
            }
        }
    }
}
