fn digit_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

const N_ITERS: usize = 100;

fn main() {
    let start = std::time::Instant::now();
    let mut difftot = 0;
    for _ in (0..N_ITERS) {
        use rayon::prelude::*;
        let numbers: Vec<i32> = (0..1_000_000).into_par_iter().map(|_| fastrand::i32(1..=100_000)).collect();

        let mut min_number = i32::MAX;
        let mut max_number = i32::MIN;

        for &number in &numbers {
            if number < min_number || number > max_number {
                if digit_sum(number) == 30 {
                    if number < min_number {
                        min_number = number;
                    }
                    if number > max_number {
                        max_number = number;
                    }
                }
            }
        }

        if min_number == i32::MAX || max_number == i32::MIN {
            println!("No numbers with digit sum of 30 found.");
        } else {
            let difference = max_number - min_number;
            difftot += difference;
        }
    }
    let end = start.elapsed();
    println!("Time elapsed: {:?}", end / N_ITERS as u32);
}