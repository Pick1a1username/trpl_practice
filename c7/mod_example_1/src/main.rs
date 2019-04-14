mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in();
            println!("Beeeeeeeeeeeep!!");
        }
    }

    fn breathe_in() {
        println!("Suuuuuuuuuuuuuuup!!");
    }
}

mod performance_group {
    use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

    println!("");
    println!("Please welcome, Clarinet Trio!!!!");
    performance_group::clarinet_trio();
}
