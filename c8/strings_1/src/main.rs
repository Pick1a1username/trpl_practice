fn main() {
    //
    // English
    //

    let s1 = String::from("hello");
    // The following code doesn't work.
    // let s1_first = &s1[0];
    let s1_first = &s1[0..1];
    println!("The first character of s1 is {}.", s1_first);

    println!("Printing characters of s1...");
    for c in s1.chars() {
        println!("{}", c);
    }

    println!("Printing bytes of s1...");
    for b in s1.bytes() {
        println!("{}", b);
    }

    println!("");
    println!("");

    //
    // Korean
    //

    let s2 = String::from("안녕하세요");
    // The following code doesn't work(will be compiled, but panicked in runtime).
    // let s2_first = &s2[0..1];
    let s2_first = &s2[0..3];
    println!("The first character of s2 is {}.", s2_first);

    println!("Printing characters of s2...");
    for c in s2.chars() {
        println!("{}", c);
    }

    println!("Printing bytes of s2...");
    for b in s2.bytes() {
        println!("{}", b);
    }

}
