fn main() {
    let s = String::from("hello");  // s coms into scope

    takes_ownership(s);             // s's value moves into the function...

    println!("{}", s);
                                    // ... and so is no longer valid here
    
    // The following code make compilation error.
    // let x = 5;

    makes_copy(x);



}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
