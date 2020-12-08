use itertools::Itertools;
use std::collections::HashSet;

fn exe(insts: &[(&str, i32)], fix: i32) -> Result<i32, i32> {
    let mut accumulator = 0;
    let mut pc = 0;
    let mut saw = HashSet::new();
    while saw.insert(pc) {
        let (cmd, val) = insts[pc as usize];
        match cmd {
            "acc" => {
                accumulator += val;
                pc += 1;
            }
            "jmp" => {
                pc += if fix == pc { 1 } else { val };
            }
            "nop" => {
                pc += if fix == pc { val } else { 1 };
            }
            _ => unreachable!(),
        }
        if pc < 0 {
            return Err(accumulator);
        }
        if pc == insts.len() as i32 {
            return Ok(accumulator);
        }
    }
    Err(accumulator)
}

pub fn run() {
    let txt = crate::common::get_input(8).unwrap();
    let insts = txt
        .lines()
        .map(|l| {
            let (cmd, val) = l.split_whitespace().collect_tuple().unwrap();
            (cmd, val.parse::<i32>().unwrap())
        })
        .collect_vec();
    if let Err(accumulator) = exe(&insts, -1) {
        dbg!(accumulator);
    }
    for (i, (cmd, _)) in insts.iter().enumerate() {
        if cmd == &"jmp" || cmd == &"nop" {
            if let Ok(accumulator) = exe(&insts, i as i32) {
                dbg!(accumulator);
            }
        }
    }
}
