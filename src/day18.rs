use itertools::Itertools;

fn find_right_parentheses(expr: &[u8], i: usize) -> usize {
    let mut c = 1;
    for j in i + 1.. {
        if expr[j] == b')' {
            c -= 1;
            if c == 0 {
                return j;
            }
        } else if expr[j] == b'(' {
            c += 1;
        }
    }
    unreachable!()
}

fn eval1(expr: &[u8]) -> u64 {
    let mut i = 0;
    let mut last_val = 0;
    let mut last_op = b'+';
    let step = |val: u64, last_val: &mut u64, last_op: u8| {
        *last_val = if last_op == b'+' {
            *last_val + val
        } else {
            *last_val * val
        };
    };
    while i != expr.len() {
        match expr[i] {
            b' ' => {
                i += 1;
            }
            b'0'..=b'9' => {
                let val = (expr[i] - b'0') as u64;
                step(val, &mut last_val, last_op);
                i += 1;
            }
            b'+' | b'*' => {
                last_op = expr[i];
                i += 1;
            }
            b'(' => {
                let j = find_right_parentheses(expr, i);
                let val = eval1(&expr[i + 1..j]);
                step(val, &mut last_val, last_op);
                i = j + 1;
            }
            _ => panic!(),
        }
    }
    last_val
}

fn eval2(expr: &[u8]) -> u64 {
    let mut i = 0;
    let mut vals = vec![];
    let mut ops = vec![];
    let step = |val: u64, vals: &mut Vec<u64>, ops: &mut Vec<u8>| match ops.last() {
        None | Some(b'*') => {
            vals.push(val);
        }
        Some(b'+') => {
            ops.pop();
            let last_val = vals.pop().unwrap();
            vals.push(last_val + val);
        }
        _ => panic!(),
    };
    while i != expr.len() {
        match expr[i] {
            b' ' => {
                i += 1;
            }
            b'0'..=b'9' => {
                let val = (expr[i] - b'0') as u64;
                step(val, &mut vals, &mut ops);
                i += 1;
            }
            b'+' | b'*' => {
                ops.push(expr[i]);
                i += 1;
            }
            b'(' => {
                let j = find_right_parentheses(expr, i);
                let val = eval2(&expr[i + 1..j]);
                step(val, &mut vals, &mut ops);
                i = j + 1;
            }
            _ => panic!(),
        }
    }
    vals.into_iter().product()
}

pub fn run() {
    let txt = crate::common::get_input(18).unwrap();
    let exprs = txt.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for expr in &exprs {
        sum1 += eval1(expr);
        sum2 += eval2(expr);
    }
    dbg!(sum1);
    dbg!(sum2);
}
