fn main() {
    // using mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 12;
    println!("The new value if x is: {x}");

    // using shadowing
    let y = 5;
    println!("The value of y is: {y}");
    let y= "six";
    println!("The new value of y is: {y}");

    // constant, must be annotated
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    // Data types
    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    let f = b"Hello"; // Byte string (&[u8])

    // Floating point
    let f = 2.0; // f64
    let g: f32 = 3.0; // f32

    // Addition
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false;

    // Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types
    let tup = ("Hello Rust!", 100_000);
    let (hello, _max_points) = tup;
    println!("The value of hello is: {hello}");
    println!("The value of max_points is: {}", tup.1);

    // Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    // create an array with 8 elements, all initialized to 0
    let byte = [0, 8];

    // Functions
    my_function();
    println!("The sum of 5 and 10 is: {}", custom_sum(5, 10));

    // Control flow
    let number = 5;

    if number < 10 {
        println!("First condition is true.");
    } else if number < 22 {
        println!("Second condition is true.");
    } else {
        println!("Neither condition is true.");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The value of result is: {result}");

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {element}");
    }

    for number in 1..4 {
        println!("{}", number);
    }
}

fn custom_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn my_function() {
    println!("Another function.");
}