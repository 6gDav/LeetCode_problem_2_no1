mod solution;
use memory_stats::memory_stats;
use solution::Solution as sol;
use std::time::Instant;

fn main() {
    println!("=== Benchmarking Two Sum Solution ===");

    // --- Small Input Test ---
    let num1 = vec![2, 4, 3];
    let num2 = vec![5, 6, 4];

    let usage_before = memory_stats().unwrap();
    let start_time = Instant::now();
    sol::add_two_numbers(num1, num2);
    let duration = start_time.elapsed();
    let usage_after = memory_stats().unwrap();

    let mem_used = usage_before.physical_mem.saturating_sub(usage_after.physical_mem);

    println!("Execution Time: {:?}", duration);
    println!("Memory Delta:   {} bytes", mem_used);
    println!("Current Memory: {} bytes", usage_after.physical_mem);

    // --- Stress Test (Large Input) ---
    println!("\n[Stress Test - 1,000 elements]");
    let num1: Vec<i32> = (0..1_000).collect();
    let num2: Vec<i32> = (0..1_000).collect();

    let _ = num1.len();
    let _ = num2.len();

    let usage_before_stress = memory_stats().unwrap();
    let start_time_stress = Instant::now();

    sol::add_two_numbers(num1, num2);

    let duration_stress = start_time_stress.elapsed();
    let usage_after_stress = memory_stats().unwrap();

    let mem_used_stress = usage_after_stress.physical_mem.saturating_sub(usage_before_stress.physical_mem);

    println!("Execution Time: {:?}", duration_stress);
    println!("Memory Delta:   {} bytes", mem_used_stress);
    println!("Current Memory: {} bytes", usage_after_stress.physical_mem);
}
