fn main() {
    let x = 4;
    let equal_to_x = | z | z == x;

    let y = 4;

    assert!(equal_to_x(y));

    let x = vec![1,2,3];
    // Note the move word. It means we consume x
    let equal_to_x = move |z| z == x;

    // println!("Can't use x here: {:?}", x);
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}
