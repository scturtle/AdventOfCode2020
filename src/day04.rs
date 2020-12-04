use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let txt = crate::common::get_input(4).unwrap();
    let mut passports = vec![];
    for (is_empty, lines) in &txt.lines().group_by(|l| l.is_empty()) {
        if !is_empty {
            passports.push(lines.collect_vec().join(" "));
        }
    }
    let mut valid0 = 0;
    let mut valid1 = 0;
    for passport in &passports {
        let fields = passport
            .split_ascii_whitespace()
            .map(|f| f.split(':').collect_tuple::<(_, _)>().unwrap())
            .collect::<HashMap<_, _>>();
        let mut ok0 = 0;
        let mut ok1 = 0;
        // check byr
        if let Some(val) = fields.get("byr") {
            ok0 += 1;
            if val.len() == 4 {
                if let Ok(val) = val.parse::<u32>() {
                    if (1920..=2002).contains(&val) {
                        ok1 += 1;
                    }
                }
            }
        }
        // check iyr
        if let Some(val) = fields.get("iyr") {
            ok0 += 1;
            if val.len() == 4 {
                if let Ok(val) = val.parse::<u32>() {
                    if (2010..=2020).contains(&val) {
                        ok1 += 1;
                    }
                }
            }
        }
        // check eyr
        if let Some(val) = fields.get("eyr") {
            ok0 += 1;
            if val.len() == 4 {
                if let Ok(val) = val.parse::<u32>() {
                    if (2020..=2030).contains(&val) {
                        ok1 += 1;
                    }
                }
            }
        }
        // check hgt
        if let Some(val) = fields.get("hgt") {
            ok0 += 1;
            if let Some(val) = val.strip_suffix("cm") {
                if let Ok(val) = val.parse::<u32>() {
                    if (150..=193).contains(&val) {
                        ok1 += 1;
                    }
                }
            } else if let Some(val) = val.strip_suffix("in") {
                if let Ok(val) = val.parse::<u32>() {
                    if (59..=76).contains(&val) {
                        ok1 += 1;
                    }
                }
            }
        }
        // check hcl
        if let Some(val) = fields.get("hcl") {
            ok0 += 1;
            if val.starts_with('#')
                && val.len() == 7
                && val[1..]
                    .chars()
                    .all(|c| c == c.to_ascii_lowercase() && c.is_digit(16))
            {
                ok1 += 1;
            }
        }
        // check ecl
        if let Some(val) = fields.get("ecl") {
            ok0 += 1;
            if "amb blu brn gry grn hzl oth"
                .split_ascii_whitespace()
                .any(|cl| cl == *val)
            {
                ok1 += 1;
            }
        }
        // check pid
        if let Some(val) = fields.get("pid") {
            ok0 += 1;
            if val.len() == 9 && val.chars().all(|c| c.is_digit(10)) {
                ok1 += 1;
            }
        }

        valid0 += (ok0 == 7) as i32;
        valid1 += (ok1 == 7) as i32;
    }
    dbg!(valid0);
    dbg!(valid1);
}
