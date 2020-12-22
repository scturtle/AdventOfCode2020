use itertools::Itertools;
use std::collections::HashSet;

fn val(p: Vec<usize>) -> usize {
    p.iter()
        .rev()
        .enumerate()
        .map(|(i, t)| (i + 1) * t)
        .sum::<usize>()
}

fn game1(p1: Vec<usize>, p2: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut p1 = p1;
    let mut p2 = p2;
    while !p1.is_empty() && !p2.is_empty() {
        let t1 = p1.remove(0);
        let t2 = p2.remove(0);
        if t1 > t2 {
            p1.push(t1);
            p1.push(t2);
        } else {
            p2.push(t2);
            p2.push(t1);
        }
    }
    (p1, p2)
}

fn game2(mut p1: Vec<usize>, mut p2: Vec<usize>) -> (bool, Vec<usize>, Vec<usize>) {
    let mut saw = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let key = (
            p1.iter().map(|i| i.to_string()).collect_vec().join(","),
            p2.iter().map(|i| i.to_string()).collect_vec().join(","),
        );
        if !saw.insert(key) {
            return (true, p1, p2);
        }
        let p1_win = game2_test(&p1, &p2);
        let t1 = p1.remove(0);
        let t2 = p2.remove(0);
        if p1_win {
            p1.push(t1);
            p1.push(t2);
        } else {
            p2.push(t2);
            p2.push(t1);
        }
    }
    (p2.is_empty(), p1, p2)
}

fn game2_test(p1: &Vec<usize>, p2: &Vec<usize>) -> bool {
    if p1.len() > p1[0] && p2.len() > p2[0] {
        let p1 = p1.iter().cloned().skip(1).take(p1[0]).collect_vec();
        let p2 = p2.iter().cloned().skip(1).take(p2[0]).collect_vec();
        game2(p1, p2).0
    } else {
        p1[0] > p2[0]
    }
}

pub fn run() {
    let txt = crate::common::get_input(22).unwrap();
    let (p1, p2) = txt.split("\n\n").collect_tuple().unwrap();
    let p1: Vec<usize> = p1.lines().skip(1).map(|i| i.parse().unwrap()).collect_vec();
    let p2: Vec<usize> = p2.lines().skip(1).map(|i| i.parse().unwrap()).collect_vec();
    let (a1, a2) = game1(p1.clone(), p2.clone());
    dbg!(val(a1));
    dbg!(val(a2));
    let (_, a1, a2) = game2(p1, p2);
    dbg!(val(a1));
    dbg!(val(a2));
}
