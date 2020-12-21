use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

struct Food {
    ingrs: BTreeSet<String>,
    alles: BTreeSet<String>,
}

pub fn run() {
    let txt = crate::common::get_input(21).unwrap();
    let mut foods = vec![];
    for l in txt.lines() {
        let (ingrs, alles) = l.split(" (contains ").collect_tuple().unwrap();
        let ingrs = ingrs.split_whitespace().map(|s| s.to_owned()).collect();
        let alles = alles
            .replace(')', "")
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        foods.push(Food { ingrs, alles });
    }
    let mut unknown_alles: BTreeSet<String> =
        foods.iter().flat_map(|f| f.alles.iter().cloned()).collect();
    let mut alle_to_ingr = BTreeMap::new();
    while foods.iter().any(|f| !f.alles.is_empty()) {
        for alle in unknown_alles.clone() {
            let mut ingrs: Option<BTreeSet<_>> = None;
            for f in &foods {
                if f.alles.contains(alle.as_str()) {
                    if let Some(ingrs) = &mut ingrs {
                        *ingrs = (*ingrs).intersection(&f.ingrs).cloned().collect();
                    } else {
                        ingrs = Some(f.ingrs.clone());
                    }
                }
            }
            if let Some(ingrs) = ingrs {
                if ingrs.len() == 1 {
                    let ingr = ingrs.into_iter().next().unwrap();
                    for f in &mut foods {
                        f.ingrs.remove(&ingr);
                        f.alles.remove(&alle);
                    }
                    unknown_alles.remove(&alle);
                    alle_to_ingr.insert(alle, ingr);
                }
            }
        }
    }
    dbg!(foods.iter().map(|f| f.ingrs.len()).sum::<usize>());
    dbg!(alle_to_ingr.values().cloned().collect_vec().join(","));
}
