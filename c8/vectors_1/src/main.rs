fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(4) {
        Some(third) => println!("The fifth element is {}", third),
        None => println!("There is no fifth element."),
    }
}
