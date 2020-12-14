use itertools::Itertools;
use std::collections::HashMap;

fn dfs(addr: &mut [char], mask: &[char], mem: &mut HashMap<u64, u64>, val: u64, i: usize) {
    if i == addr.len() {
        let addr = addr.iter().cloned().collect::<String>();
        let addr = u64::from_str_radix(&addr, 2).unwrap();
        mem.insert(addr, val);
    } else if mask[i] == '0' {
        dfs(addr, mask, mem, val, i + 1);
    } else if mask[i] == '1' {
        addr[i] = '1';
        dfs(addr, mask, mem, val, i + 1);
    } else {
        addr[i] = '0';
        dfs(addr, mask, mem, val, i + 1);
        addr[i] = '1';
        dfs(addr, mask, mem, val, i + 1);
    }
}

pub fn run() {
    let txt = crate::common::get_input(14).unwrap();
    let insts = txt.lines().collect_vec();
    // part 1
    let mut mem = HashMap::new();
    let mut mask0 = 0;
    let mut mask1 = 0;
    for inst in &insts {
        if inst.starts_with("mask") {
            let mask = &inst[7..];
            mask0 = u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
            mask1 = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
        } else {
            let (addr, val) = inst[4..]
                .split("] = ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            let val = (val & mask0) | mask1;
            mem.insert(addr, val);
        }
    }
    dbg!(mem.values().sum::<u64>());
    // part 2
    let mut mem = HashMap::new();
    let mut mask = vec![];
    for inst in &insts {
        if inst.starts_with("mask") {
            mask = inst[7..].chars().collect_vec();
        } else {
            let (addr, val) = inst[4..]
                .split("] = ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            let mut addr = format!("{:036b}", addr).chars().collect_vec();
            dfs(&mut addr, &mask, &mut mem, val, 0);
        }
    }
    dbg!(mem.values().sum::<u64>());
}
