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

type Name = String;
type Department = String;
enum Command {
    Add((Name, Department)),
    List,
    Exit,
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input);
    user_input
}

fn parse_add_employee_command(command: &str) -> Result<Command, String> {
    let words: Vec<&str> = command.split_whitespace().collect();
    if words.len() != 4 {
        println!("{:?}", words);
        return Err("add command has invalid syntax".to_string());
    }
    let employee_name: Name = words[1].to_string();
    let department: Department = words[3].to_string();
    Ok(Command::Add((employee_name, department)))
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut words = input.split_whitespace();
    let command = words.nth(0);
    match command {
        Some("Add") => parse_add_employee_command(input),
        Some("List") => Ok(Command::List),
        Some("Exit") => Ok(Command::Exit),
        _ => Err("Unknown command".to_string()),
    }
}

fn get_command() -> Result<Command, String> {
    let user_input = get_user_input();
    parse_command(&user_input)
}

fn department_employees() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("_________\nENTER COMMAND:\n_________");
        let command: Command = match get_command() {
            Ok(cmd) => cmd,
            Err(err) => {
                println!("ERROR PARSING COMMAND: {}", err);
                continue;
            }
        };
        match command {
            Command::Add((name, dept)) => {
                let dept_dictionary = departments.entry(dept.clone()).or_insert(Vec::new());
                dept_dictionary.push(name);
            }
            Command::List => {
                for (dept_name, dept_dictionary) in &departments {
                    println!("{}", dept_name);
                    for employee_name in dept_dictionary {
                        println!("...{}", employee_name);
                    }
                }
            }
            Command::Exit => break,
        }
    }
}

#[test]
fn test_department_employees() {
    department_employees()
}
