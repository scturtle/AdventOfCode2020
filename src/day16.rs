use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

pub fn run() {
    let txt = crate::common::get_input(16).unwrap();
    let mut lines = txt.lines();
    // parse all rules
    let mut rules = HashMap::new();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let (field, ranges) = l.split(": ").collect_tuple().unwrap();
        let ranges: Vec<(u64, u64)> = ranges
            .split(" or ")
            .map(|r| {
                r.split('-')
                    .map(|i| i.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();
        rules.insert(field.to_owned(), ranges);
    }
    // parse tickets
    fn to_vec(s: &str) -> Vec<u64> {
        s.split(',').map(|i| i.parse().unwrap()).collect_vec()
    }
    lines.next();
    let my_ticket: Vec<u64> = to_vec(lines.next().unwrap());
    lines.next();
    lines.next();
    let nearby_tickets: Vec<Vec<u64>> = lines.map(to_vec).collect_vec();

    // filter bad tickets
    let mut bad_sum = 0;
    let mut good_tickets = vec![my_ticket.clone()];
    for t in nearby_tickets {
        let mut bad = false;
        for v in &t {
            // valid if match any rule
            if rules
                .values()
                .any(|r| r.iter().any(|(min, max)| v >= min && v <= max))
            {
                continue;
            }
            bad_sum += v;
            bad = true;
        }
        if !bad {
            good_tickets.push(t);
        }
    }
    dbg!(bad_sum);

    let values: Vec<BTreeSet<u64>> = (0..my_ticket.len())
        .map(|idx| good_tickets.iter().map(|t| t[idx]).collect())
        .collect_vec();

    let mut possible_fields: Vec<BTreeSet<&str>> = values
        .iter()
        .map(|vals| {
            rules
                .iter()
                .filter(|(_, r)| {
                    vals.iter()
                        .all(|v| r.iter().any(|(min, max)| v >= min && v <= max))
                })
                .map(|(f, _)| f.as_str())
                .collect::<BTreeSet<_>>()
        })
        .collect_vec();

    let mut todo: BTreeSet<usize> = (0..possible_fields.len()).collect();
    while !todo.is_empty() {
        for i in todo.clone() {
            if possible_fields[i].len() == 1 {
                let f = *possible_fields[i].iter().next().unwrap();
                for fs in &mut possible_fields {
                    if fs.len() > 1 {
                        fs.remove(f);
                    }
                }
                todo.remove(&i);
            }
        }
    }

    let mut mul = 1;
    for (i, fs) in possible_fields.iter().enumerate() {
        let f = fs.iter().next().unwrap();
        if f.starts_with("departure") {
            mul *= my_ticket[i];
        }
    }
    dbg!(mul);
}
