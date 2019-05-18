use counter::Counter as Counter;

fn main() {
    let mut not_skipped_counter = Counter::new();

    println!("not_skipped_counter.next: {}", not_skipped_counter.next().unwrap_or(0));
    println!("not_skipped_counter.next: {}", not_skipped_counter.next().unwrap_or(0));
    println!("not_skipped_counter.next: {}", not_skipped_counter.next().unwrap_or(0));
    println!("not_skipped_counter.next: {}", not_skipped_counter.next().unwrap_or(0));
    println!("not_skipped_counter.next: {}", not_skipped_counter.next().unwrap_or(0));

    let mut skipped_counter = Counter::new().skip(1);

    println!("skipped_counter.next: {}", skipped_counter.next().unwrap_or(0));
    println!("skipped_counter.next: {}", skipped_counter.next().unwrap_or(0));
    println!("skipped_counter.next: {}", skipped_counter.next().unwrap_or(0));
    println!("skipped_counter.next: {}", skipped_counter.next().unwrap_or(0));
    println!("skipped_counter.next: {}", skipped_counter.next().unwrap_or(0));
}

