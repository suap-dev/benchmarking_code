use std::time::Instant;
use rand::Rng;
// use std::time::SystemTime;

const ITERATIONS: u128 = 10_000_000;
const NUMBER_OF_TESTS: u128 = 10;
const RANGE: std::ops::Range<u128> = 1..1000;



fn main() {
    rand::thread_rng();
    let mut avg_time: u128 = 0;

    println!("Calculating a sum of {ITERATIONS} random numbers ranging from {} to {} to check run times.",
        RANGE.start, RANGE.end
    );
    println!();

    for i in 0..NUMBER_OF_TESTS {
        let start = Instant::now();
        let sum = long_loop();

        let elapsed_time = start.elapsed();
        let elapsed_micros = elapsed_time.as_micros();
        let elapsed_secs = elapsed_time.as_secs();
        print!(
            "sum#{}={}, in {} mircoseconds",
            i+1, sum, elapsed_micros
        );
        if elapsed_secs > 0 {
            print!(", or {} seconds", elapsed_secs);
        }
        println!();

        avg_time += elapsed_micros;
    }
    println!();

    let avg_time = avg_time / NUMBER_OF_TESTS;
    println!("Average time: {avg_time} microseconds\n");

}

fn long_loop() -> u128 {
    let mut sum: u128 = 0;
    for _i in 0..ITERATIONS{
        sum += rand::thread_rng().gen_range(RANGE);
    }
    sum
}