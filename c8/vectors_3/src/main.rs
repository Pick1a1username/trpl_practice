fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => println!("This is integer: {}", int),
            SpreadsheetCell::Text(text) => println!("This is text: {}", text),
            SpreadsheetCell::Float(float) => println!("This is float: {}", float),
        }
    }
}
