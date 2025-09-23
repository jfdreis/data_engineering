use rayon::prelude::*;
use std::time::Instant;

fn main() {

    rayon::ThreadPoolBuilder::new().num_threads(20).build_global().unwrap();

    // Increase dataset size to 10 million
    let n = 10_000_000;
    let data: Vec<i64> = (1..=n).collect();

    // ------------------------------
    // Parallel sum with Rayon
    // ------------------------------
    println!("Starting parallel sum with Rayon...");
    let start_parallel = Instant::now();
    let parallel_sum: i64 = data
        .par_iter()          // Parallel iterator
        .map(|x| *x as i64)  // Cast to i64 to avoid overflow
        .sum();
    let duration_parallel = start_parallel.elapsed();

    println!("Parallel sum: {}", parallel_sum);
    println!("Parallel sum took: {:?}", duration_parallel);



    //wait 5 seconds
    println!("Waiting 5 seconds before sequential sum...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    // ------------------------------
    // Sequential sum
    // ------------------------------
    println!("Starting sequential sum...");
    let start_seq = Instant::now();
    let sequential_sum: i64 = data
        .iter()
        .map(|x| *x as i64)
        .sum();
    let duration_seq = start_seq.elapsed();

    println!("Sequential sum: {}", sequential_sum);
    println!("Sequential sum took: {:?}", duration_seq);

    // ------------------------------
    // Compare speedup
    // ------------------------------
    let speedup = duration_seq.as_secs_f64() / duration_parallel.as_secs_f64();
    println!("Speedup (seq / parallel): {:.2}x", speedup);
}
