use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let txt = crate::common::get_input(7).unwrap();
    let mut contain_cnt = HashMap::new();
    let mut contains = HashMap::new();
    let mut contain_by = HashMap::new();
    for l in txt.lines() {
        let (p, cs) = l.split(" bags contain ").collect_tuple().unwrap();
        if cs == "no other bags." {
            continue;
        }
        for c in cs.split(", ") {
            let spc = c.match_indices(' ').map(|(i, _)| i).collect_vec();
            let cnt = c[..spc[0]].parse::<i32>().unwrap();
            let c = &c[spc[0] + 1..spc[2]];
            contain_cnt.insert((p.to_string(), c.to_string()), cnt);
            contains
                .entry(p.to_string())
                .or_insert_with(Vec::new)
                .push(c);
            contain_by
                .entry(c.to_string())
                .or_insert_with(Vec::new)
                .push(p);
        }
    }
    let mut saw = HashSet::new();
    let mut q = vec!["shiny gold"];
    while let Some(c) = q.pop() {
        if let Some(ps) = contain_by.get(c) {
            for p in ps {
                if saw.insert(p) {
                    q.push(p);
                }
            }
        }
    }
    let cnt0 = saw.len();
    dbg!(cnt0);

    fn dfs(
        p: &str,
        cache: &mut HashMap<String, i32>,
        contains: &HashMap<String, Vec<&str>>,
        contain_cnt: &HashMap<(String, String), i32>,
    ) -> i32 {
        if let Some(bag_cnt) = cache.get(p) {
            return *bag_cnt;
        }
        let mut bag_cnt = 1;
        if let Some(cs) = contains.get(p) {
            for c in cs {
                let cnt = contain_cnt[&(p.to_string(), c.to_string())];
                bag_cnt += cnt * dfs(c, cache, &contains, &contain_cnt);
            }
        }
        cache.insert(p.to_string(), bag_cnt);
        bag_cnt
    }
    let mut cache = HashMap::new();
    let cnt1 = dfs("shiny gold", &mut cache, &contains, &contain_cnt) - 1;
    dbg!(cnt1);
}
