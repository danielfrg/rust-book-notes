fn main() {
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);

    println!("{:?}", v);

    // Get an element
    match v.get(2) {
        Some(third) => println!("3rd element is {}", third),
        None => println!("No 3rd element")
    }

    match v.get(100) {
        Some(e) => println!("100th element is {}", e),
        None => println!("No 100th element")
    }

    // Iterating
    for i in &v {
        println!("{}", i);
    }

    // Iterating with mutable
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }

    // Vectors of multiple types using enums

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.2),
        SpreadsheetCell::Text(String::from("azul")),
    ];

    println!("{:?}", row);
}
