fn main() {
    let _penny = Coin::Penny;
    let _nickel = Coin::Nickel;
    let _dime = Coin::Dime;
    let _quarter_alaska = Coin::Quarter(UsState::Alaska);
    let _quarter_california = Coin::Quarter(UsState::California);

    println!("Penny is equal to {}.", value_in_cents(_penny));
    println!("Nickel is equal to {}.", value_in_cents(_nickel));
    println!("Dime is equal to {}.", value_in_cents(_dime));
    println!("Quarter is equal to {}.", value_in_cents(_quarter_alaska));
    println!("Quarter is equal to {}.", value_in_cents(_quarter_california));


}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    New_Hampshire,
    New_Jersey,
    New_Mexico,
    New_York,
    North_Carolina,
    North_Dakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    Rhode_Island,
    South_Carolina,
    South_Dakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    West_Virginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
