use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[test]
fn test_generate_workout() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(value) = self.values.get(&arg) {
            return value.clone();
        }

        let value = (self.calculation)(arg);
        self.values.insert(arg, value);
        value
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Closures can capture values from their environment in three ways, which directly
// map to the three ways a function can take a parameter: taking ownership,
// borrowing mutably, and borrowing immutably.
// `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s
// environment. To consume the captured variables, the closure must take ownership of these
// variables and move them into the closure when it is defined. The Once part of the name
// represents the fact that the closure can’t take ownership of the same variables more than once,
// so it can be called only once.
// `FnMut` can change the environment because it mutably borrows values.
// `Fn` borrows values from the environment immutably.
#[test]
fn capturing_context() {
    let x = 4;

    let equal_to_x = move |z| z == x;
    println!("{:?}", x); // x implements copy so it is possible to use here

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn capturing_context_with_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); cant use here, because x is moved

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

#[test]
fn test_function_pointers() -> () {
    type FuncType = fn(i32) -> i32;

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: FuncType, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn use_fn() {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }

    use_fn();
}
