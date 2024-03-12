fn main() {
    // ------ Ownership rules ------
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {   // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid

    // ------ Memory and Allocation ------
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move

    // println!("{}, world!", s1); // Error: value borrowed here after move

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // ------ Ownership and Functions ------
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s); // Error: value borrowed here after move

    // to fix this, we can return the value from the function
    let s = String::from("hello");
    let s = takes_ownership_and_return(s);

    // ------ References and Borrowing ------
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // ------ Mutable References ------
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time

    println!("{}", r1);
}

fn calculate_length(s : &String) -> usize {
    let length = s.len();
    length
}

fn takes_ownership_and_return(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
