mod menu {
    pub enum Appetizer {
        Soup(u32),
        Salad(u32),
    }

    pub fn sum(order: Appetizer) -> u32 {
        match order {
            Appetizer::Soup(amount) => amount * 2,
            Appetizer::Salad(amount) => amount * 5,
        }
    }
}

fn main() {
    let order1 = menu::Appetizer::Soup(50);
    let order2 = menu::Appetizer::Salad(20);
    
    println!("The first order's billing: {} dollar(s)", menu::sum(order1));
    println!("The second order's billing: {} dollar(s)", menu::sum(order2));
}
