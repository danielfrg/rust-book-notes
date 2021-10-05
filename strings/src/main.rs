fn main() {
    // Creating a string
    let mut s = String::new();
    s = "initial content".to_string();
    s = String::from("initial contents");
    println!("{}", s);

    s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Indexing
    // Regular indexing doesn't work
    // let h = s1[0];

    // We can slice using the number of bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // If we index into a char it doesn't work
    // let s = &hello[0..3];

    // Iterating
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
