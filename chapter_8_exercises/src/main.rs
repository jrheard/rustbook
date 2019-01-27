// This is the first Rust I've ever written. Don't judge. I don't know any better yet.

use std::collections::HashMap;

// Returns the mean, median, and mode of `nums`.
fn stats(nums: &Vec<i32>) -> (f64, f64, f64){

    // XXX - is this shadowing+copying technique idiomatic or terrible?
    let mut nums = nums.to_vec();

    // Mean.
    let mut mean = 0.0;

    for num in &nums {
        mean += *num as f64;
    }

    mean /= nums.len() as f64;

    // Median.
    let median;

    nums.sort();

    if nums.len() % 2 == 0 {
        let before_midpoint = nums[(nums.len() / 2) - 1] as f64;
        let after_midpoint = nums[nums.len() / 2] as f64;

        median = (before_midpoint + after_midpoint) / 2.0;
    } else {
        median = nums[nums.len() / 2] as f64;
    }

    // Mode.
    // If there are multiple modes, I just pick one arbitrarily. In reality, it looks like
    // it's possible for a set of numbers to have zero, one, or several modes.
    let mut occurrences = HashMap::new();

    for num in &nums {
        let count = occurrences.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;

    for (number, count) in &occurrences {
        if *count > max_count {
            mode = *number;
            max_count = *count;
        }
    }

    // All done.
    (mean, median, mode.into())
}

fn main() {
    println!("Hello, world!");

    let x = vec![1, 2, 3, 4, 5, 10, 200, 3, 2, 4];

    let (mean, median, mode) = stats(&x);

    println!("the mean, median, and mode of {:?} are {}, {}, and {}", &x, mean, median, mode)
}
