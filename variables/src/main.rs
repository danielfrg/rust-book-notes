fn main() {
    // Mutables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of the constant is: {}", MAX_POINTS);

    // Shadowing

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // Shadow vs mutables
    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "   ";
    // We cant change the type with mutables
    spaces = spaces.len();
}
