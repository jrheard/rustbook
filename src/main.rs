// This is the first Rust I've ever written. Don't judge. I don't know any better yet.

use std::collections::HashMap;

use assert_approx_eq::assert_approx_eq;

fn mean(nums: &Vec<i32>) -> f64 {
    if nums.len() == 0 {
        return 0.0;
    }

    let mut mean = 0.0;

    for num in nums {
        mean += *num as f64;
    }

    mean /= nums.len() as f64;

    mean
}

fn median(nums: &Vec<i32>) -> f64 {
    if nums.len() == 0 {
        return 0.0;
    }

    let mut nums = nums.to_vec();

    nums.sort();

    if nums.len() % 2 == 0 {
        let before_midpoint = nums[(nums.len() / 2) - 1] as f64;
        let after_midpoint = nums[nums.len() / 2] as f64;

        (before_midpoint + after_midpoint) / 2.0
    } else {
        nums[nums.len() / 2] as f64
    }
}

fn mode(nums: &Vec<i32>) -> i32 {
    // If there are multiple modes, I just pick one arbitrarily. In reality, it looks like
    // it's possible for a set of numbers to have zero, one, or several modes.
    let mut occurrences = HashMap::new();

    for num in nums {
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

    mode
}

// Documentation comments aren't covered until chapter 14 and I'm on chapter 8
// right now, so I'm just winging it on comments atm.

// Prompt:
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end
// instead (“apple” becomes “apple-hay”).
// (Intentionally not handling punctuation.)
fn pig_latinize_word(word: &str) -> String {
    // (This code taken wholesale from Peter Huene, because I was having a lot of trouble
    // figuring out a sane way of expressing it because I'm still learning. Thanks, Peter!
    let first = word.chars().nth(0).unwrap();
    match first.to_lowercase().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", &word[1..], first),
    }
}

fn pig_latin(a_str: &str) -> String {
    // I don't know what a Rust closure is just yet, but I really wanted to use the map operation
    // here so I'm using it before I've gotten to that part of the book. This is fine.
    let latinized_words: Vec<String> = a_str
        .split(" ")
        .map(|word| pig_latinize_word(word))
        .collect();
    latinized_words.join(" ")
}

// TODO lifetimes

// Pretty sure &strs would be more natural than Strings, but I haven't learned about lifetimes yet.
fn add_employee(directory: &mut HashMap<String, Vec<String>>, command: &str) {
    // Parse the command.
    let words: Vec<&str> = command.split(" ").collect();
    let employee = words[1];
    let department = words[3];

    let department_entry = directory
        .entry(department.to_string())
        .or_insert(Vec::new() as Vec<String>);
    department_entry.push(employee.to_string());
}

fn sort_employees(directory: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut keys: Vec<&String> = directory.keys().collect();
    keys.sort();

    let mut result: Vec<String> = Vec::new();

    for department in &keys {
        let mut employees = directory.get(*department).unwrap().clone();
        employees.sort();
        result.push(format!("{} has employees {:?}", department, employees));
    }

    result
}

#[cfg(test)]
mod chapter_8 {
    use super::*;

    #[test]
    fn test_mean() {
        assert_approx_eq!(mean(&vec![10, 2, 38, 23, 38, 23, 21]), 22.142857);
        assert_eq!(mean(&vec![4, 8, 15, 16, 23, 42]), 18.0);
        assert_eq!(mean(&vec![5]), 5.0);
        assert_eq!(mean(&vec![]), 0.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&vec![1, 2, 3, 4, 5, 10, 200, 3, 2, 4]), 3.5);
        assert_eq!(median(&vec![10, 2, 38, 23, 38, 23, 21]), 23.0);
        assert_eq!(median(&vec![5]), 5.0);
        assert_eq!(median(&vec![]), 0.0);
    }

    #[test]
    fn test_mode() {
        assert!([2, 3, 4].contains(&mode(&vec![1, 2, 3, 4, 5, 10, 200, 3, 2, 4])));
        assert_eq!(mode(&vec![23, 10, 2, 38, 23, 38, 23, 21]), 23);
        assert_eq!(mode(&vec![5]), 5);
        assert_eq!(mode(&vec![]), 0);
    }

    #[test]
    fn test_pig_latin() {
        assert_eq!(
            pig_latin("Hello there how are you I am fine"),
            "ello-Hay here-tay ow-hay are-hay ou-yay I-hay am-hay ine-fay"
        );
    }

    #[test]
    fn test_employee_directory() {
        let mut directory = HashMap::new();
        add_employee(&mut directory, "Add Sally to Engineering");
        add_employee(&mut directory, "Add Amir to Sales");
        add_employee(&mut directory, "Add Bob to Engineering");
        add_employee(&mut directory, "Add Cathy to Engineering");
        add_employee(&mut directory, "Add Joseph to Sales");
        add_employee(&mut directory, "Add Angela to Finance");

        let result = sort_employees(&directory);

        assert_eq!(
            result,
            vec![
                "Engineering has employees [\"Bob\", \"Cathy\", \"Sally\"]",
                "Finance has employees [\"Angela\"]",
                "Sales has employees [\"Amir\", \"Joseph\"]"
            ]
        )
    }

}
