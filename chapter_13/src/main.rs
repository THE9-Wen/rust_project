use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(25, 4);
    generate_workout(24, 4);
    generate_workout(24, 3);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match HashMap::get(&self.values, &arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_enclosure = Cacher::new(|num| {
        simulated_expensive_calculation(num)
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_enclosure.value(intensity));
        println!("Next, do {} situps!", expensive_enclosure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today, run for {} minutes!", expensive_enclosure.value(intensity));
        }
    }
}
