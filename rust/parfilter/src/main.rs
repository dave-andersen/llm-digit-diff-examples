use std::cmp::{max, min};
use std::time::Instant;
use rayon::prelude::*;

const N_ITERS: usize = 2000;

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn digitdiff() -> i32 {

    let mut min_num = i32::MAX;
    let mut max_num = i32::MIN;

    const PREGEN: usize = 10_000;
    for _ in 0..PREGEN {
        let num = fastrand::i32(1..100000);
        if num < min_num || num > max_num {
            if digit_sum(num) == 30 {
                min_num = min(min_num, num);
                max_num = max(max_num, num);
            }
        }
    }
    let numbers: Vec<i32> = (0..(1_000_000 - PREGEN))
        .into_par_iter()
        .map(|_| fastrand::i32(1..100000))
        .filter(|&x| (x < min_num || x > max_num) && digit_sum(x) == 30)
        .collect();

    for num in numbers {
        if num < min_num || num > max_num {
            min_num = min(min_num, num);
            max_num = max(max_num, num);
        }
    }

    if min_num == i32::MAX || max_num == i32::MIN {
        0
    } else {
        max_num - min_num
    }
}

fn main() {
    let mut diff = 0;
    let start = Instant::now();

    for _ in 0..N_ITERS {
        diff += digitdiff();
    }
    let duration = start.elapsed();

    println!("Execution time: {:?}", duration / N_ITERS as u32);
    // This is only here to prevent optimizing away our loop by accident
    println!("Average difference: {}", diff as f64 / N_ITERS as f64);
}