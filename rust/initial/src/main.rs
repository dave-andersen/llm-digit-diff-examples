use rand::Rng;

fn digit_sum_original(n: i32) -> i32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).sum()
}

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
        let mut rng = rand::thread_rng();
        let numbers: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(1..=100_000)).collect();

        let mut min_number = i32::MAX;
        let mut max_number = i32::MIN;

        for &number in &numbers {
            if digit_sum(number) == 30 {
                if number < min_number {
                    min_number = number;
                }
                if number > max_number {
                    max_number = number;
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