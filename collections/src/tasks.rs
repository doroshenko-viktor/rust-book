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
