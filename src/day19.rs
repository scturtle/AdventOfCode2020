use itertools::Itertools;
use std::collections::HashMap;

enum Rule {
    Single(char),
    Branch(Vec<Vec<usize>>),
}

fn parse_rule(pat: &str) -> Rule {
    if pat.starts_with('"') {
        Rule::Single(pat.chars().nth(1).unwrap())
    } else {
        let branches = pat
            .split(" | ")
            .map(|b| {
                b.split(' ')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Rule::Branch(branches)
    }
}

fn rule_to_regex(rules: &HashMap<usize, Rule>, i: usize) -> String {
    match &rules.get(&i).unwrap() {
        Rule::Single(c) => c.to_string(),
        Rule::Branch(bs) => bs
            .iter()
            .map(|b| {
                b.iter()
                    .map(|i| "(".to_string() + &rule_to_regex(rules, *i) + ")")
                    .join("")
            })
            .join("|"),
    }
}

pub fn run() {
    let txt = crate::common::get_input(19).unwrap();
    let mut lines = txt.lines();
    let mut rules = HashMap::new();
    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let (id, pat) = l.split(": ").collect_tuple().unwrap();
        let id: usize = id.parse().unwrap();
        rules.insert(id, parse_rule(pat));
    }
    let msgs = lines.collect_vec();
    let pat0 = regex::Regex::new(&format!("^{}$", rule_to_regex(&rules, 0))).unwrap();
    let cnt0 = msgs.iter().filter(|msg| pat0.is_match(msg)).count();
    dbg!(cnt0);

    let p42 = rule_to_regex(&rules, 42);
    let p31 = rule_to_regex(&rules, 31);
    let pats = (1..10)
        .map(|i| {
            regex::Regex::new(
                &format!("^({})[{},]({})[{}]$", &p42, i + 1, &p31, i)
                    .replace('[', "{")
                    .replace(']', "}"),
            )
            .unwrap()
        })
        .collect_vec();
    let cnt1 = msgs
        .iter()
        .filter(|msg| pats.iter().any(|p| p.is_match(msg)))
        .count();
    dbg!(cnt1);
}
