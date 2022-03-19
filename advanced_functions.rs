#[derive(Debug)]
enum Status {
    Value(u32),
    Stop
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("{}", answer);

    let list_of_statuses : Vec<Status>
        = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses);

    println!("{}", returns_closure()(5));
}

fn add_one(x:i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32  {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(
        |x| x + 1
    )
}