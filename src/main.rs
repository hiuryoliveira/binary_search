use std::time::Instant;
use rand::Rng;
use std::thread;

fn main() {
    let items: Vec<i32> = (1..=10000000).collect();
    let target = 809889;
    match binary_search(&items, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }

    // Benchmark
    let start = Instant::now();
    let iterations = 10000000;
    for _ in 0..iterations {        
        binary_search(&items, random_value());
    }
    let duration = start.elapsed();
    println!("Benchmark duration without threads: {:?}", duration);

    // Benchmark with threads
    let start = Instant::now();
    let iterations = 10000000;
    let handle = thread::spawn(move || {
        for _ in 0..iterations {        
            binary_search(&items, random_value());
        }
    });
    handle.join().unwrap();
    let duration = start.elapsed();
    println!("Benchmark duration with threads: {:?}", duration);
}

fn random_value() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=10000000);
}

fn binary_search(items: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = items.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if items[mid] == target {
            return Some(mid);
        } 
        
        if items[mid] < target {
            left = mid + 1;
        } 
        
        if items[mid] > target {
            right = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_value_within_range() {
        let value = random_value();
        assert!(value >= 1 && value <= 10000000);
    }

    #[test]
    fn test_binary_search_found() {
        let items: Vec<i32> = (1..=100).collect();
        let target = 4;
        assert_eq!(binary_search(&items, target), Some(3));
    }

    #[test]
    fn test_binary_search_not_found() {
        let items: Vec<i32> = (1..=100).collect();
        let target = 101;
        assert_eq!(binary_search(&items, target), None);
    }
}