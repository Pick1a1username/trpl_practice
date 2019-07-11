#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            for element in &temp_vec {
                println!("{}", element);
            }

            temp_vec
        }
    };
}

fn main() {
    // By only defining a Vector, you can see each element in the Vector
    // printed to stdout.
    let _v: Vec<u32> = vec![1, 2, 3];
}
