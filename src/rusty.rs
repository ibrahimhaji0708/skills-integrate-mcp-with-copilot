// A complex Rust example using many Rust Book topics in under 20 lines
// Covers: generics, traits, lifetimes, enums, pattern matching, closures, iterators, error handling, modules, structs, impl, Option/Result, ownership, borrowing, references, slices, and concurrency (thread)

use std::thread;

trait Summable<T> { fn sum(&self) -> T; }
impl Summable<i32> for [i32] { fn sum(&self) -> i32 { self.iter().sum() } }

#[derive(Debug)]
enum Status { Ok(i32), Err(String) }

fn process<'a, F: Fn(i32) -> i32>(data: &'a [i32], f: F) -> Result<i32, String> {
    let v: Vec<i32> = data.iter().map(|&x| f(x)).collect();
    let s = v.as_slice().sum();
    if s > 0 { Ok(s) } else { Err("Sum non-positive".into()) }
}

pub fn run() {
    let data = [1, 2, 3, 4];
    let handle = thread::spawn(move || process(&data, |x| x * 2));
    match handle.join().unwrap() {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
