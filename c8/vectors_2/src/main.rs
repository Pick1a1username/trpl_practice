fn main() {
    let mut v = vec![100, 32, 57];

    println!("Step 1");

    for i in &mut v {
        println!("{}", i);
    }

    println!("");


    println!("Step 2");

    for i in &mut v {
        *i += 50;
    }

    println!("");

    println!("Step 3");

    for i in &mut v {
        println!("{}", i);
    }

    println!("");
}
