use core::slice;

// Within the extern C -block we list names and signatures of extrnal functions
// From other languages we want to call
extern "C" {
    fn abs(input: i32) -> i32;
}

pub extern "C" fn call_from_c() {
    println!("This function is callable from C code when thi sis compiled to a shared library and linked from C")
}

unsafe trait UnsafeTrait {
    fn print();
}

// Static variables have a fixed address in memory
static HELLO_WORLD: & str = "Hello, world!"; 
// Accessing and modifying mutable static variables is unsafe
// since data races are possible
static mut COUNTER: u32 = 0;

fn main() {
    println!("Lets do some unsafe rust!");

    // Raw pointers
    unsafe {
        let mut num = 5;

        // We can have raw pointers of both immutable and mutabel at the same time
        let r1 = &num as *const i32; // Immutable raw pointer
        let r2 = &mut num as *mut i32; // Mutable raw pointer

        // Address is not necessarily bvalid
        let address = 0x012345usize;
        let r = address as *const i32;

        *r2 = 10; // We can edit the value but we as programmes must quarantee no panics
        
        println!("{} {}", *r1, *r2);
    }

    // Dangerous functions
    unsafe {
        dangerous_function();

        let mut v = vec![1, 2, 3, 4, 5];
        let slice = &mut v[..];

        let (a,b) = wrapper_unsafe_function(slice, 3);
        println!("{:?} {:?}", a, b);
    }

    // Calling out external code
    unsafe {
        println!("Absolute value of -500 according to C is {}", abs(-500));
    }

    println!("{}", HELLO_WORLD);

    unsafe {
        COUNTER += 5;
        println!("{}", COUNTER);
    }
}

unsafe fn dangerous_function() {
    println!("So spooky!");
}

fn wrapper_unsafe_function(slice: &mut[i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // Returns a raw pointer *mut i32

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // Takes a raw pointer and a lnegth and creates a slice
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}