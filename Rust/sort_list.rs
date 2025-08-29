use std::io;

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // Optimization: exit early if no swaps occurred
        }
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len - 1;
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[0..mid].to_vec();
    let right = arr[mid..].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn parse_numbers(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect()
}

fn main() {
    println!("Array Sorting Algorithms");
    println!("Enter numbers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    let numbers = match parse_numbers(input.trim()) {
        Ok(nums) => nums,
        Err(_) => {
            println!("Invalid input! Please enter valid numbers separated by spaces.");
            return;
        }
    };

    if numbers.is_empty() {
        println!("No numbers entered!");
        return;
    }

    println!("\nOriginal array: {:?}", numbers);

    // Test different sorting algorithms
    let mut bubble_sorted = numbers.clone();
    bubble_sort(&mut bubble_sorted);
    println!("Bubble Sort:    {:?}", bubble_sorted);

    let mut quick_sorted = numbers.clone();
    quick_sort(&mut quick_sorted);
    println!("Quick Sort:     {:?}", quick_sorted);

    let mut merge_sorted = numbers.clone();
    merge_sort(&mut merge_sorted);
    println!("Merge Sort:     {:?}", merge_sorted);

    let mut insertion_sorted = numbers.clone();
    insertion_sort(&mut insertion_sorted);
    println!("Insertion Sort: {:?}", insertion_sorted);

    let mut rust_sorted = numbers.clone();
    rust_sorted.sort();
    println!("Rust built-in:  {:?}", rust_sorted);

    // Verify all sorting methods produce the same result
    assert_eq!(bubble_sorted, quick_sorted);
    assert_eq!(quick_sorted, merge_sorted);
    assert_eq!(merge_sorted, insertion_sorted);
    assert_eq!(insertion_sorted, rust_sorted);
    
    println!("\n✓ All sorting algorithms produced the same result!");

    // Performance demo with larger array
    let mut large_array: Vec<i32> = (1..=1000).rev().collect();
    println!("\n--- Performance test with 1000 elements (reverse sorted) ---");
    
    let start = std::time::Instant::now();
    let mut test_array = large_array.clone();
    quick_sort(&mut test_array);
    let duration = start.elapsed();
    println!("Quick Sort: {:?}", duration);
    
    let start = std::time::Instant::now();
    let mut test_array = large_array.clone();
    merge_sort(&mut test_array);
    let duration = start.elapsed();
    println!("Merge Sort: {:?}", duration);
    
    let start = std::time::Instant::now();
    large_array.sort();
    let duration = start.elapsed();
    println!("Rust built-in: {:?}", duration);
}
