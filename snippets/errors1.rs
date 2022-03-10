#![allow(dead_code, unused_variables, unused_mut)]

/* add the following to Cargo.toml to change the panic!
macro to abort immidietly instead of rewiding back
Rewinding means Rust cleans up the leftover data
Disabling rewind means smaller binary file

[profile.release]
panic = 'abort'
*/

fn main() {
    // Unrecoverable errors
    // panic!("Crash and burn");

    let v : Vec<u8> = vec![1,2,3];
    v[99];  // Results in a panic
    // You can backtrace the exact reason with RUST_BACKTRACE=1 cargo run
}
