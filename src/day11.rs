use itertools::{iproduct, Itertools};

fn next(m: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut n = m.clone();
    for (i, j) in iproduct!(0..m.len(), 0..m[0].len()) {
        if m[i][j] == b'.' {
            continue;
        }
        let mut occupied = 0;
        for (&ai, &aj) in iproduct!(&[-1, 0, 1], &[-1, 0, 1]) {
            if ai == 0 && aj == 0 {
                continue;
            }
            let i = i as i32;
            let j = j as i32;
            if i + ai != -1
                && i + ai != m.len() as i32
                && j + aj != -1
                && j + aj != m[0].len() as i32
            {
                if m[(i + ai) as usize][(j + aj) as usize] == b'#' {
                    occupied += 1;
                }
            }
        }
        if m[i][j] == b'#' && occupied >= 4 {
            n[i][j] = b'L';
        }
        if m[i][j] == b'L' && occupied == 0 {
            n[i][j] = b'#';
        }
    }
    n
}

fn next2(m: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut n = m.clone();
    for (i, j) in iproduct!(0..m.len(), 0..m[0].len()) {
        if m[i][j] == b'.' {
            continue;
        }
        let mut occupied = 0;
        for (&ai, &aj) in iproduct!(&[-1, 0, 1], &[-1, 0, 1]) {
            if ai == 0 && aj == 0 {
                continue;
            }
            let mut i = i as i32;
            let mut j = j as i32;
            while i + ai != -1
                && i + ai != m.len() as i32
                && j + aj != -1
                && j + aj != m[0].len() as i32
            {
                i += ai;
                j += aj;
                match m[i as usize][j as usize] {
                    b'#' => {
                        occupied += 1;
                        break;
                    }
                    b'L' => break,
                    _ => {}
                }
            }
        }
        if m[i][j] == b'#' && occupied >= 5 {
            n[i][j] = b'L';
        }
        if m[i][j] == b'L' && occupied == 0 {
            n[i][j] = b'#';
        }
    }
    n
}

fn count(m: &Vec<Vec<u8>>) -> usize {
    m.iter()
        .map(|l| l.iter().filter(|&&b| b == b'#').count())
        .sum::<usize>()
}

pub fn run() {
    let txt = crate::common::get_input(11).unwrap();
    let m0 = txt.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let mut m = m0.clone();
    loop {
        let n = next(&m);
        if n == m {
            break;
        }
        m = n;
    }
    dbg!(count(&m));
    m = m0;
    loop {
        let n = next2(&m);
        if n == m {
            break;
        }
        m = n;
    }
    dbg!(count(&m));
}
