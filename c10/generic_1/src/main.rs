struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };

    println!("p1's x: {}", p1.x());
    // The following code causes compile error.
    // println!("distance from origin to p1: {}", p1.distance_from_origin());

    let p2 = Point { x: 5.0, y: 10.0 };

    println!("p2's x: {}", p2.x());
    println!("distance from origin to p2: {}", p2.distance_from_origin());

}
