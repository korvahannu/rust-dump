use iterators2;

fn main() {
    let sum : u32 = iterators2::Counter::new() // Create a new counter
        .zip(iterators2::Counter::new().skip(1)) // Zip the previous counter to a new counter
        .map(|(a,b)| a * b) // Map it back to a single iterator
        .filter(|x| x % 3 == 0) // Filter everything out that isnt divisible by 3
        .sum(); // Get the sum
    
    println!("{}", sum);
}
