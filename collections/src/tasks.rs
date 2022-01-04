use core::num;
use std::collections::HashMap;

fn get_mean(numbers: Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    return sum as f64 / numbers.len() as f64;
}

#[test]
fn test_get_mean() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result_mean = get_mean(numbers);
    assert_eq!(result_mean, 3 as f64);
}

fn get_median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid_index: usize = numbers.len() / 2;
    numbers[mid_index]
}

#[test]
fn test_get_median() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let median = get_median(&mut numbers);
    assert_eq!(median, 3);
}

fn get_most_common(numbers: &Vec<i32>) -> Option<i32> {
    let mut quantities = HashMap::new();
    for num in numbers {
        let mut val = quantities.entry(num).or_insert(0);
        *val += 1;
    }
    let max = quantities.into_iter().max_by(|x, y| x.1.cmp(&y.1));
    match max {
        Some(x) => Some(x.0.clone()),
        _ => None,
    }
}

#[test]
fn test_get_most_common() {
    let numbers = vec![1, 2, 3, 1, 4, 5, 1];
    let most_common = get_most_common(&numbers).unwrap();
    assert_eq!(most_common, 1)
}

fn get_as_pig_latin(value: &str) {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let words = value.split_whitespace();
    for word in words {
        if word.len() < 1 {
            continue;
        }
        let first_char = word.chars().nth(0).unwrap();
        let new_word = if vowels.contains(&first_char) {
            format!("{}-hay", word)
        } else {
            if word.len() > 1 {
                let left: String = word.chars().into_iter().take(1).collect();
                let right: String = word.chars().into_iter().skip(1).collect();
                format!("{}-{}ay", right, left)
            } else {
                word.to_string()
            }
        };
        println!("{}", new_word)
    }
}

#[test]
fn test_get_as_pig_latin() {
    get_as_pig_latin("Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company");
}
