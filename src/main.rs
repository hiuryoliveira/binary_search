use std::time::Instant;

fn main() {
    let items: Vec<i32> = (1..=10000000).collect();

    let target = 809889;
    match binary_search(&items, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found", target),
    }


    // Benchmark
    let start = Instant::now();
    let iterations = 1000000;
    for _ in 0..iterations {        
        binary_search(&items, target);
    }
    let duration = start.elapsed();
    println!("Benchmark duration: {:?}", duration);
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