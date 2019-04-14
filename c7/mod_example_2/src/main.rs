mod plant {
    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");
    println!("v is now {:?}.", v);

    v.name = String::from("butternut squash");
    println!("v is now {:?}.", v);
    println!("{} are delicious.", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
