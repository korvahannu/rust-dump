use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Fn traits are provided by standard library
// All closures implement either Fn, FnMut, or FnOnce

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32> // Some(u32) or None
}

impl<T: Fn(u32)->u32> Cacher<T> {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {

    // We save the definition of an anonymous function inside expensive_closure
    // Type annotation for num and return value are optional:
    // let expensive_closure = |num : u32| -> u32 {
    /*let expensive_closure = |num| {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    };*/

    // By using cacher we only do the expensive calculation once
    let mut expensive_closure = Cacher::new(|num|  {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
}

// Optional way of doing a more modular cacher
struct AdvancedCacher<B, T: Fn(B) -> B> {
    calculation: T,
    value: HashMap<Option<B>, B>
}

impl<B, T: Fn(B) -> B> AdvancedCacher<B, T> {
    fn new(calculation: T) -> AdvancedCacher<B, T> {
        AdvancedCacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, value:B) -> B {
        // En jaksanut tehdä loppuun :--D
        value
    }
}