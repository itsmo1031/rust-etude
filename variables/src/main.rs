use std::io;

fn main() {
    re_define();
    float();
    calc();
    bool();
    char();
    tuple();
    array();
    array_out_of_index();
}

fn re_define() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    println!();
}

fn float() {
    let x = 2.0; // f64
    let y: f32 = 3.0;

    println!("{x}");
    println!("{y}");

    println!();
}

fn calc() {
    // addition
    let sum = 5 + 10;
    println!("Sum is: {sum}");

    // subtraction
    let diff = 95.5 - 4.3;
    println!("Diff is: {diff}");

    // multiplication
    let prod = 4 * 30;
    println!("Prod is: {prod}");

    // division
    let quotient = 56.7 / 32.2;
    println!("Div is: {quotient}");
    let truncated = -5 / 3;
    println!("Div(truncated) is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder is: {remainder}");

    println!();
}

fn bool() {
    let t = true;
    println!("bool true: {t}");

    let f: bool = false; // 타입 명시
    println!("bool false: {f}");

    println!();
}

fn char() {
    let c = 'z';
    println!("char: {c}");
    let z: char = 'ℤ';
    println!("char: {z}");
    let heart_eyed_cat = '😻';
    println!("char: {heart_eyed_cat}");

    println!();
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destructing

    println!("x: {x}, y: {y}, z: {z}");
    println!("tuple: {:?}", tup);

    let five_hundred = tup.0;
    println!("use index: {five_hundred}");

    println!();
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    println!("array: {:?}", a);

    let first = a[0];
    println!("array[0]: {first}");

    let a = [3; 5]; // 3이 5개
    println!("array: {:?}", a);

    println!();
}

fn array_out_of_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; // out of index error may occur.

    println!("The value of the element at index {index} is: {element}");
}
