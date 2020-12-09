use itertools::{Itertools, MinMaxResult};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run() {
    const P: usize = 25;
    let txt = crate::common::get_input(9).unwrap();
    let arr = txt.lines().map(|x| x.parse::<u64>().unwrap()).collect_vec();

    let mut m: HashMap<u64, usize> = HashMap::new();
    for i in 0..P - 1 {
        for j in i + 1..P {
            *m.entry(arr[i] + arr[j]).or_default() += 1;
        }
    }
    let mut sum0 = 0;
    for i in P..arr.len() {
        let new = arr[i];
        if *m.get(&new).unwrap_or(&0) == 0 {
            sum0 = new;
            break;
        }
        let old = arr[i - P];
        for j in i - P + 1..i {
            *m.entry(old + arr[j]).or_default() -= 1;
        }
        for j in i - P + 1..i {
            *m.entry(new + arr[j]).or_default() += 1;
        }
    }
    dbg!(sum0);

    let mut acc = 0;
    let (mut l, mut r) = (0, 0);
    loop {
        match acc.cmp(&sum0) {
            Ordering::Less => {
                acc += arr[r];
                r += 1;
            }
            Ordering::Greater => {
                acc -= arr[l];
                l += 1;
            }
            _ => {
                break;
            }
        }
    }
    let sum1 = match (l..r).map(|i| arr[i]).minmax() {
        MinMaxResult::MinMax(arr, b) => arr + b,
        _ => panic!(),
    };
    dbg!(sum1);
}
