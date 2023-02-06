use std::collections::HashMap;
fn main() {

    let _v: Vec<i32> = Vec::new(); //empty
    let _v1 = vec![1, 2, 3]; // pre-defined, immutable   

    let mut v2 = Vec::new(); // mutable
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // reading elements from vector
    let vc = vec![1, 2, 3, 4, 5];

    let third: &i32 = &vc[2]; //this syntax is not preferred
    println!("The third element is {third}"); 

    // let val: &i32 = &vc[100];
    // println!("The third element is {val}");

    let third: Option<&i32> = vc.get(2);
    match third {
        Some(num) => println!("The third element is {num}"),
        None => println!("There is no third element."),
    }

    // vector of enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let colors = vec![Color::Red, Color::Green, Color::Blue];

    println!("{:?}", colors);
    println!("{:?}", row);

    match &row[1] {
        SpreadsheetCell::Text(i) => println!("{}", i),
        _ => println!("Not an integer"),
    };

    // strings
    let _s = String::new(); // empty string

    let data = "initial contents"; // string literal
    let s = data.to_string(); // string literal -> string
    println!("{}",s);

    let mut s = String::from("foobar"); // user-defined
    s.push_str("123"); // appends string literal
    s.push('!'); // appends character
    println!("{}",s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // ownership of s1 is taken
    println!("{}",s3);

    let ss1 = String::from("Hello, ");
    let ss2 = String::from("world!");
    let s3 = format!("{}{}",ss1,ss2); // ownership not taken
    println!("{}",s3);

    let hello = "Здравствуйте"; //24 characters not 12
    let s = &hello[0..4];
    println!("{}",s);
    for c in hello.chars() {
        print!("{c},");
    }
    println!();
    for c in hello.bytes() {
        print!("{c},");
    }
    println!();

    // HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // insert value in Hashmap
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 40);
    scores.insert(String::from("Brown"), 20);
    scores.insert(String::from("Red"), 30);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get value from Hashmap
    println!("Score is {}",score);

    for (key, value) in &scores { //printing key-value pairs
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // this overwrites value of Blue

    scores.entry(String::from("Yellow")).or_insert(50); // or_insert checks whether there is a value or not if there is it does nothing else it adds the value
    scores.entry(String::from("Blue")).or_insert(50); // this will do nothing bcoz Blue already exists

    println!("{:?}", scores);

     // getting frequency of each word using hashmap
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); 
        *count += 1;
    }

    println!("{:?}", map);
}
