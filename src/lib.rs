use std::cmp::{max, min};

pub fn dp_best_cost(a: &[u32], b: &[u32], n: usize) -> u32 {
    let mut t: Vec<u32> = Vec::new();

    for (i, cost) in a.iter().enumerate().take(n) {
        if i == 0 {
            t.push(*cost);
            continue;
        }

        let mut max_prev_t = None;

        // Find the maximum amount of money we could make previously given enough rest days from i
        for j in 0..i {
            // If the current day is over the previous day + amount of rest days needed
            if i > j + b[j] as usize {
                if max_prev_t.is_none() {
                    max_prev_t = Some(t[j]);
                } else if let Some(max_prev_t_val) = max_prev_t {
                    if max_prev_t_val < t[j] {
                        max_prev_t = Some(t[j]);
                    }
                }
            }
        }

        if max_prev_t.is_none() {
            max_prev_t = Some(0);
        }

        t.push(*cost + max_prev_t.unwrap());
    }

    *t.iter().max().unwrap()
}

pub fn dj(s: &[u32], l: &[u32]) -> u32 {
    let mut s_indexed = s.iter().enumerate().collect::<Vec<_>>();
    s_indexed.sort_by(|x, y| x.1.partial_cmp(y.1).unwrap());

    let mut t: Vec<u32> = vec![1; l.len()];

    for i in 0..t.len() {
        if i == 0 {
            continue;
        }

        for j in 0..i {
            if s_indexed[i].1 > s_indexed[j].1 && l[s_indexed[i].0] > l[s_indexed[j].0] {
                t[i] = max(t[i], t[j] + 1)
            }
        }
    }

    *t.iter().max().unwrap()
}

pub fn most_keypresses(n: u32) -> u32 {
    // Possible clipboard values range from 0 to n, so we need to have n + 1 spaces
    // To prevent overflowing, leave some headroom
    let mut t: Vec<Vec<u32>> = vec![vec![u32::MAX - 100; (n + 1) as usize]; n as usize];

    for i in 0..n {
        if i == 0 {
            t[0][0] = 1; // Base case, typing 1 with 0 in the clipboard
            t[0][1] = 4; // Base case, typing 1 with 1 in the clipboard (a - Ctrl A - Ctrl C)
            continue;
        }

        for j in 0..=i + 1 {
            // From 1 to i
            // Adding 'a' with 1 keypress from previous row
            let add_a = t[(i - 1) as usize][j as usize] + 1;

            // Pasting - find i and j that sum up to the current value
            let mut paste_case = u32::MAX;
            for i_p in 0..i {
                if (i_p + 1) + j == (i + 1) {
                    paste_case = min(paste_case, t[i_p as usize][j as usize] + 2);
                }
            }

            // Copying
            let mut copy_case = u32::MAX;
            if j == i + 1 {
                for j_p in 0..=i + 1 {
                     copy_case = min(copy_case, t[i as usize][j_p as usize] + 3);
                }
            }

            t[i as usize][j as usize] = *[add_a, copy_case, paste_case].iter().min().unwrap();
        }
    }

    *t[(n - 1) as usize].iter().min().unwrap()
}
