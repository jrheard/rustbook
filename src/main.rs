// This is the first Rust I've ever written. Don't judge. I don't know any better yet.

use std::collections::HashMap;

use colored::*;

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
    println!("\n\n{}\n\n", "Implementations of suggested exercises from the Rust book.".white().bold());


    println!("{}", "Chapter 8:".green().bold());

    println!("\n{}", "Given a list of integers, use a vector and return the mean, median, and mode of the list.".green());

    let x = vec![1, 2, 3, 4, 5, 10, 200, 3, 2, 4];

    let (mean, median, mode) = stats(&x);

    println!("The mean, median, and mode of {:?} are {}, {}, and {}.", &x, mean, median, mode);

    println!("\n{}", "Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!".green());
    println!("{}", "UNIMPLEMENTED".red());

    println!("\n{}", "Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.".green());
    println!("{}", "UNIMPLEMENTED".red());
}
