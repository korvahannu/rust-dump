d #![allow(dead_code, unused_variables, unused_mut, unused_assignments)]

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn get(&self) -> &Point<T> {
        &self
    }
}

impl Point<String> {    // Only Point<String> can use this function
    fn print(&self) {
        println!("x:{} y:{}", self.x, self.y);
    }
}

enum Options<T, U> {
    Test(T),
    Production(U)
}

impl<T, U> Options<T, U> {
    fn mix<T2, U2>(&self, p : Options<T2, U2>) {
        // This is an example of generic method
    }
}

fn main() {
    let integer = Point { x: 5, y: 5 };
    let string = Point { x : String::from("hey"), y: String::from("hey")};

    string.print();

    let mut option : Options<String, u32> = Options::Test(String::from("Test"));
    option = Options::Production(5);

    match option {
        Options::Test(st) => {
            println!("{}", st);
        },
        Options::Production(nm) => {
            println!("{}", nm * 5)
        }
    };
}

// The following returns an error because
// the things applied to &item do not apply for all types
// for example, if list would be made of strings
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
